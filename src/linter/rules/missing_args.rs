use crate::diagnostic::Diagnostic;
use crate::linter::rule::Rule;
use crate::parser::ast::*;

/// W003: コマンドの必須引数が不足している場合に警告するルールです。
pub(crate) struct MissingArgs;

/// コマンド名と最小引数数の定義です (by_clause を含まない)。
static REQUIRED_ARGS: &[(&str, usize)] = &[
    ("stats", 1),
    ("chart", 1),
    ("timechart", 1),
    ("eval", 1),
    ("rename", 1),
    ("rex", 1),
    ("table", 1),
    ("fields", 1),
    ("lookup", 1),
    ("where", 1),
    ("sort", 1),
    ("bin", 1),
    ("bucket", 1),
    ("top", 1),
    ("rare", 1),
    ("join", 1),
    ("dedup", 1),
    ("replace", 1),
    ("convert", 1),
    ("fillnull", 1),
];

impl Rule for MissingArgs {
    fn id(&self) -> &'static str {
        "W003"
    }

    fn description(&self) -> &'static str {
        "command is missing required arguments"
    }

    fn check(&self, query: &Query, _source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        for stage in &query.stages {
            self.check_stage(stage, &mut diagnostics);
        }
        diagnostics
    }
}

impl MissingArgs {
    fn check_stage(&self, stage: &PipelineStage, diagnostics: &mut Vec<Diagnostic>) {
        if let StageKind::Command(cmd) = &stage.kind {
            let total_args = cmd.arguments.len()
                + cmd.by_clause.as_ref().map(|b| b.len()).unwrap_or(0)
                + if cmd.as_clause.is_some() { 1 } else { 0 };

            for &(name, min_args) in REQUIRED_ARGS {
                if cmd.name.eq_ignore_ascii_case(name) && total_args < min_args {
                    diagnostics.push(Diagnostic::warning(
                        "W003",
                        format!(
                            "command '{}' requires at least {} argument(s), but {} provided",
                            cmd.name, min_args, total_args
                        ),
                        cmd.span,
                    ));
                    break;
                }
            }

            // 引数内のサブサーチも検査します
            for arg in &cmd.arguments {
                match arg {
                    CommandArg::Positional(expr) => self.check_expr(expr, diagnostics),
                    CommandArg::Named { value, .. } => self.check_expr(value, diagnostics),
                }
            }
        }
    }

    fn check_expr(&self, expr: &Expr, diagnostics: &mut Vec<Diagnostic>) {
        if let Expr::SubSearch(query) = expr {
            for stage in &query.stages {
                self.check_stage(stage, diagnostics);
            }
        }
    }
}
