use crate::diagnostic::Diagnostic;
use crate::spl2::parser::ast::Spl2Query;

/// SPL2 用の Lint ルールのトレイトです。
/// 各ルールはこのトレイトを実装し、SPL2 AST を検査して診断メッセージを返します。
pub trait Spl2Rule {
    /// ルール ID を返します (例: "S002")。
    fn id(&self) -> &'static str;

    /// ルールの説明を返します。
    fn description(&self) -> &'static str;

    /// AST を検査し、検出した問題を診断メッセージとして返します。
    fn check(&self, query: &Spl2Query, source: &str) -> Vec<Diagnostic>;
}
