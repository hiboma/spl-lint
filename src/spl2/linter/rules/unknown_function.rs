use crate::diagnostic::Diagnostic;
use crate::spl2::linter::known_eval_functions::is_known_spl2_eval_function;
use crate::spl2::linter::known_stats_functions::is_known_spl2_stats_function;
use crate::spl2::linter::rule::Spl2Rule;
use crate::spl2::parser::ast::*;

/// S003: 未知の SPL2 eval/stats 関数名の使用を検出するルールです。
pub(crate) struct Spl2UnknownFunction;

impl Spl2Rule for Spl2UnknownFunction {
    fn id(&self) -> &'static str {
        "S003"
    }

    fn description(&self) -> &'static str {
        "unknown SPL2 eval/stats function name"
    }

    fn check(&self, query: &Spl2Query, _source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        for stage in &query.stages {
            self.check_stage(stage, &mut diagnostics);
        }
        diagnostics
    }
}

impl Spl2UnknownFunction {
    fn check_stage(&self, stage: &Spl2PipelineStage, diagnostics: &mut Vec<Diagnostic>) {
        match &stage.kind {
            Spl2StageKind::Command(cmd) => {
                for arg in &cmd.arguments {
                    match arg {
                        Spl2CommandArg::Positional(expr, _) => self.check_expr(expr, diagnostics),
                        Spl2CommandArg::Named { value, .. } => self.check_expr(value, diagnostics),
                    }
                }
                if let Some(ref by_exprs) = cmd.by_clause {
                    for expr in by_exprs {
                        self.check_expr(expr, diagnostics);
                    }
                }
            }
            Spl2StageKind::FromStatement(stmt) => {
                if let Some(ref where_clause) = stmt.where_clause {
                    self.check_expr(where_clause, diagnostics);
                }
                if let Some(ref select) = stmt.select {
                    for item in select {
                        self.check_expr(&item.expr, diagnostics);
                    }
                }
                if let Some(ref group_by) = stmt.group_by {
                    for expr in group_by {
                        self.check_expr(expr, diagnostics);
                    }
                }
                if let Some(ref having) = stmt.having {
                    self.check_expr(having, diagnostics);
                }
            }
            Spl2StageKind::SelectStatement(stmt) => {
                for item in &stmt.items {
                    self.check_expr(&item.expr, diagnostics);
                }
                if let Some(ref where_clause) = stmt.where_clause {
                    self.check_expr(where_clause, diagnostics);
                }
                if let Some(ref group_by) = stmt.group_by {
                    for expr in group_by {
                        self.check_expr(expr, diagnostics);
                    }
                }
                if let Some(ref having) = stmt.having {
                    self.check_expr(having, diagnostics);
                }
            }
        }
    }

    fn check_expr(&self, expr: &Spl2Expr, diagnostics: &mut Vec<Diagnostic>) {
        match expr {
            Spl2Expr::FunctionCall(fc) => {
                if !is_known_spl2_eval_function(&fc.name) && !is_known_spl2_stats_function(&fc.name)
                {
                    diagnostics.push(Diagnostic::warning(
                        "S003",
                        format!("unknown function '{}'", fc.name),
                        fc.span,
                    ));
                }
                for arg in &fc.arguments {
                    match arg {
                        Spl2FunctionArg::Positional(expr) => self.check_expr(expr, diagnostics),
                        Spl2FunctionArg::Named { value, .. } => self.check_expr(value, diagnostics),
                    }
                }
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
            Spl2Expr::InList { expr, values, .. } => {
                self.check_expr(expr, diagnostics);
                for v in values {
                    self.check_expr(v, diagnostics);
                }
            }
            Spl2Expr::Between {
                expr, low, high, ..
            } => {
                self.check_expr(expr, diagnostics);
                self.check_expr(low, diagnostics);
                self.check_expr(high, diagnostics);
            }
            Spl2Expr::IsNull { expr, .. } => {
                self.check_expr(expr, diagnostics);
            }
            Spl2Expr::Like { expr, pattern, .. } => {
                self.check_expr(expr, diagnostics);
                self.check_expr(pattern, diagnostics);
            }
            _ => {}
        }
    }
}
