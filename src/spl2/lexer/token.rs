/// ソースコード上の位置を表します。
/// SPL1 の Span と同じ構造ですが、SPL2 モジュール内で独立して使用します。
pub use crate::lexer::token::Span;

/// SPL2 トークンの種類を表します。
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // --- リテラル ---
    /// 整数リテラル
    Integer(i64),
    /// 浮動小数点リテラル
    Float(f64),
    /// 文字列リテラル ("..." で囲まれた文字列)
    StringLiteral(String),
    /// Raw 文字列リテラル (@"..." で囲まれた文字列)
    RawString(String),
    /// シングルクォートで囲まれたフィールド名 ('field-name')
    SingleQuotedField(String),
    /// null リテラル
    Null,
    /// ワイルドカード付き文字列 (*foo*, foo*)
    Wildcard(String),

    // --- 識別子 ---
    /// 識別子 (フィールド名、コマンド名など)
    Identifier(String),

    // --- 論理キーワード ---
    And,
    Or,
    Not,
    Xor,

    // --- SPL キーワード ---
    As,
    By,
    True,
    False,

    // --- SQL キーワード ---
    From,
    Select,
    Where,
    GroupBy,
    Having,
    OrderBy,
    Limit,
    Offset,
    Join,
    Inner,
    Left,
    Outer,
    On,
    Asc,
    Desc,
    Distinct,
    In,
    Is,
    Like,
    Between,
    Exists,
    Into,
    Union,

    // --- 演算子 ---
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
    /// `->`
    Arrow,
    /// `..`
    DotDot,

    // --- 区切り記号 ---
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `[`
    LBracket,
    /// `]`
    RBracket,
    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `:`
    Colon,
    /// `$`
    Dollar,

    // --- 特殊リテラル ---
    /// バッククォートで囲まれた検索リテラル (`search literal`)
    SearchLiteral(String),

    // --- 特殊 ---
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
