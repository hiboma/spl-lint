use crate::lexer::token::Span;

/// SPL2 クエリ全体を表します。パイプラインステージのリストです。
#[derive(Debug, Clone, PartialEq)]
pub struct Spl2Query {
    pub stages: Vec<Spl2PipelineStage>,
    pub span: Span,
}

/// SPL2 パイプラインの 1 ステージを表します。
#[derive(Debug, Clone, PartialEq)]
pub struct Spl2PipelineStage {
    pub kind: Spl2StageKind,
    pub span: Span,
}

/// SPL2 ステージの種類を表します。
#[derive(Debug, Clone, PartialEq)]
pub enum Spl2StageKind {
    /// FROM dataset [WHERE ...] [GROUP BY ...] [SELECT ...] 文
    FromStatement(FromStatement),
    /// SELECT ... FROM ... WHERE ... 文
    SelectStatement(SelectStatement),
    /// コマンド (stats, eval, where, table 等)
    Command(Spl2Command),
}

/// FROM 文を表します。
#[derive(Debug, Clone, PartialEq)]
pub struct FromStatement {
    /// データソース名
    pub dataset: String,
    /// JOIN 句のリスト
    pub joins: Vec<JoinClause>,
    /// WHERE 句
    pub where_clause: Option<Box<Spl2Expr>>,
    /// GROUP BY 句
    pub group_by: Option<Vec<Spl2Expr>>,
    /// HAVING 句
    pub having: Option<Box<Spl2Expr>>,
    /// SELECT 句 (射影)
    pub select: Option<Vec<SelectItem>>,
    /// ORDER BY 句
    pub order_by: Option<Vec<OrderByItem>>,
    /// LIMIT 句
    pub limit: Option<Box<Spl2Expr>>,
    /// OFFSET 句
    pub offset: Option<Box<Spl2Expr>>,
    pub span: Span,
}

/// SELECT 文を表します。
#[derive(Debug, Clone, PartialEq)]
pub struct SelectStatement {
    /// DISTINCT かどうか
    pub distinct: bool,
    /// SELECT 項目
    pub items: Vec<SelectItem>,
    /// FROM 句のデータソース名
    pub from: Option<String>,
    /// JOIN 句のリスト
    pub joins: Vec<JoinClause>,
    /// WHERE 句
    pub where_clause: Option<Box<Spl2Expr>>,
    /// GROUP BY 句
    pub group_by: Option<Vec<Spl2Expr>>,
    /// HAVING 句
    pub having: Option<Box<Spl2Expr>>,
    /// ORDER BY 句
    pub order_by: Option<Vec<OrderByItem>>,
    /// LIMIT 句
    pub limit: Option<Box<Spl2Expr>>,
    /// OFFSET 句
    pub offset: Option<Box<Spl2Expr>>,
    pub span: Span,
}

/// SELECT 項目を表します。
#[derive(Debug, Clone, PartialEq)]
pub struct SelectItem {
    /// 式
    pub expr: Spl2Expr,
    /// AS 別名
    pub alias: Option<String>,
}

/// JOIN 句を表します。
#[derive(Debug, Clone, PartialEq)]
pub struct JoinClause {
    /// JOIN の種類
    pub join_type: JoinType,
    /// JOIN 対象のデータソース
    pub dataset: String,
    /// ON 条件
    pub on_condition: Option<Box<Spl2Expr>>,
}

/// JOIN の種類を表します。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JoinType {
    Inner,
    Left,
    LeftOuter,
}

/// ORDER BY 項目を表します。
#[derive(Debug, Clone, PartialEq)]
pub struct OrderByItem {
    pub expr: Spl2Expr,
    pub direction: SortDirection,
}

/// ソート方向を表します。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortDirection {
    Asc,
    Desc,
}

/// SPL2 コマンドを表します。
#[derive(Debug, Clone, PartialEq)]
pub struct Spl2Command {
    pub name: String,
    pub arguments: Vec<Spl2CommandArg>,
    pub by_clause: Option<Vec<Spl2Expr>>,
    pub as_clause: Option<String>,
    pub span: Span,
}

