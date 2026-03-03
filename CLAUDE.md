# CLAUDE.md - spl-lint

## プロジェクト概要

Splunk の検索言語 SPL (Search Processing Language) に対する静的解析ツール (linter) です。
クエリ文字列を受け取り、構文エラーやベストプラクティス違反を検出・報告します。

- 言語: Rust (edition 2024)
- 対象: Splunk Search Processing Language (SPL, SPL2)

## ビルド・テスト

```bash
cargo build          # ビルド
cargo test           # 全テスト実行
cargo run -- FILE    # lint 実行
```

## CLI 使い方

```bash
spl-lint <file>...               # ファイルを lint する
echo 'query' | spl-lint          # 標準入力から lint する
spl-lint --format json <file>    # JSON 形式で出力する
spl-lint --disable W002 <file>   # ルールを無効化する
spl-lint --list-rules            # ルール一覧を表示する
spl-lint --list-commands         # SPL コマンド一覧を表示する
spl-lint --list-functions        # eval 関数一覧を表示する
spl-lint --verbose <file>        # lint 成功時にメッセージを表示する (-v でも可)
spl-lint --trim <file>           # コメントを除去して整形済みクエリを出力する
spl-lint --trim --write <file>   # コメントを除去してファイルに書き戻す
spl-lint --spl2 <file>          # SPL2 モードで lint する
spl-lint --spl2 --list-rules    # SPL2 ルール一覧を表示する
spl-lint --spl2 --list-commands # SPL2 コマンド一覧を表示する
spl-lint --spl2 --list-functions # SPL2 eval 関数一覧を表示する
```

- 終了コード: エラー/警告あり=1, なし=0
- テキスト出力は miette によるソース位置付き表示です

## アーキテクチャ

3 層構造で実装しています。

```
CLI (main.rs)    -- clap + miette による入出力
Linter           -- Rule トレイト + LintEngine による AST 走査
Parser           -- 手書き再帰下降パーサー (Lexer -> Token -> AST)
```

処理の流れ: `ソース文字列 -> Lexer -> Vec<Token> -> Parser -> AST (Query) -> LintEngine -> Vec<Diagnostic>`

## ファイル構成

```
src/
├── main.rs                              # CLI エントリポイント (clap, miette)
├── lib.rs                               # ライブラリルート
├── diagnostic.rs                        # Diagnostic, Severity 型
├── lexer/
│   ├── mod.rs                           # Lexer 実装
│   └── token.rs                         # Token, TokenKind, Span 型
├── parser/
│   ├── mod.rs                           # Parser 実装
│   └── ast.rs                           # AST ノード定義
└── linter/
    ├── mod.rs                           # LintEngine
    ├── rule.rs                          # Rule トレイト定義
    ├── known_commands.rs                # SPL コマンド一覧 (155+)
    ├── known_eval_functions.rs          # eval 関数一覧 (101+)
    ├── known_stats_functions.rs         # stats 関数一覧 (34+)
    └── rules/
        ├── mod.rs                       # ルールモジュール一覧
        ├── syntax_error.rs              # E001
        ├── wildcard_only_search.rs      # W001
        ├── unknown_command.rs           # W002
        ├── missing_args.rs              # W003
        ├── pipe_style.rs                # W004
        ├── ambiguous_precedence.rs      # W005
        ├── filter_after_aggregation.rs  # W006
        └── unknown_eval_function.rs     # W007
└── spl2/
    ├── mod.rs                           # SPL2 モジュールルート
    ├── lexer/
    │   ├── mod.rs                       # SPL2 Lexer
    │   └── token.rs                     # SPL2 TokenKind
    ├── parser/
    │   ├── mod.rs                       # SPL2 Parser
    │   └── ast.rs                       # SPL2 AST
    └── linter/
        ├── mod.rs                       # Spl2LintEngine
        ├── rule.rs                      # Spl2Rule トレイト定義
        ├── known_commands.rs            # SPL2 コマンド一覧 (50)
        ├── known_eval_functions.rs      # SPL2 eval 関数一覧 (130+)
        ├── known_stats_functions.rs     # SPL2 stats 関数一覧 (35+)
        └── rules/
            ├── mod.rs                   # ルールモジュール一覧
            ├── syntax_error.rs          # S001
            ├── unknown_command.rs       # S002
            ├── unknown_function.rs      # S003
            ├── deprecated_dot_concat.rs # S004
            ├── unquoted_special_field.rs # S005
            └── reserved_word_field.rs   # S006
testdata/
├── spl/                                 # SPL1 テスト用 .spl ファイル
│   └── examples/                        # SPL1 プロダクションクエリ
├── spl2/                                # SPL2 テスト用 .spl2 ファイル
└── spl2-sql/                            # SPL2 SQL 構文テスト用 .spl2 ファイル
```

