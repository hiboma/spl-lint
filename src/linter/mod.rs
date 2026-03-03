pub mod known_commands;
pub mod known_eval_functions;
pub mod known_stats_functions;
pub mod rule;
pub mod rules;

use crate::diagnostic::Diagnostic;
use crate::lexer::Lexer;
use crate::parser::Parser;
use rule::Rule;

/// Lint エンジンです。登録されたルールを AST に対して実行します。
pub struct LintEngine {
    rules: Vec<Box<dyn Rule>>,
}

impl LintEngine {
    /// デフォルトのルールセットで LintEngine を作成します。
    pub fn new() -> Self {
        let rules: Vec<Box<dyn Rule>> = vec![
            Box::new(rules::syntax_error::SyntaxError),
            Box::new(rules::wildcard_only_search::WildcardOnlySearch),
            Box::new(rules::unknown_command::UnknownCommand),
            Box::new(rules::missing_args::MissingArgs),
            Box::new(rules::pipe_style::PipeStyle),
            Box::new(rules::ambiguous_precedence::AmbiguousPrecedence),
            Box::new(rules::filter_after_aggregation::FilterAfterAggregation),
            Box::new(rules::unknown_eval_function::UnknownEvalFunction),
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

        // E001: 構文エラーを診断に変換します
        for err in &parse_errors {
            diagnostics.push(Diagnostic::error("E001", &err.message, err.span));
        }

        // AST が取得できた場合、各ルールを実行します
        if let Some(ref query) = query {
            for rule in &self.rules {
                // E001 は parse_errors から直接追加済みなのでスキップします
                if rule.id() == "E001" {
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
    pub fn rules(&self) -> &[Box<dyn Rule>] {
        &self.rules
    }
}

impl Default for LintEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostic::Severity;

    #[test]
    fn test_lint_valid_query() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("status=200 | stats count by src_ip");
        assert!(
            diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            diagnostics
        );
    }

    #[test]
    fn test_lint_syntax_error() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("(status=200");
        assert!(!diagnostics.is_empty());
        assert_eq!(diagnostics[0].rule_id, "E001");
        assert_eq!(diagnostics[0].severity, Severity::Error);
    }

    #[test]
    fn test_lint_unknown_command() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("status=200 | fooBarBaz count");
        assert!(!diagnostics.is_empty());
        let w002: Vec<_> = diagnostics.iter().filter(|d| d.rule_id == "W002").collect();
        assert_eq!(w002.len(), 1);
    }

    #[test]
    fn test_lint_known_command() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("status=200 | stats count by src_ip");
        assert!(
            diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            diagnostics
        );
    }

