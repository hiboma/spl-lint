use crate::lexer::token::Span;

/// クエリ全体を表します。パイプラインステージのリストです。
#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    pub stages: Vec<PipelineStage>,
    pub span: Span,
}

/// パイプラインの 1 ステージを表します。
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineStage {
    pub kind: StageKind,
    pub span: Span,
}

/// ステージの種類を表します。
#[derive(Debug, Clone, PartialEq)]
pub enum StageKind {
    /// search 式 (暗黙的/明示的 search コマンド)
    Search(SearchExpr),
    /// コマンド (stats, eval, where, table 等)
    Command(Command),
    /// バッククォートマクロ呼び出し (`macro_name(args)`)
    MacroCall(String),
}

/// search 式を表します。
#[derive(Debug, Clone, PartialEq)]
pub enum SearchExpr {
    /// フリーテキスト
    FreeText(String),
    /// フィールドフィルタ (field op value)
    FieldFilter {
        field: String,
        op: CompareOp,
        value: FilterValue,
    },
    /// 論理 AND
    And(Box<SearchExpr>, Box<SearchExpr>),
    /// 論理 OR
    Or(Box<SearchExpr>, Box<SearchExpr>),
    /// 論理 NOT
    Not(Box<SearchExpr>),
    /// 括弧で囲まれた式
    Grouped(Box<SearchExpr>),
    /// ワイルドカード
    Wildcard(String),
    /// サブサーチ ([search ...])
    SubSearch(Box<Query>),
}

/// 比較演算子を表します。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompareOp {
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
}

/// フィルタの右辺値を表します。
#[derive(Debug, Clone, PartialEq)]
pub enum FilterValue {
    /// 文字列リテラル
    String(String),
    /// 数値
    Number(f64),
    /// ワイルドカード
    Wildcard(String),
    /// フィールド参照
    Field(String),
    /// 真偽値
    Bool(bool),
}

/// コマンドを表します。
#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub name: String,
    pub arguments: Vec<CommandArg>,
    pub by_clause: Option<Vec<String>>,
    pub as_clause: Option<String>,
    pub span: Span,
}

/// コマンドの引数を表します。
#[derive(Debug, Clone, PartialEq)]
pub enum CommandArg {
    /// 位置引数
    Positional(Expr),
    /// 名前付き引数 (name=value)
    Named { name: String, value: Expr },
}

/// 関数呼び出しを表します。
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<FunctionArg>,
    pub span: Span,
}

/// 関数の引数を表します。
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionArg {
    /// 位置引数
    Positional(Expr),
    /// 名前付き引数 (name=value)
    Named { name: String, value: Expr },
}

/// 式を表します。
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// 数値リテラル
    Number(f64),
    /// 文字列リテラル
    String(String),
    /// 真偽値リテラル
    Bool(bool),
    /// フィールド参照
    Field(String),
    /// ワイルドカード
    Wildcard(String),
    /// 関数呼び出し
    FunctionCall(FunctionCall),
    /// 二項演算 (算術)
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    /// 単項演算
    UnaryOp { op: UnaryOp, operand: Box<Expr> },
    /// サブサーチ ([...] ブロック内のパイプライン)
    SubSearch(Box<Query>),
    /// 比較式 (引数内で使用: field != value)
    CompareExpr {
        left: Box<Expr>,
        op: CompareOp,
        right: Box<Expr>,
    },
}

/// 二項演算子を表します。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    /// 文字列連結 (`.`)
    Concat,
}

/// 単項演算子を表します。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
}