/// SPL2 コマンドの引数を表します。
#[derive(Debug, Clone, PartialEq)]
pub enum Spl2CommandArg {
    /// 位置引数 (AS 別名付き可)
    Positional(Spl2Expr, Option<String>),
    /// 名前付き引数 (name=value)
    Named { name: String, value: Spl2Expr },
}

/// SPL2 関数呼び出しを表します。
#[derive(Debug, Clone, PartialEq)]
pub struct Spl2FunctionCall {
    pub name: String,
    pub arguments: Vec<Spl2FunctionArg>,
    pub span: Span,
}

/// SPL2 関数の引数を表します。
#[derive(Debug, Clone, PartialEq)]
pub enum Spl2FunctionArg {
    /// 位置引数
    Positional(Spl2Expr),
    /// 名前付き引数 (name=value)
    Named { name: String, value: Spl2Expr },
}

/// 比較演算子を表します。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Spl2CompareOp {
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
}

/// 二項演算子を表します。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Spl2BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    /// `.` による文字列連結 (SPL2 では非推奨)
    Concat,
}

/// 単項演算子を表します。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Spl2UnaryOp {
    Neg,
    Not,
}

/// SPL2 式を表します。
#[derive(Debug, Clone, PartialEq)]
pub enum Spl2Expr {
    /// 数値リテラル
    Number(f64),
    /// 文字列リテラル
    String(String),
    /// 真偽値リテラル
    Bool(bool),
    /// null リテラル
    Null,
    /// フィールド参照
    Field(String),
    /// シングルクォートフィールド参照
    QuotedField(String),
    /// ワイルドカード
    Wildcard(String),
    /// 関数呼び出し
    FunctionCall(Spl2FunctionCall),
    /// 二項演算 (算術)
    BinaryOp {
        left: Box<Spl2Expr>,
        op: Spl2BinaryOp,
        right: Box<Spl2Expr>,
    },
    /// 比較式
    CompareExpr {
        left: Box<Spl2Expr>,
        op: Spl2CompareOp,
        right: Box<Spl2Expr>,
    },
    /// 単項演算
    UnaryOp {
        op: Spl2UnaryOp,
        operand: Box<Spl2Expr>,
    },
    /// 論理 AND
    And(Box<Spl2Expr>, Box<Spl2Expr>),
    /// 論理 OR
    Or(Box<Spl2Expr>, Box<Spl2Expr>),
    /// 論理 XOR
    Xor(Box<Spl2Expr>, Box<Spl2Expr>),
    /// 論理 NOT
    Not(Box<Spl2Expr>),
    /// IN リスト (expr IN (v1, v2, ...))
    InList {
        expr: Box<Spl2Expr>,
        values: Vec<Spl2Expr>,
        negated: bool,
    },
    /// BETWEEN (expr BETWEEN a AND b)
    Between {
        expr: Box<Spl2Expr>,
        low: Box<Spl2Expr>,
        high: Box<Spl2Expr>,
        negated: bool,
    },
    /// IS NULL / IS NOT NULL
    IsNull { expr: Box<Spl2Expr>, negated: bool },
    /// LIKE (expr LIKE pattern)
    Like {
        expr: Box<Spl2Expr>,
        pattern: Box<Spl2Expr>,
        negated: bool,
    },
    /// 配列リテラル [1, 2, 3]
    ArrayLiteral(Vec<Spl2Expr>),
    /// オブジェクトリテラル {key: "value"}
    ObjectLiteral(Vec<(String, Spl2Expr)>),
    /// ラムダ式 ($a -> $a + 10)
    Lambda {
        params: Vec<String>,
        body: Box<Spl2Expr>,
    },
    /// パラメータ参照 ($variable)
    ParameterRef(String),
    /// 検索リテラル (`search literal`)
    SearchLiteral(String),
    /// サブクエリ
    SubQuery(Box<Spl2Query>),
    /// 括弧で囲まれた式
    Grouped(Box<Spl2Expr>),
    /// Star (*)
    Star,
}
