use crate::diagnostic::Diagnostic;
use crate::linter::known_commands::is_aggregate_command;
use crate::linter::rule::Rule;
use crate::parser::ast::*;

/// W006: 集約コマンドの後にフィルタがある場合に警告するルールです。
/// フィルタは集約の前に配置した方がパフォーマンスが向上します。
pub(crate) struct FilterAfterAggregation;

impl Rule for FilterAfterAggregation {
    fn id(&self) -> &'static str {
        "W006"
    }

    fn description(&self) -> &'static str {
        "filter after aggregate may reduce performance"
    }

    fn check(&self, query: &Query, _source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let mut seen_aggregate = false;

        for stage in &query.stages {
            match &stage.kind {
                StageKind::Command(cmd) => {
                    if is_aggregate_command(&cmd.name) {
                        seen_aggregate = true;
                    }
                }
                StageKind::Search(_) => {
                    if seen_aggregate {
                        diagnostics.push(Diagnostic::info(
                            "W006",
                            "filter placed after aggregate command; consider moving filters before aggregation for better performance",
                            stage.span,
                        ));
                    }
                }
                StageKind::MacroCall(_) => {}
            }
        }

        diagnostics
    }
}
