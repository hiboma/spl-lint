use crate::diagnostic::Diagnostic;
use crate::linter::known_commands::is_known_command;
use crate::linter::rule::Rule;
use crate::parser::ast::*;

/// W002: 未知のコマンド名の使用を検出するルールです。
pub(crate) struct UnknownCommand;

impl Rule for UnknownCommand {
    fn id(&self) -> &'static str {
        "W002"
    }

    fn description(&self) -> &'static str {
        "unknown command name"
    }

    fn check(&self, query: &Query, _source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        for stage in &query.stages {
            self.check_stage(stage, &mut diagnostics);
        }
        diagnostics
    }
}

impl UnknownCommand {
    fn check_stage(&self, stage: &PipelineStage, diagnostics: &mut Vec<Diagnostic>) {
        if let StageKind::Command(cmd) = &stage.kind {
            if !is_known_command(&cmd.name) {
                diagnostics.push(Diagnostic::warning(
                    "W002",
                    format!("unknown command '{}'", cmd.name),
                    cmd.span,
                ));
            }
            // サブサーチ内の引数も検査します
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
            Expr::SubSearch(query) => {
                for stage in &query.stages {
                    self.check_stage(stage, diagnostics);
                }
            }
            Expr::FunctionCall(fc) => {
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
