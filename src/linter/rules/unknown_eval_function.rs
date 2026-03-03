use crate::diagnostic::Diagnostic;
use crate::linter::known_eval_functions::is_known_eval_function;
use crate::linter::known_stats_functions::is_known_stats_function;
use crate::linter::rule::Rule;
use crate::parser::ast::*;

/// W007: 未知の eval 関数名の使用を検出するルールです。
pub(crate) struct UnknownEvalFunction;

impl Rule for UnknownEvalFunction {
    fn id(&self) -> &'static str {
        "W007"
    }

    fn description(&self) -> &'static str {
        "unknown eval/stats function name"
    }

    fn check(&self, query: &Query, _source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        for stage in &query.stages {
            self.check_stage(stage, &mut diagnostics);
        }
        diagnostics
    }
}

impl UnknownEvalFunction {
    fn check_stage(&self, stage: &PipelineStage, diagnostics: &mut Vec<Diagnostic>) {
        if let StageKind::Command(cmd) = &stage.kind {
            for arg in &cmd.arguments {
                match arg {
                    CommandArg::Positional(expr) => self.check_expr(expr, diagnostics),
                    CommandArg::Named { value, .. } => self.check_expr(value, diagnostics),
                }
            }
        }
    }

    fn check_expr(&self, expr: &Expr, diagnostics: &mut Vec<Diagnostic>) {
        match expr {
            Expr::FunctionCall(fc) => {
                if !is_known_eval_function(&fc.name) && !is_known_stats_function(&fc.name) {
                    diagnostics.push(Diagnostic::warning(
                        "W007",
                        format!("unknown function '{}'", fc.name),
                        fc.span,
                    ));
                }
                // 引数内の式も検査します
                for arg in &fc.arguments {
                    match arg {
                        FunctionArg::Positional(expr) => self.check_expr(expr, diagnostics),
                        FunctionArg::Named { value, .. } => self.check_expr(value, diagnostics),
                    }
                }
            }
            Expr::BinaryOp { left, right, .. } => {
                self.check_expr(left, diagnostics);
                self.check_expr(right, diagnostics);
            }
            Expr::UnaryOp { operand, .. } => {
                self.check_expr(operand, diagnostics);
            }
            Expr::SubSearch(query) => {
                for stage in &query.stages {
                    self.check_stage(stage, diagnostics);
                }
            }
            Expr::CompareExpr { left, right, .. } => {
                self.check_expr(left, diagnostics);
                self.check_expr(right, diagnostics);
            }
            Expr::Number(_)
            | Expr::String(_)
            | Expr::Bool(_)
            | Expr::Field(_)
            | Expr::Wildcard(_) => {}
        }
    }
}
