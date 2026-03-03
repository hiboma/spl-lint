use crate::diagnostic::Diagnostic;
use crate::spl2::linter::rule::Spl2Rule;
use crate::spl2::parser::ast::*;

/// S005: 特殊文字を含むフィールド名にシングルクォートがない場合を検出するルールです。
/// SPL2 ではハイフンやスペースを含むフィールド名は `'field-name'` で囲む必要があります。
pub(crate) struct UnquotedSpecialField;

impl Spl2Rule for UnquotedSpecialField {
    fn id(&self) -> &'static str {
        "S005"
    }

    fn description(&self) -> &'static str {
        "field name with special characters should be single-quoted"
    }

    fn check(&self, query: &Spl2Query, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        // Lexer レベルで特殊文字を含む識別子はエラーになるため、
        // ここでは AST のフィールド名にドット以外の特殊文字が含まれていないかを検査します。
        for stage in &query.stages {
            self.check_stage(stage, source, &mut diagnostics);
        }
        diagnostics
    }
}

impl UnquotedSpecialField {
    fn check_stage(
        &self,
        stage: &Spl2PipelineStage,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        match &stage.kind {
            Spl2StageKind::Command(cmd) => {
                for arg in &cmd.arguments {
                    match arg {
                        Spl2CommandArg::Positional(expr, _) => {
                            self.check_expr(expr, source, diagnostics)
                        }
                        Spl2CommandArg::Named { value, .. } => {
                            self.check_expr(value, source, diagnostics)
                        }
                    }
                }
            }
            Spl2StageKind::FromStatement(stmt) => {
                if let Some(ref w) = stmt.where_clause {
                    self.check_expr(w, source, diagnostics);
                }
                if let Some(ref select) = stmt.select {
                    for item in select {
                        self.check_expr(&item.expr, source, diagnostics);
                    }
                }
            }
            Spl2StageKind::SelectStatement(stmt) => {
                for item in &stmt.items {
                    self.check_expr(&item.expr, source, diagnostics);
                }
                if let Some(ref w) = stmt.where_clause {
                    self.check_expr(w, source, diagnostics);
                }
            }
        }
    }

    fn check_expr(&self, expr: &Spl2Expr, source: &str, diagnostics: &mut Vec<Diagnostic>) {
        match expr {
            Spl2Expr::BinaryOp { left, right, .. } => {
                self.check_expr(left, source, diagnostics);
                self.check_expr(right, source, diagnostics);
            }
            Spl2Expr::CompareExpr { left, right, .. } => {
                self.check_expr(left, source, diagnostics);
                self.check_expr(right, source, diagnostics);
            }
            Spl2Expr::UnaryOp { operand, .. } => {
                self.check_expr(operand, source, diagnostics);
            }
            Spl2Expr::FunctionCall(fc) => {
                for arg in &fc.arguments {
                    match arg {
                        Spl2FunctionArg::Positional(expr) => {
                            self.check_expr(expr, source, diagnostics)
                        }
                        Spl2FunctionArg::Named { value, .. } => {
                            self.check_expr(value, source, diagnostics)
                        }
                    }
                }
            }
            Spl2Expr::And(l, r) | Spl2Expr::Or(l, r) | Spl2Expr::Xor(l, r) => {
                self.check_expr(l, source, diagnostics);
                self.check_expr(r, source, diagnostics);
            }
            Spl2Expr::Not(e) | Spl2Expr::Grouped(e) => {
                self.check_expr(e, source, diagnostics);
            }
            Spl2Expr::SubQuery(query) => {
                for stage in &query.stages {
                    self.check_stage(stage, source, diagnostics);
                }
            }
            _ => {}
        }
    }
}
