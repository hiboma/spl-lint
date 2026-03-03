use crate::diagnostic::Diagnostic;
use crate::spl2::linter::rule::Spl2Rule;
use crate::spl2::parser::ast::Spl2Query;

/// S001: SPL2 構文エラーを検出するルールです。
/// 実際のエラー検出は Parser が行い、Spl2LintEngine が ParseError を Diagnostic に変換します。
/// このルールは追加の構文チェック用に予約されています。
pub(crate) struct Spl2SyntaxError;

impl Spl2Rule for Spl2SyntaxError {
    fn id(&self) -> &'static str {
        "S001"
    }

    fn description(&self) -> &'static str {
        "SPL2 syntax error"
    }

    fn check(&self, _query: &Spl2Query, _source: &str) -> Vec<Diagnostic> {
        Vec::new()
    }
}