    #[test]
    fn test_lint_pipeline_with_unknown_command() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("status=404 | myCustomCmd x");
        let w002_count = diagnostics.iter().filter(|d| d.rule_id == "W002").count();
        assert_eq!(w002_count, 1);
    }

    #[test]
    fn test_lint_multiple_issues() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("status=404 | unknownA count | unknownB count");
        let w002_count = diagnostics.iter().filter(|d| d.rule_id == "W002").count();
        assert_eq!(w002_count, 2);
    }

    #[test]
    fn test_lint_empty_query() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("");
        assert!(diagnostics.is_empty());
    }

    // ---- W001: wildcard-only search ----

    #[test]
    fn test_w001_wildcard_only() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("*");
        let w001 = diagnostics.iter().filter(|d| d.rule_id == "W001").count();
        assert_eq!(w001, 1);
    }

    #[test]
    fn test_w001_wildcard_not_triggered() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("*error*");
        let w001 = diagnostics.iter().filter(|d| d.rule_id == "W001").count();
        assert_eq!(w001, 0);
    }

    // ---- W003: missing args ----

    #[test]
    fn test_w003_stats_no_args() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("status=200 | stats");
        let w003 = diagnostics.iter().filter(|d| d.rule_id == "W003").count();
        assert_eq!(w003, 1);
    }

    #[test]
    fn test_w003_stats_with_args() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("status=200 | stats count");
        let w003 = diagnostics.iter().filter(|d| d.rule_id == "W003").count();
        assert_eq!(w003, 0);
    }

    #[test]
    fn test_w003_table_no_args() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("status=200 | table");
        let w003 = diagnostics.iter().filter(|d| d.rule_id == "W003").count();
        assert_eq!(w003, 1);
    }

    #[test]
    fn test_w003_eval_with_args() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("status=200 | eval total = count * 2");
        let w003 = diagnostics.iter().filter(|d| d.rule_id == "W003").count();
        assert_eq!(w003, 0);
    }

    // ---- W004: pipe style ----

    #[test]
    fn test_w004_pipe_no_spaces() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("status=200|stats count");
        let w004 = diagnostics.iter().filter(|d| d.rule_id == "W004").count();
        assert_eq!(w004, 1);
    }

    #[test]
    fn test_w004_pipe_with_spaces() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("status=200 | stats count");
        let w004 = diagnostics.iter().filter(|d| d.rule_id == "W004").count();
        assert_eq!(w004, 0);
    }

    #[test]
    fn test_w004_pipe_missing_left_space() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("status=200| stats count");
        let w004 = diagnostics.iter().filter(|d| d.rule_id == "W004").count();
        assert_eq!(w004, 1);
    }

    // ---- W005: ambiguous precedence ----

    #[test]
    fn test_w005_and_or_no_parens() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("x=1 OR y=2 AND z=3");
        let w005 = diagnostics.iter().filter(|d| d.rule_id == "W005").count();
        assert_eq!(w005, 1);
    }

    #[test]
    fn test_w005_and_or_with_parens() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("x=1 OR (y=2 AND z=3)");
        let w005 = diagnostics.iter().filter(|d| d.rule_id == "W005").count();
        assert_eq!(w005, 0);
    }

    #[test]
    fn test_w005_only_or_no_warning() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("x=1 OR y=2 OR z=3");
        let w005 = diagnostics.iter().filter(|d| d.rule_id == "W005").count();
        assert_eq!(w005, 0);
    }

    // ---- W006: filter order ----

    #[test]
    fn test_w006_filter_after_aggregate() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("status=200 | stats count by host | status=200");
        let w006 = diagnostics.iter().filter(|d| d.rule_id == "W006").count();
        assert_eq!(w006, 1);
    }

    #[test]
    fn test_w006_filter_before_aggregate_ok() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("status=200 | stats count by host");
        let w006 = diagnostics.iter().filter(|d| d.rule_id == "W006").count();
        assert_eq!(w006, 0);
    }

    #[test]
    fn test_w006_timechart_then_filter() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("status=200 | timechart count | status=200");
        let w006 = diagnostics.iter().filter(|d| d.rule_id == "W006").count();
        assert_eq!(w006, 1);
    }

    // ---- W007: unknown eval function ----

    #[test]
    fn test_w007_unknown_eval_function() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("status=200 | eval x = fooBarBaz(y)");
        let w007 = diagnostics.iter().filter(|d| d.rule_id == "W007").count();
        assert_eq!(w007, 1);
    }

    #[test]
    fn test_w007_known_eval_function() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("status=200 | eval x = if(y > 0, 1, 0)");
        let w007 = diagnostics.iter().filter(|d| d.rule_id == "W007").count();
        assert_eq!(w007, 0);
    }

    #[test]
    fn test_w007_known_stats_function() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("status=200 | stats count(status) by host");
        let w007 = diagnostics.iter().filter(|d| d.rule_id == "W007").count();
        assert_eq!(w007, 0);
    }

    // ---- 複合テスト ----

    #[test]
    fn test_lint_multibyte_error_message() {
        let engine = LintEngine::new();
        let diagnostics = engine.lint("これはテスト | stats count");
        let e001: Vec<_> = diagnostics.iter().filter(|d| d.rule_id == "E001").collect();
        assert!(!e001.is_empty());
        for d in &e001 {
            assert!(
                !d.message.contains("'ã'"),
                "error message should not contain raw byte char: {}",
                d.message
            );
        }
    }

    #[test]
    fn test_lint_full_pipeline_no_issues() {
        let engine = LintEngine::new();
        let diagnostics =
            engine.lint("status=200 host=web01 | stats count by src_ip | sort count | head 10");
        assert!(
            diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            diagnostics
        );
    }

    // ---- production クエリ統合テスト ----

    #[test]
    fn test_production_queries_no_e001() {
        let engine = LintEngine::new();
        let dir = std::path::Path::new("testdata/spl/examples");
        let files: Vec<_> = std::fs::read_dir(dir)
            .expect("failed to read testdata/spl/examples/")
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|ext| ext == "spl"))
            .collect();

        assert!(
            !files.is_empty(),
            "no .spl files found in testdata/spl/examples/"
        );

        let mut failures = Vec::new();
        for path in &files {
            let source = std::fs::read_to_string(path).expect("failed to read file");
            let diagnostics = engine.lint(&source);
            let e001: Vec<_> = diagnostics.iter().filter(|d| d.rule_id == "E001").collect();
            if !e001.is_empty() {
                failures.push(format!(
                    "{}: {} E001 error(s) - {}",
                    path.display(),
                    e001.len(),
                    e001[0].message,
                ));
            }
        }

        assert!(
            failures.is_empty(),
            "E001 errors found in {} / {} production queries:\n{}",
            failures.len(),
            files.len(),
            failures.join("\n"),
        );
    }
}
