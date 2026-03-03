pub mod token;

use token::{Span, Token, TokenKind};

/// SPL の字句解析器です。
pub(crate) struct Lexer<'a> {
    source: &'a str,
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            bytes: source.as_bytes(),
            pos: 0,
        }
    }

    /// 全てのトークンを返します。末尾に Eof トークンを含みます。
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            let is_eof = tok.kind == TokenKind::Eof;
            tokens.push(tok);
            if is_eof {
                break;
            }
        }
        tokens
    }

    /// 次のトークンを 1 つ返します。
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();

        if self.pos >= self.bytes.len() {
            return Token::new(TokenKind::Eof, Span::new(self.pos, self.pos));
        }

        let start = self.pos;
        let ch = self.bytes[self.pos];

        match ch {
            b'"' => self.lex_string_literal(start),
            b'\'' => self.lex_single_quoted_string(start),
            b'`' => self.lex_backtick_macro(start),
            b'0'..=b'9' => self.lex_number(start),
            b'|' => self.single_char_token(TokenKind::Pipe, start),
            b'(' => self.single_char_token(TokenKind::LParen, start),
            b')' => self.single_char_token(TokenKind::RParen, start),
            b'[' => self.single_char_token(TokenKind::LBracket, start),
            b']' => self.single_char_token(TokenKind::RBracket, start),
            b',' => self.single_char_token(TokenKind::Comma, start),
            b'+' => self.single_char_token(TokenKind::Plus, start),
            b'-' => self.lex_minus(start),
            b'/' => self.single_char_token(TokenKind::Slash, start),
            b'%' => self.single_char_token(TokenKind::Percent, start),
            b'*' => self.lex_star_or_wildcard(start),
            b'=' => self.lex_eq(start),
            b'!' => self.lex_bang(start),
            b'<' => self.lex_lt(start),
            b'>' => self.lex_gt(start),
            b'.' => self.lex_dot(start),
            b':' => self.single_char_token(TokenKind::Colon, start),
            _ if is_ident_start(ch) => self.lex_identifier_or_wildcard(start),
            _ if !ch.is_ascii() => {
                // 連続する非ASCIIバイトをまとめて1つのErrorトークンにします
                while self.pos < self.bytes.len() && !self.bytes[self.pos].is_ascii() {
                    self.pos += 1;
                }
                let text = String::from_utf8_lossy(&self.bytes[start..self.pos]);
                Token::new(
                    TokenKind::Error(format!("unexpected text: '{}'", text)),
                    Span::new(start, self.pos),
                )
            }
            _ => {
                self.pos += 1;
                Token::new(
                    TokenKind::Error(format!("unexpected character: '{}'", ch as char)),
                    Span::new(start, self.pos),
                )
            }
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    fn single_char_token(&mut self, kind: TokenKind, start: usize) -> Token {
        self.pos += 1;
        Token::new(kind, Span::new(start, self.pos))
    }

    fn lex_string_literal(&mut self, start: usize) -> Token {
        self.pos += 1; // 開始の " をスキップします
        let mut value = String::new();

        while self.pos < self.bytes.len() {
            match self.bytes[self.pos] {
                b'"' => {
                    self.pos += 1;
                    return Token::new(TokenKind::StringLiteral(value), Span::new(start, self.pos));
                }
                b'\\' => {
                    self.pos += 1;
                    if self.pos < self.bytes.len() {
                        match self.bytes[self.pos] {
                            b'"' => value.push('"'),
                            b'\\' => value.push('\\'),
                            b'n' => value.push('\n'),
                            b't' => value.push('\t'),
                            b'r' => value.push('\r'),
                            other => {
                                value.push('\\');
                                value.push(other as char);
                            }
                        }
                        self.pos += 1;
                    }
                }
                _ => {
                    value.push(self.bytes[self.pos] as char);
                    self.pos += 1;
                }
            }
        }

        Token::new(
            TokenKind::Error("unterminated string literal".to_string()),
            Span::new(start, self.pos),
        )
    }

    fn lex_single_quoted_string(&mut self, start: usize) -> Token {
        self.pos += 1; // 開始の `'` をスキップします
        let mut value = String::new();

        while self.pos < self.bytes.len() {
            match self.bytes[self.pos] {
                b'\'' => {
                    self.pos += 1;
                    return Token::new(TokenKind::StringLiteral(value), Span::new(start, self.pos));
                }
                b'\\' => {
                    self.pos += 1;
                    if self.pos < self.bytes.len() {
                        match self.bytes[self.pos] {
                            b'\'' => value.push('\''),
                            b'\\' => value.push('\\'),
                            other => {
                                value.push('\\');
                                value.push(other as char);
                            }
                        }
                        self.pos += 1;
                    }
                }
                _ => {
                    value.push(self.bytes[self.pos] as char);
                    self.pos += 1;
                }
            }
        }

        Token::new(
            TokenKind::Error("unterminated single-quoted string".to_string()),
            Span::new(start, self.pos),
        )
    }

    fn lex_backtick_macro(&mut self, start: usize) -> Token {
        self.pos += 1; // 開始の `` ` `` をスキップします
        let content_start = self.pos;

        while self.pos < self.bytes.len() && self.bytes[self.pos] != b'`' {
            self.pos += 1;
        }

        let content = self.source[content_start..self.pos].to_string();

        if self.pos < self.bytes.len() {
            self.pos += 1; // 閉じの `` ` `` をスキップします
        }

        Token::new(
            TokenKind::BacktickMacro(content),
            Span::new(start, self.pos),
        )
    }

    fn lex_number(&mut self, start: usize) -> Token {
        while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_digit() {
            self.pos += 1;
        }

        // 小数点があるか確認します
        if self.pos < self.bytes.len() && self.bytes[self.pos] == b'.' {
            // `..` (range) の場合は整数として返します
            if self.pos + 1 < self.bytes.len() && self.bytes[self.pos + 1] == b'.' {
                let text = &self.source[start..self.pos];
                let value = text.parse::<i64>().unwrap_or(0);
                return Token::new(TokenKind::Integer(value), Span::new(start, self.pos));
            }
            // `.` の後に数字が続く場合のみ浮動小数点として扱います
            if self.pos + 1 < self.bytes.len() && self.bytes[self.pos + 1].is_ascii_digit() {
                self.pos += 1; // `.` をスキップします
                while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_digit() {
                    self.pos += 1;
                }
                let text = &self.source[start..self.pos];
                let value = text.parse::<f64>().unwrap_or(0.0);
                return Token::new(TokenKind::Float(value), Span::new(start, self.pos));
            }
        }

        let text = &self.source[start..self.pos];
        let value = text.parse::<i64>().unwrap_or(0);
        Token::new(TokenKind::Integer(value), Span::new(start, self.pos))
    }

    fn lex_star_or_wildcard(&mut self, start: usize) -> Token {
        self.pos += 1; // `*` をスキップします

        // `*` の後に識別子文字が続く場合はワイルドカードとして扱います
        if self.pos < self.bytes.len() && is_ident_continue(self.bytes[self.pos]) {
            while self.pos < self.bytes.len()
                && (is_ident_continue(self.bytes[self.pos]) || self.bytes[self.pos] == b'*')
            {
                self.pos += 1;
            }
            let text = self.source[start..self.pos].to_string();
            return Token::new(TokenKind::Wildcard(text), Span::new(start, self.pos));
        }

        Token::new(TokenKind::Star, Span::new(start, self.pos))
    }

    fn lex_bang(&mut self, start: usize) -> Token {
        self.pos += 1;
        if self.pos < self.bytes.len() && self.bytes[self.pos] == b'=' {
            self.pos += 1;
            return Token::new(TokenKind::NotEq, Span::new(start, self.pos));
        }
        Token::new(TokenKind::Bang, Span::new(start, self.pos))
    }

    fn lex_lt(&mut self, start: usize) -> Token {
        self.pos += 1;
        if self.pos < self.bytes.len() && self.bytes[self.pos] == b'=' {
            self.pos += 1;
            return Token::new(TokenKind::LtEq, Span::new(start, self.pos));
        }
        // テンプレート変数 <<Name>> の検出
        if self.pos < self.bytes.len() && self.bytes[self.pos] == b'<' {
            self.pos += 1; // 2 番目の `<` を消費します
            let name_start = self.pos;
            while self.pos < self.bytes.len() && self.bytes[self.pos] != b'>' {
                self.pos += 1;
            }
            let name = self.source[name_start..self.pos].to_string();
            // `>>` を消費します
            if self.pos < self.bytes.len() && self.bytes[self.pos] == b'>' {
                self.pos += 1;
            }
            if self.pos < self.bytes.len() && self.bytes[self.pos] == b'>' {
                self.pos += 1;
            }
            return Token::new(TokenKind::TemplateVar(name), Span::new(start, self.pos));
        }
        Token::new(TokenKind::Lt, Span::new(start, self.pos))
    }

    fn lex_gt(&mut self, start: usize) -> Token {
        self.pos += 1;
        if self.pos < self.bytes.len() && self.bytes[self.pos] == b'=' {
            self.pos += 1;
            return Token::new(TokenKind::GtEq, Span::new(start, self.pos));
        }
        Token::new(TokenKind::Gt, Span::new(start, self.pos))
    }

    fn lex_eq(&mut self, start: usize) -> Token {
        self.pos += 1;
        // `==` は `=` と同じ等値比較です
        if self.pos < self.bytes.len() && self.bytes[self.pos] == b'=' {
            self.pos += 1;
        }
        Token::new(TokenKind::Eq, Span::new(start, self.pos))
    }

    fn lex_dot(&mut self, start: usize) -> Token {
        self.pos += 1;
        if self.pos < self.bytes.len() && self.bytes[self.pos] == b'.' {
            self.pos += 1;
            return Token::new(TokenKind::DotDot, Span::new(start, self.pos));
        }
        Token::new(TokenKind::Dot, Span::new(start, self.pos))
    }

    fn lex_minus(&mut self, start: usize) -> Token {
        self.pos += 1;
        // 負の数値リテラルとして扱う場合: `-` の後に数字が続く場合
        // ただし、前のトークンが値の場合は減算演算子として扱います
        // 簡略化のため、ここでは単純に Minus として返します
        Token::new(TokenKind::Minus, Span::new(start, self.pos))
    }

    fn lex_identifier_or_wildcard(&mut self, start: usize) -> Token {
        // 先頭文字は is_ident_start で判定済みなので無条件に消費します
        self.pos += 1;
        while self.pos < self.bytes.len() && is_ident_continue(self.bytes[self.pos]) {
            self.pos += 1;
        }

        // ドット区切りのフィールド名に対応します (例: src_ip.country)
        // 波括弧 `{}` 付きフィールドパスにも対応します (例: events{}.type)
        loop {
            // `{}` を識別子の一部として消費します (例: events{})
            if self.pos + 1 < self.bytes.len()
                && self.bytes[self.pos] == b'{'
                && self.bytes[self.pos + 1] == b'}'
            {
                self.pos += 2; // `{}` をスキップします
                continue;
            }

            // ドット区切り
            if self.pos < self.bytes.len() && self.bytes[self.pos] == b'.' {
                // `..` (range 演算子) の場合は中断します
                if self.pos + 1 < self.bytes.len() && self.bytes[self.pos + 1] == b'.' {
                    break;
                }
                // `.` の後に識別子文字が続く場合のみドット区切りとして扱います
                if self.pos + 1 < self.bytes.len() && is_ident_start(self.bytes[self.pos + 1]) {
                    self.pos += 1; // `.` をスキップします
                    while self.pos < self.bytes.len() && is_ident_continue(self.bytes[self.pos]) {
                        self.pos += 1;
                    }
                    continue;
                }
            }

            break;
        }

        // 識別子の後に `*` が続く場合はワイルドカードとして扱います
        if self.pos < self.bytes.len() && self.bytes[self.pos] == b'*' {
            self.pos += 1;
            // ワイルドカードの残りを読み取ります (例: foo*bar*)
            while self.pos < self.bytes.len()
                && (is_ident_continue(self.bytes[self.pos]) || self.bytes[self.pos] == b'*')
            {
                self.pos += 1;
            }
            let text = self.source[start..self.pos].to_string();
            return Token::new(TokenKind::Wildcard(text), Span::new(start, self.pos));
        }

        let text = &self.source[start..self.pos];
        let kind = match text {
            // AND, OR, NOT は大文字のみキーワードです
            "AND" => TokenKind::And,
            "OR" => TokenKind::Or,
            "NOT" => TokenKind::Not,
            // as, by は大文字小文字を区別しません
            _ if text.eq_ignore_ascii_case("as") => TokenKind::As,
            _ if text.eq_ignore_ascii_case("by") => TokenKind::By,
            _ if text.eq_ignore_ascii_case("true") => TokenKind::True,
            _ if text.eq_ignore_ascii_case("false") => TokenKind::False,
            _ => TokenKind::Identifier(text.to_string()),
        };
        Token::new(kind, Span::new(start, self.pos))
    }
}

