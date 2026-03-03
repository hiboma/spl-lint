use crate::diagnostic::Diagnostic;
use crate::linter::rule::Rule;
use crate::parser::ast::*;

/// W005: AND/OR 演算子の優先順位が曖昧な式を検出するルールです。
/// SPL では AND が OR より結合が強いです。括弧なしで AND と OR を混在させている場合に警告します。
pub(crate) struct AmbiguousPrecedence;

impl Rule for AmbiguousPrecedence {
    fn id(&self) -> &'static str {
        "W005"
    }

    fn description(&self) -> &'static str {
        "ambiguous AND/OR precedence without parentheses"
    }

    fn check(&self, query: &Query, _source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        for stage in &query.stages {
            self.check_stage(stage, &mut diagnostics);
        }
        diagnostics
    }
}

impl AmbiguousPrecedence {
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
            // OR の左辺か右辺に括弧なしの AND がある場合が要注意です
            SearchExpr::Or(left, right) => {
                if self.contains_ungrouped_and(left) || self.contains_ungrouped_and(right) {
                    diagnostics.push(Diagnostic::warning(
                        "W005",
                        "AND/OR used without parentheses; AND binds tighter than OR in SPL. Consider adding explicit parentheses.",
                        span,
                    ));
                }
                self.check_search(left, span, diagnostics);
                self.check_search(right, span, diagnostics);
            }
            SearchExpr::And(left, right) => {
                self.check_search(left, span, diagnostics);
                self.check_search(right, span, diagnostics);
            }
            SearchExpr::Not(inner) => {
                self.check_search(inner, span, diagnostics);
            }
            SearchExpr::Grouped(_)
            | SearchExpr::FreeText(_)
            | SearchExpr::FieldFilter { .. }
            | SearchExpr::Wildcard(_)
            | SearchExpr::SubSearch(_) => {}
        }
    }

    /// 式が括弧で囲まれていない AND を直接含むか判定します。
    fn contains_ungrouped_and(&self, search: &SearchExpr) -> bool {
        matches!(search, SearchExpr::And(..))
    }
}
