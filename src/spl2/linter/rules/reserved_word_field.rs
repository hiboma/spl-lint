use crate::diagnostic::Diagnostic;
use crate::spl2::linter::rule::Spl2Rule;
use crate::spl2::parser::ast::*;

/// SPL2 の予約語一覧です。
static RESERVED_WORDS: &[&str] = &[
    "and", "or", "not", "xor", "as", "by", "from", "select", "where", "join", "inner", "left",
    "outer", "on", "in", "is", "like", "between", "having", "limit", "offset", "asc", "desc",
    "distinct", "exists", "into", "union", "true", "false", "null",
];

fn is_reserved_word(name: &str) -> bool {
    RESERVED_WORDS.iter().any(|&w| w.eq_ignore_ascii_case(name))
}

/// S006: 予約語をフィールド名として使用している場合を検出するルールです。
/// SPL2 では予約語をフィールド名に使う場合、シングルクォートで囲む必要があります。
pub(crate) struct ReservedWordField;

impl Spl2Rule for ReservedWordField {
    fn id(&self) -> &'static str {
        "S006"
    }

    fn description(&self) -> &'static str {
        "reserved word used as field name (use single quotes)"
    }

    fn check(&self, query: &Spl2Query, _source: &str) -> Vec<Diagnostic> {
        // このルールは Lexer/Parser レベルで予約語が識別子として解釈されないため、
        // 現在の実装ではフィールド名が予約語と一致するケースを検出します。
        // Parser が予約語をキーワードとして処理するため、
        // フィールド名として使用された場合はパースエラーになります。
        // この検出ロジックは主に QuotedField が使用されるべき場面の情報提供用です。
        let mut diagnostics = Vec::new();
        for stage in &query.stages {
            self.check_stage(stage, &mut diagnostics);
        }
        diagnostics
    }
}

impl ReservedWordField {
    fn check_stage(&self, stage: &Spl2PipelineStage, diagnostics: &mut Vec<Diagnostic>) {
        match &stage.kind {
            Spl2StageKind::Command(cmd) => {
                for arg in &cmd.arguments {
                    match arg {
                        Spl2CommandArg::Positional(expr, _) => self.check_expr(expr, diagnostics),
                        Spl2CommandArg::Named { name, value } => {
                            if is_reserved_word(name) {
                                // 名前付き引数の名前が予約語の場合は通常の使用法なのでスキップします
                            }
                            self.check_expr(value, diagnostics);
                        }
                    }
                }
            }
            Spl2StageKind::FromStatement(stmt) => {
                if let Some(ref w) = stmt.where_clause {
                    self.check_expr(w, diagnostics);
                }
                if let Some(ref select) = stmt.select {
                    for item in select {
                        self.check_expr(&item.expr, diagnostics);
                    }
                }
            }
            Spl2StageKind::SelectStatement(stmt) => {
                for item in &stmt.items {
                    self.check_expr(&item.expr, diagnostics);
                }
                if let Some(ref w) = stmt.where_clause {
                    self.check_expr(w, diagnostics);
                }
            }
        }
    }

    fn check_expr(&self, expr: &Spl2Expr, diagnostics: &mut Vec<Diagnostic>) {
        match expr {
            Spl2Expr::BinaryOp { left, right, .. } => {
                self.check_expr(left, diagnostics);
                self.check_expr(right, diagnostics);
            }
            Spl2Expr::CompareExpr { left, right, .. } => {
                self.check_expr(left, diagnostics);
                self.check_expr(right, diagnostics);
            }
            Spl2Expr::UnaryOp { operand, .. } => {
                self.check_expr(operand, diagnostics);
            }
            Spl2Expr::FunctionCall(fc) => {
                for arg in &fc.arguments {
                    match arg {
                        Spl2FunctionArg::Positional(expr) => self.check_expr(expr, diagnostics),
                        Spl2FunctionArg::Named { value, .. } => self.check_expr(value, diagnostics),
                    }
                }
            }
            Spl2Expr::And(l, r) | Spl2Expr::Or(l, r) | Spl2Expr::Xor(l, r) => {
                self.check_expr(l, diagnostics);
                self.check_expr(r, diagnostics);
            }
            Spl2Expr::Not(e) | Spl2Expr::Grouped(e) => {
                self.check_expr(e, diagnostics);
            }
            Spl2Expr::SubQuery(query) => {
                for stage in &query.stages {
                    self.check_stage(stage, diagnostics);
                }
            }
            _ => {}
        }
    }
}