## SPL 構文仕様 (実装済み)

### Lexer が認識するトークン

| カテゴリ | トークン |
|---|---|
| リテラル | 整数, 浮動小数点, 文字列 (`"..."`), ワイルドカード |
| 識別子 | 通常 (`field`), ドット付き (`src_ip.country`) |
| キーワード | `AND`, `OR`, `NOT` (大文字のみ), `as`, `by`, `true`, `false` (大文字小文字不問) |
| 演算子 | `=`, `!=`, `<`, `<=`, `>`, `>=`, `\|`, `!`, `+`, `-`, `*`, `/`, `%` |
| 区切り記号 | `()`, `[]`, `,`, `.`, `:` |
| ワイルドカード | `*foo*`, `error*`, `*` |
| コメント | バッククォート (`` `comment` ``) |

### Parser が生成する AST

- `Query` -- パイプラインステージのリスト
- `PipelineStage` -- `StageKind` (Search / Command)
- `SearchExpr` -- FreeText, FieldFilter, And, Or, Not, Grouped, Wildcard, SubSearch
- `Command` -- コマンド名 + 引数リスト (位置引数, 名前付き引数, by 節, as 節)
- `Expr` -- Number, String, Bool, Field, Wildcard, FunctionCall, BinaryOp, UnaryOp, SubSearch
- `FunctionCall` -- 関数名 + 引数リスト

### SPL 固有の注意点

- **AND は OR より結合が強い**: `a OR b AND c` は `a OR (b AND c)` と解釈されます
- **暗黙の AND**: `src=a dst=b` は `src=a AND dst=b` と同等です
- **キーワードの大文字小文字**: `AND`, `OR`, `NOT` は大文字のみがキーワードです
- **`as`, `by` は大文字小文字不問**: `AS`, `By`, `by` はすべてキーワードです
- **サブサーチ**: `[search ...]` のように `[]` 内にパイプラインを記述します
- **バッククォートコメント**: `` `this is a comment` `` 形式でコメントを記述します

## SPL1 Lint ルール一覧

| ID | 重大度 | カテゴリ | 説明 |
|---|---|---|---|
| `E001` | error | syntax | 構文エラー (パース失敗) |
| `W001` | warning | performance | `*` のみの search (全件マッチ) |
| `W002` | warning | correctness | 未知のコマンド名 |
| `W003` | warning | correctness | コマンドの必須引数が不足 |
| `W004` | info | style | パイプ `\|` の前後に空白がない |
| `W005` | warning | correctness | AND/OR 優先順位が曖昧 (括弧なし混在) |
| `W006` | info | performance | 集約コマンドの後にフィルタ |
| `W007` | warning | correctness | 未知の eval 関数名 |

## SPL2 Lint ルール一覧

| ID | 重大度 | カテゴリ | 説明 |
|---|---|---|---|
| `S001` | error | syntax | SPL2 構文エラー (パース失敗) |
| `S002` | warning | correctness | 未知の SPL2 コマンド名 |
| `S003` | warning | correctness | 未知の SPL2 eval/stats 関数名 |
| `S004` | warning | correctness | `.` による文字列連結 (SPL2 では `+` を使用) |
| `S005` | info | style | 特殊文字フィールド名にシングルクォートがない |
| `S006` | warning | correctness | 予約語をフィールド名として使用 |

## SPL2 対応コマンド一覧 (50)

| カテゴリ | コマンド |
|---|---|
| DataSource | `from`, `into`, `loadjob`, `makeresults`, `mstats`, `tstats`, `union` |
| Search | `search`, `where`, `dedup`, `head`, `reverse` |
| Reporting | `stats`, `eventstats`, `streamstats`, `timechart`, `timewrap` |
| Eval | `eval`, `bin`, `convert`, `addinfo` |
| Field | `fields`, `rename`, `table`, `rex`, `spath`, `makemv`, `mvcombine`, `mvexpand`, `nomv`, `fillnull`, `replace`, `untable`, `fieldsummary`, `flatten`, `expand` |
| Sort | `sort` |
| Lookup | `lookup` |
| Join | `join`, `append`, `appendcols`, `appendpipe` |
| Geo | `iplocation` |
| Security | `decrypt` |
| EventType | `typer` |
| Tags | `tags` |
| Flow | `branch`, `route`, `thru` |
| Interop | `spl1` |
| OCSF | `ocsf` |

