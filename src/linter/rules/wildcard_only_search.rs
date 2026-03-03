use crate::diagnostic::Diagnostic;
use crate::linter::rule::Rule;
use crate::parser::ast::*;

/// W001: `*` のみの search 式を検出するルールです。
/// `*` は全てのイベントにマッチするため、意図的でない場合は不要です。
pub(crate) struct WildcardOnlySearch;

impl Rule for WildcardOnlySearch {
    fn id(&self) -> &'static str {
        "W001"
    }

    fn description(&self) -> &'static str {
        "wildcard-only search matches all events"
    }

    fn check(&self, query: &Query, _source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        for stage in &query.stages {
            self.check_stage(stage, &mut diagnostics);
        }
        diagnostics
    }
}

impl WildcardOnlySearch {
    fn check_stage(&self, stage: &PipelineStage, diagnostics: &mut Vec<Diagnostic>) {
        if let StageKind::Search(search) = &stage.kind {
            self.check_search(search, stage.span, diagnostics);
        }
    }

    fn check_search(
        &self,
        search: &SearchExpr,
        span: crate::lexer::token::Span,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        match search {
            SearchExpr::FreeText(text) if text == "*" => {
                diagnostics.push(Diagnostic::warning(
                    "W001",
                    "'*' search matches all events and may be unnecessary",
                    span,
                ));
            }
            SearchExpr::And(left, right) | SearchExpr::Or(left, right) => {
                self.check_search(left, span, diagnostics);
                self.check_search(right, span, diagnostics);
            }
            SearchExpr::Not(inner) | SearchExpr::Grouped(inner) => {
                self.check_search(inner, span, diagnostics);
            }
            SearchExpr::SubSearch(query) => {
                for stage in &query.stages {
                    self.check_stage(stage, diagnostics);
                }
            }
            SearchExpr::FreeText(_) | SearchExpr::FieldFilter { .. } | SearchExpr::Wildcard(_) => {}
        }
    }
}
