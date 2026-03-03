pub mod token;

use token::{Span, Token, TokenKind};

/// SPL2 の字句解析器です。
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
        self.skip_whitespace();

        if self.pos >= self.bytes.len() {
            return Token::new(TokenKind::Eof, Span::new(self.pos, self.pos));
        }

        let start = self.pos;
        let ch = self.bytes[self.pos];

        match ch {
            b'"' => self.lex_string_literal(start),
            b'\'' => self.lex_single_quoted_field(start),
            b'@' => {
                // @"..." Raw 文字列リテラル
                if self.pos + 1 < self.bytes.len() && self.bytes[self.pos + 1] == b'"' {
                    self.lex_raw_string(start)
                } else {
                    self.lex_identifier_or_wildcard(start)
                }
            }
            b'`' => self.lex_search_literal(start),
            b'$' => self.lex_dollar(start),
            b'0'..=b'9' => self.lex_number(start),
            b'|' => self.single_char_token(TokenKind::Pipe, start),
            b'(' => self.single_char_token(TokenKind::LParen, start),
            b')' => self.single_char_token(TokenKind::RParen, start),
            b'[' => self.single_char_token(TokenKind::LBracket, start),
            b']' => self.single_char_token(TokenKind::RBracket, start),
            b'{' => self.single_char_token(TokenKind::LBrace, start),
            b'}' => self.single_char_token(TokenKind::RBrace, start),
            b',' => self.single_char_token(TokenKind::Comma, start),
            b'+' => self.single_char_token(TokenKind::Plus, start),
            b'-' => self.lex_minus_or_arrow(start),
            b'/' => self.lex_slash(start),
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

    fn skip_whitespace(&mut self) {
        while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
        // 行コメント (// ...) をスキップします
        if self.pos + 1 < self.bytes.len()
            && self.bytes[self.pos] == b'/'
            && self.bytes[self.pos + 1] == b'/'
        {
            while self.pos < self.bytes.len() && self.bytes[self.pos] != b'\n' {
                self.pos += 1;
            }
            self.skip_whitespace();
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

    fn lex_raw_string(&mut self, start: usize) -> Token {
        self.pos += 2; // @" をスキップします
        let mut value = String::new();

        while self.pos < self.bytes.len() {
            if self.bytes[self.pos] == b'"' {
                // "" はエスケープされた " です
                if self.pos + 1 < self.bytes.len() && self.bytes[self.pos + 1] == b'"' {
                    value.push('"');
                    self.pos += 2;
                } else {
                    self.pos += 1;
                    return Token::new(TokenKind::RawString(value), Span::new(start, self.pos));
                }
            } else {
                value.push(self.bytes[self.pos] as char);
                self.pos += 1;
            }
        }

        Token::new(
            TokenKind::Error("unterminated raw string literal".to_string()),
            Span::new(start, self.pos),
        )
    }

    fn lex_single_quoted_field(&mut self, start: usize) -> Token {
        self.pos += 1; // 開始の ' をスキップします
        let mut value = String::new();

        while self.pos < self.bytes.len() {
            match self.bytes[self.pos] {
                b'\'' => {
                    self.pos += 1;
                    return Token::new(
                        TokenKind::SingleQuotedField(value),
                        Span::new(start, self.pos),
                    );
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
            TokenKind::Error("unterminated single-quoted field name".to_string()),
            Span::new(start, self.pos),
        )
    }

    fn lex_search_literal(&mut self, start: usize) -> Token {
        self.pos += 1; // 開始の ` をスキップします
        let content_start = self.pos;

        while self.pos < self.bytes.len() && self.bytes[self.pos] != b'`' {
            self.pos += 1;
        }

        let content = self.source[content_start..self.pos].to_string();

        if self.pos < self.bytes.len() {
            self.pos += 1; // 閉じの ` をスキップします
        }

        Token::new(
            TokenKind::SearchLiteral(content),
            Span::new(start, self.pos),
        )
    }

    fn lex_dollar(&mut self, start: usize) -> Token {
        self.pos += 1; // $ をスキップします
        Token::new(TokenKind::Dollar, Span::new(start, self.pos))
    }

    fn lex_number(&mut self, start: usize) -> Token {
        while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_digit() {
            self.pos += 1;
        }

        // 小数点があるか確認します
        if self.pos < self.bytes.len() && self.bytes[self.pos] == b'.' {
            if self.pos + 1 < self.bytes.len() && self.bytes[self.pos + 1] == b'.' {
                let text = &self.source[start..self.pos];
                let value = text.parse::<i64>().unwrap_or(0);
                return Token::new(TokenKind::Integer(value), Span::new(start, self.pos));
            }
            if self.pos + 1 < self.bytes.len() && self.bytes[self.pos + 1].is_ascii_digit() {
                self.pos += 1;
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
        self.pos += 1;

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

    fn lex_minus_or_arrow(&mut self, start: usize) -> Token {
        self.pos += 1;
        if self.pos < self.bytes.len() && self.bytes[self.pos] == b'>' {
            self.pos += 1;
            return Token::new(TokenKind::Arrow, Span::new(start, self.pos));
        }
        Token::new(TokenKind::Minus, Span::new(start, self.pos))
    }

    fn lex_slash(&mut self, start: usize) -> Token {
        self.pos += 1;
        // // コメントは skip_whitespace で処理済みなので、ここでは単純に Slash を返します
        Token::new(TokenKind::Slash, Span::new(start, self.pos))
    }

    fn lex_identifier_or_wildcard(&mut self, start: usize) -> Token {
        self.pos += 1;
        while self.pos < self.bytes.len() && is_ident_continue(self.bytes[self.pos]) {
            self.pos += 1;
        }

        // ドット区切りのフィールド名に対応します (例: src_ip.country)
        loop {
            if self.pos < self.bytes.len() && self.bytes[self.pos] == b'.' {
                if self.pos + 1 < self.bytes.len() && self.bytes[self.pos + 1] == b'.' {
                    break;
                }
                if self.pos + 1 < self.bytes.len() && is_ident_start(self.bytes[self.pos + 1]) {
                    self.pos += 1;
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
            while self.pos < self.bytes.len()
                && (is_ident_continue(self.bytes[self.pos]) || self.bytes[self.pos] == b'*')
            {
                self.pos += 1;
            }
            let text = self.source[start..self.pos].to_string();
            return Token::new(TokenKind::Wildcard(text), Span::new(start, self.pos));
        }

        let text = &self.source[start..self.pos];
        let kind = classify_keyword(text);
        Token::new(kind, Span::new(start, self.pos))
    }
}

/// 識別子テキストをキーワードまたは識別子に分類します。
fn classify_keyword(text: &str) -> TokenKind {
    // SPL2 では AND, OR, NOT, XOR は大文字小文字を区別しません
    let lower = text.to_ascii_lowercase();
    match lower.as_str() {
        "and" => TokenKind::And,
        "or" => TokenKind::Or,
        "not" => TokenKind::Not,
        "xor" => TokenKind::Xor,
        "as" => TokenKind::As,
        "by" => TokenKind::By,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "null" => TokenKind::Null,
        "from" => TokenKind::From,
        "select" => TokenKind::Select,
        "where" => TokenKind::Where,
        "having" => TokenKind::Having,
        "limit" => TokenKind::Limit,
        "offset" => TokenKind::Offset,
        "join" => TokenKind::Join,
        "inner" => TokenKind::Inner,
        "left" => TokenKind::Left,
        "outer" => TokenKind::Outer,
        "on" => TokenKind::On,
        "asc" => TokenKind::Asc,
        "desc" => TokenKind::Desc,
        "distinct" => TokenKind::Distinct,
        "in" => TokenKind::In,
        "is" => TokenKind::Is,
        "like" => TokenKind::Like,
        "between" => TokenKind::Between,
        "exists" => TokenKind::Exists,
        "into" => TokenKind::Into,
        "union" => TokenKind::Union,
        _ => TokenKind::Identifier(text.to_string()),
    }
}

/// "GROUP BY" と "ORDER BY" の 2 語キーワードを検出するために使用します。
/// Parser 側で処理します。
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
    fn test_null_literal() {
        assert_eq!(kinds("null"), vec![TokenKind::Null, TokenKind::Eof]);
    }

    #[test]
    fn test_null_case_insensitive() {
        assert_eq!(kinds("NULL"), vec![TokenKind::Null, TokenKind::Eof]);
    }

    #[test]
    fn test_xor_keyword() {
        assert_eq!(
            kinds("a XOR b"),
            vec![
                TokenKind::Identifier("a".to_string()),
                TokenKind::Xor,
                TokenKind::Identifier("b".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_arrow_token() {
        assert_eq!(
            kinds("$x -> $x + 1"),
            vec![
                TokenKind::Dollar,
                TokenKind::Identifier("x".to_string()),
                TokenKind::Arrow,
                TokenKind::Dollar,
                TokenKind::Identifier("x".to_string()),
                TokenKind::Plus,
                TokenKind::Integer(1),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_braces() {
        assert_eq!(
            kinds("{}"),
            vec![TokenKind::LBrace, TokenKind::RBrace, TokenKind::Eof]
        );
    }

    #[test]
    fn test_raw_string() {
        assert_eq!(
            kinds(r#"@"hello\nworld""#),
            vec![
                TokenKind::RawString("hello\\nworld".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_raw_string_with_escaped_quote() {
        assert_eq!(
            kinds(r#"@"say ""hi""  ""#),
            vec![
                TokenKind::RawString("say \"hi\"  ".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_single_quoted_field() {
        assert_eq!(
            kinds("'my-field'"),
            vec![
                TokenKind::SingleQuotedField("my-field".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_search_literal() {
        assert_eq!(
            kinds("`status=200`"),
            vec![
                TokenKind::SearchLiteral("status=200".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_sql_keywords() {
        assert_eq!(
            kinds("FROM SELECT WHERE"),
            vec![
                TokenKind::From,
                TokenKind::Select,
                TokenKind::Where,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_sql_keywords_case_insensitive() {
        assert_eq!(
            kinds("from select where"),
            vec![
                TokenKind::From,
                TokenKind::Select,
                TokenKind::Where,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_and_or_not_case_insensitive() {
        assert_eq!(
            kinds("and OR Not"),
            vec![
                TokenKind::And,
                TokenKind::Or,
                TokenKind::Not,
                TokenKind::Eof
            ]
        );
    }

    #[test]
    fn test_join_keywords() {
        assert_eq!(
            kinds("JOIN INNER LEFT OUTER ON"),
            vec![
                TokenKind::Join,
                TokenKind::Inner,
                TokenKind::Left,
                TokenKind::Outer,
                TokenKind::On,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_pipeline_query() {
        let tokens = kinds("FROM main WHERE status=200 | stats count() BY host");
        assert_eq!(
            tokens,
            vec![
                TokenKind::From,
                TokenKind::Identifier("main".to_string()),
                TokenKind::Where,
                TokenKind::Identifier("status".to_string()),
                TokenKind::Eq,
                TokenKind::Integer(200),
                TokenKind::Pipe,
                TokenKind::Identifier("stats".to_string()),
                TokenKind::Identifier("count".to_string()),
                TokenKind::LParen,
                TokenKind::RParen,
                TokenKind::By,
                TokenKind::Identifier("host".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_line_comment() {
        assert_eq!(
            kinds("foo // this is a comment\nbar"),
            vec![
                TokenKind::Identifier("foo".to_string()),
                TokenKind::Identifier("bar".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_dollar_param() {
        assert_eq!(
            kinds("$var"),
            vec![
                TokenKind::Dollar,
                TokenKind::Identifier("var".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_dotted_field() {
        assert_eq!(
            kinds("src_ip.country"),
            vec![
                TokenKind::Identifier("src_ip.country".to_string()),
                TokenKind::Eof,
            ]
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
    fn test_in_between_is_like() {
        assert_eq!(
            kinds("IN BETWEEN IS LIKE"),
            vec![
                TokenKind::In,
                TokenKind::Between,
                TokenKind::Is,
                TokenKind::Like,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_distinct_asc_desc() {
        assert_eq!(
            kinds("DISTINCT ASC DESC"),
            vec![
                TokenKind::Distinct,
                TokenKind::Asc,
                TokenKind::Desc,
                TokenKind::Eof,
            ]
        );
    }
}
