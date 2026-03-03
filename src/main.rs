use std::io::Read;
use std::process;

use clap::Parser;
use miette::{LabeledSpan, Severity as MietteSeverity, miette};

use spl_lint::diagnostic::{Diagnostic, Severity};
use spl_lint::linter::LintEngine;
use spl_lint::linter::known_commands::KNOWN_COMMAND_ENTRIES;
use spl_lint::linter::known_eval_functions::KNOWN_EVAL_FUNCTION_ENTRIES;
use spl_lint::spl2::linter::Spl2LintEngine;
use spl_lint::spl2::linter::known_commands::KNOWN_SPL2_COMMAND_ENTRIES;
use spl_lint::spl2::linter::known_eval_functions::KNOWN_SPL2_EVAL_FUNCTION_ENTRIES;

#[derive(Parser)]
#[command(
    name = "spl-lint",
    version,
    about = "A linter for Splunk Search Processing Language (SPL)"
)]
struct Cli {
    /// lint 対象のファイルパス (指定しない場合は標準入力から読み込みます)
    files: Vec<String>,

    /// 出力フォーマット
    #[arg(long, default_value = "text", value_parser = ["text", "json"])]
    format: String,

    /// 無効にするルール ID (カンマ区切り)
    #[arg(long, value_delimiter = ',')]
    disable: Vec<String>,

    /// 登録されているルールの一覧を表示します
    #[arg(long)]
    list_rules: bool,

    /// SPL コマンド一覧を表示します
    #[arg(long)]
    list_commands: bool,

    /// eval 関数一覧を表示します
    #[arg(long)]
    list_functions: bool,

    /// バッククォートコメントを除去して整形済みクエリを出力します
    #[arg(long)]
    trim: bool,

    /// --trim と併用し、整形結果をファイルに直接書き戻します
    #[arg(long, requires = "trim")]
    write: bool,

    /// lint 成功時にメッセージを表示します
    #[arg(long, short)]
    verbose: bool,

    /// SPL2 モードで lint を実行します
    #[arg(long)]
    spl2: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.list_rules {
        if cli.spl2 {
            print_spl2_rules();
        } else {
            print_rules();
        }
        return;
    }

    if cli.list_commands {
        if cli.spl2 {
            print_spl2_commands();
        } else {
            print_commands();
        }
        return;
    }

    if cli.list_functions {
        if cli.spl2 {
            print_spl2_functions();
        } else {
            print_functions();
        }
        return;
    }

    if cli.spl2 {
        run_spl2_mode(&cli);
        return;
    }

    let engine = LintEngine::new();
    let mut has_errors = false;

    let mut file_count: usize = 0;

    if cli.files.is_empty() {
        // 標準入力から読み込みます
        let mut source = String::new();
        if let Err(e) = std::io::stdin().read_to_string(&mut source) {
            eprintln!("error: failed to read stdin: {}", e);
            process::exit(2);
        }
        file_count = 1;
        if cli.trim {
            let trimmed = trim_source(&source);
            let diagnostics = run_lint(&engine, &trimmed, &cli.disable);
            if !diagnostics.is_empty() {
                has_errors = true;
                print_diagnostics(&diagnostics, &trimmed, "<stdin>", &cli.format);
            } else {
                print!("{}", trimmed);
            }
        } else {
            let diagnostics = run_lint(&engine, &source, &cli.disable);
            if !diagnostics.is_empty() {
                has_errors = true;
                print_diagnostics(&diagnostics, &source, "<stdin>", &cli.format);
            }
        }
    } else {
        for path in &cli.files {
            match std::fs::read_to_string(path) {
                Ok(source) => {
                    file_count += 1;
                    if cli.trim {
                        let trimmed = trim_source(&source);
                        let diagnostics = run_lint(&engine, &trimmed, &cli.disable);
                        if !diagnostics.is_empty() {
                            has_errors = true;
                            print_diagnostics(&diagnostics, &trimmed, path, &cli.format);
                        } else if cli.write {
                            if let Err(e) = std::fs::write(path, &trimmed) {
                                eprintln!("error: failed to write '{}': {}", path, e);
                                has_errors = true;
                            }
                        } else {
                            print!("{}", trimmed);
                        }
                    } else {
                        let diagnostics = run_lint(&engine, &source, &cli.disable);
                        if !diagnostics.is_empty() {
                            has_errors = true;
                            print_diagnostics(&diagnostics, &source, path, &cli.format);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("error: failed to read '{}': {}", path, e);
                    has_errors = true;
                }
            }
        }
    }

    if has_errors {
        process::exit(1);
    }

    if cli.verbose {
        println!("ok: {} file(s) checked, no issues found.", file_count);
    }
}

fn run_lint(engine: &LintEngine, source: &str, disable: &[String]) -> Vec<Diagnostic> {
    engine
        .lint(source)
        .into_iter()
        .filter(|d| !disable.iter().any(|id| id == &d.rule_id))
        .collect()
}

fn print_rules() {
    let engine = LintEngine::new();
    for rule in engine.rules() {
        println!("{}: {}", rule.id(), rule.description());
    }
}

fn print_commands() {
    let mut current_category = "";
    for entry in KNOWN_COMMAND_ENTRIES {
        if entry.category != current_category {
            if !current_category.is_empty() {
                println!();
            }
            println!("[{}]", entry.category);
            current_category = entry.category;
        }
        println!("  {}", entry.name);
    }
}

fn print_functions() {
    let mut current_category = "";
    for entry in KNOWN_EVAL_FUNCTION_ENTRIES {
        if entry.category != current_category {
            if !current_category.is_empty() {
                println!();
            }
            println!("[{}]", entry.category);
            current_category = entry.category;
        }
        println!("  {}", entry.name);
    }
}

fn print_diagnostics(diagnostics: &[Diagnostic], source: &str, filename: &str, format: &str) {
    match format {
        "json" => print_json(diagnostics, filename),
        _ => print_text(diagnostics, source, filename),
    }
}

fn print_json(diagnostics: &[Diagnostic], filename: &str) {
    #[derive(serde::Serialize)]
    struct JsonOutput<'a> {
        file: &'a str,
        diagnostics: &'a [Diagnostic],
    }

    let output = JsonOutput {
        file: filename,
        diagnostics,
    };
    match serde_json::to_string_pretty(&output) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("error: failed to serialize JSON: {}", e),
    }
}

