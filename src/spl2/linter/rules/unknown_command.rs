use crate::diagnostic::Diagnostic;
use crate::spl2::linter::known_commands::is_known_spl2_command;
use crate::spl2::linter::rule::Spl2Rule;
use crate::spl2::parser::ast::*;

/// S002: 未知の SPL2 コマンド名の使用を検出するルールです。
pub(crate) struct Spl2UnknownCommand;

impl Spl2Rule for Spl2UnknownCommand {
    fn id(&self) -> &'static str {
        "S002"
    }

    fn description(&self) -> &'static str {
        "unknown SPL2 command name"
    }

    fn check(&self, query: &Spl2Query, _source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        for stage in &query.stages {
            self.check_stage(stage, &mut diagnostics);
        }
        diagnostics
    }
}

impl Spl2UnknownCommand {
    fn check_stage(&self, stage: &Spl2PipelineStage, diagnostics: &mut Vec<Diagnostic>) {
        if let Spl2StageKind::Command(cmd) = &stage.kind {
            if !is_known_spl2_command(&cmd.name) {
                diagnostics.push(Diagnostic::warning(
                    "S002",
                    format!("unknown SPL2 command '{}'", cmd.name),
                    cmd.span,
                ));
            }
            // 引数内のサブクエリも検査します
            for arg in &cmd.arguments {
                match arg {
                    Spl2CommandArg::Positional(expr, _) => self.check_expr(expr, diagnostics),
                    Spl2CommandArg::Named { value, .. } => self.check_expr(value, diagnostics),
                }
            }
        }
    }

    fn check_expr(&self, expr: &Spl2Expr, diagnostics: &mut Vec<Diagnostic>) {
        match expr {
            Spl2Expr::SubQuery(query) => {
                for stage in &query.stages {
                    self.check_stage(stage, diagnostics);
                }
            }
            Spl2Expr::FunctionCall(fc) => {
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
            Spl2Expr::Not(e) => self.check_expr(e, diagnostics),
            Spl2Expr::Grouped(e) => self.check_expr(e, diagnostics),
            Spl2Expr::ArrayLiteral(elements) => {
                for e in elements {
                    self.check_expr(e, diagnostics);
                }
            }
            Spl2Expr::Lambda { body, .. } => self.check_expr(body, diagnostics),
            _ => {}
        }
    }
}
