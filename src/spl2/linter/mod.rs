pub mod known_commands;
pub mod known_eval_functions;
pub mod known_stats_functions;
pub mod rule;
pub mod rules;

use crate::diagnostic::Diagnostic;
use crate::spl2::lexer::Lexer;
use crate::spl2::parser::Parser;
use rule::Spl2Rule;

/// SPL2 用の Lint エンジンです。登録されたルールを AST に対して実行します。
pub struct Spl2LintEngine {
    rules: Vec<Box<dyn Spl2Rule>>,
}

impl Spl2LintEngine {
    /// デフォルトのルールセットで Spl2LintEngine を作成します。
    pub fn new() -> Self {
        let rules: Vec<Box<dyn Spl2Rule>> = vec![
            Box::new(rules::syntax_error::Spl2SyntaxError),
            Box::new(rules::unknown_command::Spl2UnknownCommand),
            Box::new(rules::unknown_function::Spl2UnknownFunction),
            Box::new(rules::deprecated_dot_concat::DeprecatedDotConcat),
            Box::new(rules::unquoted_special_field::UnquotedSpecialField),
            Box::new(rules::reserved_word_field::ReservedWordField),
        ];
        Self { rules }
    }

    /// クエリ文字列に対して lint を実行し、診断メッセージのリストを返します。
    pub fn lint(&self, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // Lexer
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();

        // Parser
        let parser = Parser::new(tokens);
        let (query, parse_errors) = parser.parse();

        // S001: 構文エラーを診断に変換します
        for err in &parse_errors {
            diagnostics.push(Diagnostic::error("S001", &err.message, err.span));
        }

        // AST が取得できた場合、各ルールを実行します
        if let Some(ref query) = query {
            for rule in &self.rules {
                // S001 は parse_errors から直接追加済みなのでスキップします
                if rule.id() == "S001" {
                    if parse_errors.is_empty() {
                        diagnostics.extend(rule.check(query, source));
                    }
                    continue;
                }
                diagnostics.extend(rule.check(query, source));
            }
        }

        // span の開始位置でソートします
        diagnostics.sort_by_key(|d| d.span.start);
        diagnostics
    }

    /// 登録されているルールの一覧を返します。
    pub fn rules(&self) -> &[Box<dyn Spl2Rule>] {
        &self.rules
    }
}

impl Default for Spl2LintEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lint_valid_spl2_query() {
        let engine = Spl2LintEngine::new();
        let diagnostics = engine.lint("FROM main WHERE status=200 | stats count() BY host");
        assert!(
            diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            diagnostics
        );
    }

    #[test]
    fn test_lint_empty_query() {
        let engine = Spl2LintEngine::new();
        let diagnostics = engine.lint("");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_lint_select_query() {
        let engine = Spl2LintEngine::new();
        let diagnostics =
            engine.lint("SELECT count() AS cnt FROM main WHERE status=200 GROUP BY host");
        assert!(
            diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            diagnostics
        );
    }

    #[test]
    fn test_lint_unknown_command() {
        let engine = Spl2LintEngine::new();
        let diagnostics = engine.lint("FROM main | fooBarBaz count");
        let s002: Vec<_> = diagnostics.iter().filter(|d| d.rule_id == "S002").collect();
        assert_eq!(s002.len(), 1);
    }

    #[test]
    fn test_lint_unknown_function() {
        let engine = Spl2LintEngine::new();
        let diagnostics = engine.lint("FROM main | eval x = unknownFunc(y)");
        let s003: Vec<_> = diagnostics.iter().filter(|d| d.rule_id == "S003").collect();
        assert_eq!(s003.len(), 1);
    }

    #[test]
    fn test_lint_known_function() {
        let engine = Spl2LintEngine::new();
        let diagnostics = engine.lint("FROM main | eval x = if(y > 0, 1, 0)");
        let s003: Vec<_> = diagnostics.iter().filter(|d| d.rule_id == "S003").collect();
        assert_eq!(s003.len(), 0);
    }

    #[test]
    fn test_lint_deprecated_dot_concat() {
        let engine = Spl2LintEngine::new();
        let diagnostics = engine.lint("FROM main | eval x = a . b");
        let s004: Vec<_> = diagnostics.iter().filter(|d| d.rule_id == "S004").collect();
        assert_eq!(s004.len(), 1);
    }

    #[test]
    fn test_lint_pipeline_multiple_stages() {
        let engine = Spl2LintEngine::new();
        let diagnostics =
            engine.lint("FROM main WHERE status=200 | stats count() BY host | sort count");
        assert!(
            diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            diagnostics
        );
    }
}
