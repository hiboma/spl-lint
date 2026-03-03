use serde::Serialize;

/// ソースコード上の位置を表します。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Span {
    /// 開始位置 (バイトオフセット)
    pub start: usize,
    /// 終了位置 (バイトオフセット、排他的)
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

/// トークンの種類を表します。
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // リテラル
    /// 整数リテラル
    Integer(i64),
    /// 浮動小数点リテラル
    Float(f64),
    /// 文字列リテラル ("..." で囲まれた文字列)
    StringLiteral(String),
    /// ワイルドカード付き文字列 (*foo*, foo*)
    Wildcard(String),

    // 識別子・キーワード
    /// 識別子 (フィールド名、コマンド名など)
    Identifier(String),

    // キーワード
    And,
    Or,
    Not,
    As,
    By,
    True,
    False,

    // 演算子
    /// `=`
    Eq,
    /// `!=`
    NotEq,
    /// `<`
    Lt,
    /// `<=`
    LtEq,
    /// `>`
    Gt,
    /// `>=`
    GtEq,
    /// `|`
    Pipe,
    /// `!`
    Bang,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `..`
    DotDot,

    // 区切り記号
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `[`
    LBracket,
    /// `]`
    RBracket,
    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `:`
    Colon,

    // マクロ・テンプレート
    /// バッククォートで囲まれたマクロ呼び出し (`macro_name(args)`)
    BacktickMacro(String),
    /// テンプレート変数 (<<Name>>)
    TemplateVar(String),

    // 特殊
    /// ファイル終端
    Eof,
    /// 不正なトークン
    Error(String),
}

/// ソース位置付きのトークンを表します。
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}