fn print_text(diagnostics: &[Diagnostic], source: &str, filename: &str) {
    for d in diagnostics {
        let (line, col) = offset_to_line_col(source, d.span.start);
        let miette_severity = match d.severity {
            Severity::Error => MietteSeverity::Error,
            Severity::Warning => MietteSeverity::Warning,
            Severity::Info => MietteSeverity::Advice,
        };

        let span_len = if d.span.end > d.span.start {
            d.span.end - d.span.start
        } else {
            1
        };

        let report = miette!(
            severity = miette_severity,
            labels = vec![LabeledSpan::at(
                d.span.start..d.span.start + span_len,
                &d.rule_id
            )],
            "{}:{}:{}: [{}] {}",
            filename,
            line,
            col,
            d.rule_id,
            d.message
        )
        .with_source_code(source.to_string());

        eprintln!("{:?}", report);
    }
}

/// 各行の末尾の空白文字を除去し、ファイル末尾を改行 1 つで終端します。
/// トップレベルの行頭空白を除去します。
/// バッククォートコメントを除去します。
fn trim_source(source: &str) -> String {
    let mut bracket_depth: usize = 0;
    let mut result: String = source
        .lines()
        .map(|line| {
            let trimmed = line.trim_end();
            let is_toplevel = bracket_depth == 0;
            // トップレベルの場合のみ行頭空白を除去します
            let trimmed = if is_toplevel {
                trimmed.trim_start()
            } else {
                trimmed
            };
            // 括弧の深さを更新します
            bracket_depth = update_nesting_depth(trimmed, bracket_depth);
            let line_result = remove_backtick_comments(trimmed);
            // コメント除去後に残る先頭空白もトップレベルでは除去します
            if is_toplevel {
                line_result.trim_start().to_string()
            } else {
                line_result
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    // 末尾に改行が 1 つだけ入るようにします (POSIX 準拠)
    if !result.is_empty() {
        result.push('\n');
    }

    result
}

/// 文字列リテラルを考慮しながら行内の括弧の深さを更新します。
fn update_nesting_depth(line: &str, depth: usize) -> usize {
    let bytes = line.as_bytes();
    let mut d = depth;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'"' => {
                i += 1;
                while i < bytes.len() {
                    if bytes[i] == b'\\' {
                        i += 2;
                    } else if bytes[i] == b'"' {
                        i += 1;
                        break;
                    } else {
                        i += 1;
                    }
                }
            }
            b'`' => {
                // バッククォートコメントをスキップします
                i += 1;
                while i < bytes.len() && bytes[i] != b'`' {
                    i += 1;
                }
                if i < bytes.len() {
                    i += 1;
                }
            }
            b'[' | b'(' => {
                d += 1;
                i += 1;
            }
            b']' | b')' => {
                d = d.saturating_sub(1);
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }
    d
}

/// バッククォートコメントを除去します。
fn remove_backtick_comments(line: &str) -> String {
    let bytes = line.as_bytes();
    let mut result = String::new();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'"' => {
                result.push('"');
                i += 1;
                while i < bytes.len() {
                    if bytes[i] == b'\\' {
                        result.push('\\');
                        i += 1;
                        if i < bytes.len() {
                            result.push(bytes[i] as char);
                            i += 1;
                        }
                    } else if bytes[i] == b'"' {
                        result.push('"');
                        i += 1;
                        break;
                    } else {
                        result.push(bytes[i] as char);
                        i += 1;
                    }
                }
            }
            b'`' => {
                // バッククォートコメントをスキップします
                i += 1;
                while i < bytes.len() && bytes[i] != b'`' {
                    i += 1;
                }
                if i < bytes.len() {
                    i += 1;
                }
            }
            _ => {
                result.push(bytes[i] as char);
                i += 1;
            }
        }
    }
    // 末尾の空白を除去します (先頭は trim_source 側で制御します)
    result.trim_end().to_string()
}

fn run_spl2_mode(cli: &Cli) {
    let engine = Spl2LintEngine::new();
    let mut has_errors = false;
    let mut file_count: usize = 0;

    if cli.files.is_empty() {
        let mut source = String::new();
        if let Err(e) = std::io::stdin().read_to_string(&mut source) {
            eprintln!("error: failed to read stdin: {}", e);
            process::exit(2);
        }
        file_count = 1;
        let diagnostics = run_spl2_lint(&engine, &source, &cli.disable);
        if !diagnostics.is_empty() {
            has_errors = true;
            print_diagnostics(&diagnostics, &source, "<stdin>", &cli.format);
        }
    } else {
        for path in &cli.files {
            match std::fs::read_to_string(path) {
                Ok(source) => {
                    file_count += 1;
                    let diagnostics = run_spl2_lint(&engine, &source, &cli.disable);
                    if !diagnostics.is_empty() {
                        has_errors = true;
                        print_diagnostics(&diagnostics, &source, path, &cli.format);
                    }
                }
                Err(e) => {
                    eprintln!("error: failed to read '{}': {}", path, e);
                    has_errors = true;
                }
            }
        }
    }

    if has_errors {
        process::exit(1);
    }

    if cli.verbose {
        println!("ok: {} file(s) checked, no issues found.", file_count);
    }
}

fn run_spl2_lint(engine: &Spl2LintEngine, source: &str, disable: &[String]) -> Vec<Diagnostic> {
    engine
        .lint(source)
        .into_iter()
        .filter(|d| !disable.iter().any(|id| id == &d.rule_id))
        .collect()
}

fn print_spl2_rules() {
    let engine = Spl2LintEngine::new();
    for rule in engine.rules() {
        println!("{}: {}", rule.id(), rule.description());
    }
}

fn print_spl2_commands() {
    let mut current_category = "";
    for entry in KNOWN_SPL2_COMMAND_ENTRIES {
        if entry.category != current_category {
            if !current_category.is_empty() {
                println!();
            }
            println!("[{}]", entry.category);
            current_category = entry.category;
        }
        println!("  {}", entry.name);
    }
}

fn print_spl2_functions() {
    let mut current_category = "";
    for entry in KNOWN_SPL2_EVAL_FUNCTION_ENTRIES {
        if entry.category != current_category {
            if !current_category.is_empty() {
                println!();
            }
            println!("[{}]", entry.category);
            current_category = entry.category;
        }
        println!("  {}", entry.name);
    }
}

/// バイトオフセットを行番号・列番号 (1-based) に変換します。
fn offset_to_line_col(source: &str, offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    for (i, ch) in source.char_indices() {
        if i >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset_to_line_col_start() {
        assert_eq!(offset_to_line_col("hello", 0), (1, 1));
    }

    #[test]
    fn test_offset_to_line_col_middle() {
        assert_eq!(offset_to_line_col("hello", 3), (1, 4));
    }

    #[test]
    fn test_offset_to_line_col_newline() {
        assert_eq!(offset_to_line_col("hello\nworld", 6), (2, 1));
    }

    #[test]
    fn test_offset_to_line_col_second_line() {
        assert_eq!(offset_to_line_col("hello\nworld", 8), (2, 3));
    }

    #[test]
    fn test_trim_source_trailing_spaces() {
        assert_eq!(trim_source("hello   \nworld  \n"), "hello\nworld\n");
    }

    #[test]
    fn test_trim_source_trailing_tabs() {
        assert_eq!(trim_source("hello\t\t\nworld\n"), "hello\nworld\n");
    }

    #[test]
    fn test_trim_source_no_trailing_newline() {
        assert_eq!(trim_source("hello"), "hello\n");
    }

    #[test]
    fn test_trim_source_empty() {
        assert_eq!(trim_source(""), "");
    }

    #[test]
    fn test_trim_source_backtick_comment_removed() {
        assert_eq!(trim_source("`comment` status=200\n"), "status=200\n");
    }

    #[test]
    fn test_trim_source_inline_backtick_comment() {
        assert_eq!(trim_source("status=200 `inline comment`\n"), "status=200\n");
    }

    #[test]
    fn test_trim_source_leading_spaces_at_toplevel() {
        assert_eq!(
            trim_source("  status=200\n  | stats count\n"),
            "status=200\n| stats count\n"
        );
    }

    #[test]
    fn test_trim_source_subsearch_indent_preserved() {
        let input = "| join [\n    search status=200\n    | stats count\n  ]\n";
        let expected = "| join [\n    search status=200\n    | stats count\n  ]\n";
        assert_eq!(trim_source(input), expected);
    }

    #[test]
    fn test_trim_source_string_with_backtick_preserved() {
        assert_eq!(
            trim_source(r#"status="test`value""#),
            "status=\"test`value\"\n"
        );
    }
}