## SPL2 対応 eval 関数一覧 (97)

| カテゴリ | 関数 |
|---|---|
| Bitwise | `bit_and`, `bit_or`, `bit_not`, `bit_xor`, `bit_shift_left`, `bit_shift_right` |
| Conditional | `case`, `cidrmatch`, `coalesce`, `false`, `if`, `in`, `like`, `lookup`, `match`, `null`, `nullif`, `searchmatch`, `true`, `validate` |
| Conversion | `ipmask`, `printf`, `toarray`, `tobool`, `todouble`, `toint`, `tojson`, `tomv`, `tonumber`, `toobject`, `tostring`, `to_ocsf` |
| Cryptographic | `md5`, `sha1`, `sha256`, `sha512` |
| DateTime | `now`, `relative_time`, `strftime`, `strptime`, `time` |
| Informational | `isarray`, `isbool`, `isdouble`, `isint`, `ismv`, `isnotnull`, `isnull`, `isnum`, `isobject`, `isstr`, `typeof` |
| JSON | `json`, `json_object`, `json_append`, `json_array`, `json_array_to_mv`, `json_delete`, `json_entries`, `json_extend`, `json_extract`, `json_extract_exact`, `json_has_key_exact`, `json_keys`, `json_set`, `json_set_exact`, `json_valid` |
| Mathematical | `abs`, `ceiling`, `ceil`, `exact`, `exp`, `floor`, `ln`, `log`, `pi`, `pow`, `round`, `sigfig`, `sqrt`, `sum` |
| Multivalue | `commands`, `mvappend`, `mvcount`, `mvdedup`, `mvfilter`, `mvfind`, `mvindex`, `mvjoin`, `mvmap`, `mvrange`, `mvreverse`, `mvsort`, `mvzip`, `mv_to_json_array`, `split` |
| Statistical | `avg`, `max`, `min`, `random` |
| Text | `len`, `lower`, `ltrim`, `replace`, `rtrim`, `spath`, `substr`, `trim`, `upper`, `urldecode` |
| Trigonometric | `acos`, `acosh`, `asin`, `asinh`, `atan`, `atan2`, `atanh`, `cos`, `cosh`, `hypot`, `sin`, `sinh`, `tan`, `tanh` |
| HigherOrder | `all`, `any`, `filter`, `map`, `reduce` |
| Object | `object_to_array` |

## SPL2 対応 stats 関数一覧 (41)

| カテゴリ | 関数 |
|---|---|
| Aggregate | `avg`, `count`, `dc`, `distinct_count`, `estdc`, `estdc_error`, `exactperc`, `max`, `mean`, `median`, `min`, `mode`, `perc`, `percentile`, `range`, `stdev`, `stdevp`, `sum`, `sumsq`, `upperperc`, `var`, `varp` |
| EventOrder | `earliest`, `earliest_time`, `first`, `last`, `latest`, `latest_time` |
| Multivalue | `list`, `values` |
| Rate | `per_day`, `per_hour`, `per_minute`, `per_second`, `rate`, `rate_avg`, `rate_sum` |
| SPL2 | `dataset`, `pivot`, `span`, `sparkline` |

## 新しいルールの追加方法

1. `src/linter/rules/` に新しいファイルを作成します
2. `Rule` トレイトを実装します (`id`, `description`, `check`)
3. `src/linter/rules/mod.rs` にモジュールを追加します
4. `src/linter/mod.rs` の `LintEngine::new()` にルールを登録します
5. `src/linter/mod.rs` の `#[cfg(test)]` にテストを追加します

```rust
// Rule トレイト
pub trait Rule {
    fn id(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn check(&self, query: &Query, source: &str) -> Vec<Diagnostic>;
}
```

## 依存ライブラリ

| ライブラリ | 用途 |
|---|---|
| `clap` (4) | CLI 引数解析 |
| `miette` (7) | ソース位置付きエラー表示 |
| `serde` + `serde_json` (1) | JSON 出力, Serialize 導出 |
| `thiserror` (2) | エラー型定義 |
| `insta` (1, dev) | スナップショットテスト |

## コーディング規約

- ファイル末尾に改行を入れます (POSIX 仕様)
- テストは各モジュール内の `#[cfg(test)] mod tests` に記述します
- AST の走査はルールごとに再帰的に行います (`check_stage` -> `check_search` -> `check_expr`)
- 新しい AST ノードを追加した場合、全ルールの `check_expr` / `check_search` に分岐を追加します