fn is_ident_start(ch: u8) -> bool {
    ch.is_ascii_alphabetic() || ch == b'_' || ch == b'@'
}

fn is_ident_continue(ch: u8) -> bool {
    ch.is_ascii_alphanumeric() || ch == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(input);
        lexer.tokenize()
    }

    fn kinds(input: &str) -> Vec<TokenKind> {
        tokenize(input).into_iter().map(|t| t.kind).collect()
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(kinds(""), vec![TokenKind::Eof]);
    }

    #[test]
    fn test_whitespace_only() {
        assert_eq!(kinds("   \n\t  "), vec![TokenKind::Eof]);
    }

    #[test]
    fn test_backtick_macro() {
        assert_eq!(
            kinds("`this is a comment`"),
            vec![
                TokenKind::BacktickMacro("this is a comment".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_backtick_macro_before_token() {
        assert_eq!(
            kinds("`comment` foo"),
            vec![
                TokenKind::BacktickMacro("comment".to_string()),
                TokenKind::Identifier("foo".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_integer_literal() {
        assert_eq!(kinds("42"), vec![TokenKind::Integer(42), TokenKind::Eof]);
    }

    #[test]
    fn test_float_literal() {
        assert_eq!(kinds("3.14"), vec![TokenKind::Float(3.14), TokenKind::Eof]);
    }

    #[test]
    fn test_string_literal() {
        assert_eq!(
            kinds(r#""hello world""#),
            vec![
                TokenKind::StringLiteral("hello world".to_string()),
                TokenKind::Eof
            ]
        );
    }

    #[test]
    fn test_string_escape() {
        assert_eq!(
            kinds(r#""say \"hi\"""#),
            vec![
                TokenKind::StringLiteral("say \"hi\"".to_string()),
                TokenKind::Eof
            ]
        );
    }

    #[test]
    fn test_unterminated_string() {
        let tokens = kinds(r#""unterminated"#);
        assert!(matches!(&tokens[0], TokenKind::Error(_)));
    }

    #[test]
    fn test_keywords_uppercase() {
        assert_eq!(
            kinds("AND OR NOT"),
            vec![
                TokenKind::And,
                TokenKind::Or,
                TokenKind::Not,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_lowercase_and_or_not_are_identifiers() {
        assert_eq!(
            kinds("and or not"),
            vec![
                TokenKind::Identifier("and".to_string()),
                TokenKind::Identifier("or".to_string()),
                TokenKind::Identifier("not".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_as_by_case_insensitive() {
        assert_eq!(
            kinds("as AS By BY by"),
            vec![
                TokenKind::As,
                TokenKind::As,
                TokenKind::By,
                TokenKind::By,
                TokenKind::By,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_true_false() {
        assert_eq!(
            kinds("true false TRUE FALSE True False"),
            vec![
                TokenKind::True,
                TokenKind::False,
                TokenKind::True,
                TokenKind::False,
                TokenKind::True,
                TokenKind::False,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_identifier() {
        assert_eq!(
            kinds("status"),
            vec![TokenKind::Identifier("status".to_string()), TokenKind::Eof]
        );
    }

    #[test]
    fn test_dotted_field() {
        assert_eq!(
            kinds("src_ip.country"),
            vec![
                TokenKind::Identifier("src_ip.country".to_string()),
                TokenKind::Eof
            ]
        );
    }

    #[test]
    fn test_operators() {
        assert_eq!(
            kinds("= != < <= > >= | ! + - * / %"),
            vec![
                TokenKind::Eq,
                TokenKind::NotEq,
                TokenKind::Lt,
                TokenKind::LtEq,
                TokenKind::Gt,
                TokenKind::GtEq,
                TokenKind::Pipe,
                TokenKind::Bang,
                TokenKind::Plus,
                TokenKind::Minus,
                TokenKind::Star,
                TokenKind::Slash,
                TokenKind::Percent,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_delimiters() {
        assert_eq!(
            kinds("( ) [ ] , ."),
            vec![
                TokenKind::LParen,
                TokenKind::RParen,
                TokenKind::LBracket,
                TokenKind::RBracket,
                TokenKind::Comma,
                TokenKind::Dot,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_colon() {
        assert_eq!(kinds(":"), vec![TokenKind::Colon, TokenKind::Eof]);
    }

    #[test]
    fn test_dotdot() {
        assert_eq!(
            kinds("1..10"),
            vec![
                TokenKind::Integer(1),
                TokenKind::DotDot,
                TokenKind::Integer(10),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_wildcard_prefix() {
        assert_eq!(
            kinds("*login*"),
            vec![TokenKind::Wildcard("*login*".to_string()), TokenKind::Eof]
        );
    }

    #[test]
    fn test_wildcard_suffix() {
        assert_eq!(
            kinds("error*"),
            vec![TokenKind::Wildcard("error*".to_string()), TokenKind::Eof]
        );
    }

    #[test]
    fn test_star_alone() {
        assert_eq!(kinds("*"), vec![TokenKind::Star, TokenKind::Eof]);
    }

    #[test]
    fn test_pipeline_query() {
        let tokens = kinds(r#"status=200 | stats count by src_ip"#);
        assert_eq!(
            tokens,
            vec![
                TokenKind::Identifier("status".to_string()),
                TokenKind::Eq,
                TokenKind::Integer(200),
                TokenKind::Pipe,
                TokenKind::Identifier("stats".to_string()),
                TokenKind::Identifier("count".to_string()),
                TokenKind::By,
                TokenKind::Identifier("src_ip".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_complex_query() {
        let tokens = kinds(r#"src="client" dest="server" | stats count by src"#);
        assert_eq!(
            tokens,
            vec![
                TokenKind::Identifier("src".to_string()),
                TokenKind::Eq,
                TokenKind::StringLiteral("client".to_string()),
                TokenKind::Identifier("dest".to_string()),
                TokenKind::Eq,
                TokenKind::StringLiteral("server".to_string()),
                TokenKind::Pipe,
                TokenKind::Identifier("stats".to_string()),
                TokenKind::Identifier("count".to_string()),
                TokenKind::By,
                TokenKind::Identifier("src".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_logical_expression() {
        let tokens = kinds("status=404 AND (method=GET OR method=POST)");
        assert_eq!(
            tokens,
            vec![
                TokenKind::Identifier("status".to_string()),
                TokenKind::Eq,
                TokenKind::Integer(404),
                TokenKind::And,
                TokenKind::LParen,
                TokenKind::Identifier("method".to_string()),
                TokenKind::Eq,
                TokenKind::Identifier("GET".to_string()),
                TokenKind::Or,
                TokenKind::Identifier("method".to_string()),
                TokenKind::Eq,
                TokenKind::Identifier("POST".to_string()),
                TokenKind::RParen,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_span_positions() {
        let tokens = tokenize("foo = 42");
        assert_eq!(tokens[0].span, Span::new(0, 3)); // foo
        assert_eq!(tokens[1].span, Span::new(4, 5)); // =
        assert_eq!(tokens[2].span, Span::new(6, 8)); // 42
    }

    #[test]
    fn test_multibyte_error_is_single_token() {
        let tokens = tokenize("これはテスト");
        assert_eq!(tokens.len(), 2); // Error + Eof
        assert!(matches!(&tokens[0].kind, TokenKind::Error(msg) if msg.contains("これはテスト")));
        assert_eq!(tokens[0].span, Span::new(0, 18));
    }

    #[test]
    fn test_multibyte_before_pipe() {
        let tokens = tokenize("日本語 | stats count");
        assert!(matches!(&tokens[0].kind, TokenKind::Error(msg) if msg.contains("日本語")));
        assert_eq!(tokens[1].kind, TokenKind::Pipe);
        assert_eq!(tokens[2].kind, TokenKind::Identifier("stats".to_string()));
    }

    #[test]
    fn test_multibyte_mixed_with_ascii() {
        let tokens = tokenize("あ=い");
        assert!(matches!(&tokens[0].kind, TokenKind::Error(_)));
        assert_eq!(tokens[1].kind, TokenKind::Eq);
        assert!(matches!(&tokens[2].kind, TokenKind::Error(_)));
    }

    #[test]
    fn test_numeric_comparison() {
        let tokens = kinds("status >= 400");
        assert_eq!(
            tokens,
            vec![
                TokenKind::Identifier("status".to_string()),
                TokenKind::GtEq,
                TokenKind::Integer(400),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_subsearch_brackets() {
        let tokens = kinds("[search src=10.0.0.1]");
        assert_eq!(
            tokens,
            vec![
                TokenKind::LBracket,
                TokenKind::Identifier("search".to_string()),
                TokenKind::Identifier("src".to_string()),
                TokenKind::Eq,
                TokenKind::Float(10.0),
                TokenKind::Dot,
                TokenKind::Float(0.1),
                TokenKind::RBracket,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_eval_expression() {
        let tokens = kinds("eval total = count * 2 + offset");
        assert_eq!(
            tokens,
            vec![
                TokenKind::Identifier("eval".to_string()),
                TokenKind::Identifier("total".to_string()),
                TokenKind::Eq,
                TokenKind::Identifier("count".to_string()),
                TokenKind::Star,
                TokenKind::Integer(2),
                TokenKind::Plus,
                TokenKind::Identifier("offset".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_function_call() {
        let tokens = kinds("stats count(eval(status>200)) by host");
        assert_eq!(
            tokens,
            vec![
                TokenKind::Identifier("stats".to_string()),
                TokenKind::Identifier("count".to_string()),
                TokenKind::LParen,
                TokenKind::Identifier("eval".to_string()),
                TokenKind::LParen,
                TokenKind::Identifier("status".to_string()),
                TokenKind::Gt,
                TokenKind::Integer(200),
                TokenKind::RParen,
                TokenKind::RParen,
                TokenKind::By,
                TokenKind::Identifier("host".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_multiple_backtick_macros() {
        assert_eq!(
            kinds("`comment1` foo `comment2` bar"),
            vec![
                TokenKind::BacktickMacro("comment1".to_string()),
                TokenKind::Identifier("foo".to_string()),
                TokenKind::BacktickMacro("comment2".to_string()),
                TokenKind::Identifier("bar".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_backtick_macro_with_special_chars() {
        assert_eq!(
            kinds("`this has | pipes and = equals`"),
            vec![
                TokenKind::BacktickMacro("this has | pipes and = equals".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_rename_as() {
        let tokens = kinds("rename src AS source");
        assert_eq!(
            tokens,
            vec![
                TokenKind::Identifier("rename".to_string()),
                TokenKind::Identifier("src".to_string()),
                TokenKind::As,
                TokenKind::Identifier("source".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_stats_by() {
        let tokens = kinds("stats count by host, src_ip");
        assert_eq!(
            tokens,
            vec![
                TokenKind::Identifier("stats".to_string()),
                TokenKind::Identifier("count".to_string()),
                TokenKind::By,
                TokenKind::Identifier("host".to_string()),
                TokenKind::Comma,
                TokenKind::Identifier("src_ip".to_string()),
                TokenKind::Eof,
            ]
        );
    }
}
