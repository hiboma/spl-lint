use crate::diagnostic::Diagnostic;
use crate::lexer::token::Span;
use crate::spl2::linter::rule::Spl2Rule;
use crate::spl2::parser::ast::*;

/// S004: `.` による文字列連結の使用を検出するルールです。
/// SPL2 では `+` を使用します。
pub(crate) struct DeprecatedDotConcat;

impl Spl2Rule for DeprecatedDotConcat {
    fn id(&self) -> &'static str {
        "S004"
    }

    fn description(&self) -> &'static str {
        "deprecated '.' string concatenation (use '+' in SPL2)"
    }

    fn check(&self, query: &Spl2Query, _source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        for stage in &query.stages {
            self.check_stage(stage, &mut diagnostics);
        }
        diagnostics
    }
}

impl DeprecatedDotConcat {
    fn check_stage(&self, stage: &Spl2PipelineStage, diagnostics: &mut Vec<Diagnostic>) {
        match &stage.kind {
            Spl2StageKind::Command(cmd) => {
                for arg in &cmd.arguments {
                    match arg {
                        Spl2CommandArg::Positional(expr, _) => self.check_expr(expr, diagnostics),
                        Spl2CommandArg::Named { value, .. } => self.check_expr(value, diagnostics),
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
            Spl2Expr::BinaryOp {
                left,
                op: Spl2BinaryOp::Concat,
                right,
            } => {
                diagnostics.push(Diagnostic::warning(
                    "S004",
                    "use '+' instead of '.' for string concatenation in SPL2".to_string(),
                    Span::new(0, 0), // 正確な位置は AST に含まれないため簡略化しています
                ));
                self.check_expr(left, diagnostics);
                self.check_expr(right, diagnostics);
            }
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
            Spl2Expr::ArrayLiteral(elements) => {
                for e in elements {
                    self.check_expr(e, diagnostics);
                }
            }
            Spl2Expr::Lambda { body, .. } => {
                self.check_expr(body, diagnostics);
            }
            _ => {}
        }
    }
}
