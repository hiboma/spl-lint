pub mod ast;

use crate::lexer::token::{Span, Token, TokenKind};
use ast::*;

/// パースエラーを表します。
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ParseError {
    pub message: String,
    pub span: Span,
}

impl ParseError {
    fn new(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }
}

/// SPL の構文解析器です。
pub(crate) struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    errors: Vec<ParseError>,
    /// パイプの後のステージかどうかを追跡します
    after_pipe: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            errors: Vec::new(),
            after_pipe: false,
        }
    }

    /// クエリ文字列をパースして AST とエラーのリストを返します。
    pub fn parse(mut self) -> (Option<Query>, Vec<ParseError>) {
        let query = self.parse_query();
        (query, self.errors)
    }

    // ---- ユーティリティ ----

    fn peek(&self) -> &TokenKind {
        self.tokens
            .get(self.pos)
            .map(|t| &t.kind)
            .unwrap_or(&TokenKind::Eof)
    }

    fn current_span(&self) -> Span {
        self.tokens
            .get(self.pos)
            .map(|t| t.span)
            .unwrap_or_else(|| {
                self.tokens
                    .last()
                    .map(|t| Span::new(t.span.end, t.span.end))
                    .unwrap_or(Span::new(0, 0))
            })
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens.get(self.pos).cloned().unwrap_or(Token {
            kind: TokenKind::Eof,
            span: self.current_span(),
        });
        if tok.kind != TokenKind::Eof {
            self.pos += 1;
        }
        tok
    }

    fn expect(&mut self, expected: &TokenKind) -> Option<Token> {
        if self.peek() == expected {
            Some(self.advance())
        } else {
            let span = self.current_span();
            self.errors.push(ParseError::new(
                format!("expected {:?}, found {:?}", expected, self.peek()),
                span,
            ));
            None
        }
    }

    // ---- クエリパース ----

    fn parse_query(&mut self) -> Option<Query> {
        let start = self.current_span().start;
        let mut stages = Vec::new();

        // 空入力の処理
        if matches!(self.peek(), TokenKind::Eof) {
            return Some(Query {
                stages,
                span: Span::new(start, start),
            });
        }

        // 先頭が `|` の場合はスキップしてコマンドとして扱います
        // (例: `| makeresults | eval ...` のような generating command パターン)
        if matches!(self.peek(), TokenKind::Pipe) {
            self.advance(); // 先頭の `|` を消費します
            self.after_pipe = true;
        }

        // 最初のステージをパースします
        // サブサーチ内では after_pipe=true のまま維持し、コマンドとして判定します
        // トップレベルでは after_pipe=false でフリーテキスト検索として判定します
        if let Some(stage) = self.parse_stage() {
            stages.push(stage);
        }

        // パイプで区切られた後続のステージをパースします
        while matches!(self.peek(), TokenKind::Pipe) {
            self.advance(); // `|` を消費します
            self.after_pipe = true;
            if let Some(stage) = self.parse_stage() {
                stages.push(stage);
            } else {
                // エラーリカバリ: 次のパイプまでスキップします
                self.recover_to_pipe();
            }
        }

        // 閉じ括弧でない限り、EOF でない場合はエラーです
        if !matches!(self.peek(), TokenKind::Eof | TokenKind::RBracket) {
            let span = self.current_span();
            self.errors.push(ParseError::new(
                format!("unexpected token: {:?}", self.peek()),
                span,
            ));
        }

        let end = self
            .tokens
            .get(self.pos.saturating_sub(1))
            .map(|t| t.span.end)
            .unwrap_or(start);

        Some(Query {
            stages,
            span: Span::new(start, end),
        })
    }

    fn recover_to_pipe(&mut self) {
        while !matches!(
            self.peek(),
            TokenKind::Pipe | TokenKind::Eof | TokenKind::RBracket
        ) {
            self.advance();
        }
    }

    fn parse_stage(&mut self) -> Option<PipelineStage> {
        let start = self.current_span().start;

        // パイプの後のバッククォートマクロはマクロ呼び出しステージとして扱います
        if self.after_pipe {
            if let TokenKind::BacktickMacro(ref content) = self.peek().clone() {
                let content = content.clone();
                let tok = self.advance();
                return Some(PipelineStage {
                    kind: StageKind::MacroCall(content),
                    span: Span::new(start, tok.span.end),
                });
            }
        }

        // コマンドかどうか判定します
        if let TokenKind::Identifier(name) = self.peek().clone() {
            // search コマンドの場合は search 式として扱います
            if name.eq_ignore_ascii_case("search") {
                return self.parse_search_stage(start);
            }

            // パイプの後は常にコマンドとして扱います (比較演算子が続く場合を除く)
            if self.after_pipe {
                if self.is_command_start() {
                    return self.parse_command_stage(start);
                }
            } else {
                // 最初のステージ: 比較演算子が続かない場合のみコマンドとして扱います
                // ただし、search 式として解釈可能な場合もあります
                // 比較演算子が続く場合はフィールドフィルタです
                if self.is_comparison_next() {
                    // フィールドフィルタとして扱います
                } else {
                    // 比較演算子が続かない場合でも、最初のステージでは
                    // フリーテキスト検索かコマンドか曖昧です。
                    // search 式としてパースします。
                }
            }
        }

        // search 式として扱います
        self.parse_search_expression_stage(start)
    }

    /// コマンドの開始かどうかを先読みで判定します。
    /// 識別子の後に `=` が来る場合はフィールドフィルタ (search 式) です。
    fn is_command_start(&self) -> bool {
        if !matches!(self.peek(), TokenKind::Identifier(_)) {
            return false;
        }

        let next = self
            .tokens
            .get(self.pos + 1)
            .map(|t| &t.kind)
            .unwrap_or(&TokenKind::Eof);

        // `ident=`, `ident!=`, `ident>`, `ident<` パターンはフィールドフィルタです
        !matches!(
            next,
            TokenKind::Eq
                | TokenKind::NotEq
                | TokenKind::Lt
                | TokenKind::LtEq
                | TokenKind::Gt
                | TokenKind::GtEq
        )
    }

    fn parse_search_stage(&mut self, start: usize) -> Option<PipelineStage> {
        self.advance(); // `search` を消費します

        if let Some(expr) = self.parse_search_or() {
            let end = self.current_span().start;
            Some(PipelineStage {
                kind: StageKind::Search(expr),
                span: Span::new(start, end),
            })
        } else {
            None
        }
    }

    fn parse_search_expression_stage(&mut self, start: usize) -> Option<PipelineStage> {
        if let Some(expr) = self.parse_search_or() {
            let end = self.current_span().start;
            Some(PipelineStage {
                kind: StageKind::Search(expr),
                span: Span::new(start, end),
            })
        } else {
            None
        }
    }

    // ---- Search 式パース ----

    fn parse_search_or(&mut self) -> Option<SearchExpr> {
        let mut left = self.parse_search_and()?;

        while matches!(self.peek(), TokenKind::Or) {
            self.advance(); // OR を消費します
            let right = self.parse_search_and()?;
            left = SearchExpr::Or(Box::new(left), Box::new(right));
        }

        Some(left)
    }

    fn parse_search_and(&mut self) -> Option<SearchExpr> {
        let mut left = self.parse_search_not()?;

        loop {
            if matches!(self.peek(), TokenKind::And) {
                self.advance(); // AND を消費します
                let right = self.parse_search_not()?;
                left = SearchExpr::And(Box::new(left), Box::new(right));
            } else if self.is_implicit_and() {
                let right = self.parse_search_not()?;
                left = SearchExpr::And(Box::new(left), Box::new(right));
            } else {
                break;
            }
        }

        Some(left)
    }

    fn parse_search_not(&mut self) -> Option<SearchExpr> {
        if matches!(self.peek(), TokenKind::Not) {
            self.advance(); // NOT を消費します
            let expr = self.parse_search_primary()?;
            return Some(SearchExpr::Not(Box::new(expr)));
        }
        self.parse_search_primary()
    }

    fn parse_search_primary(&mut self) -> Option<SearchExpr> {
        match self.peek().clone() {
            TokenKind::LParen => {
                self.advance(); // `(` を消費します
                let expr = self.parse_search_or()?;
                self.expect(&TokenKind::RParen);
                Some(SearchExpr::Grouped(Box::new(expr)))
            }
            TokenKind::LBracket => {
                // サブサーチ [search ...] / [eval ...] 等
                self.advance(); // `[` を消費します
                let saved = self.after_pipe;
                self.after_pipe = true;
                let query = self.parse_query()?;
                self.after_pipe = saved;
                self.expect(&TokenKind::RBracket);
                Some(SearchExpr::SubSearch(Box::new(query)))
            }
            TokenKind::Star => {
                self.advance();
                Some(SearchExpr::FreeText("*".to_string()))
            }
            TokenKind::Wildcard(ref text) => {
                let text = text.clone();
                self.advance();
                Some(SearchExpr::Wildcard(text))
            }
            TokenKind::StringLiteral(ref text) => {
                let text = text.clone();
                if self.is_comparison_next() {
                    // 文字列リテラルをフィールド名として扱います
                    // (例: "metadata.eventType"="RemoteResponseSessionStartEvent")
                    self.parse_field_filter(text)
                } else {
                    self.advance();
                    Some(SearchExpr::FreeText(text))
                }
            }
            TokenKind::Identifier(ref name) => {
                let name = name.clone();
                if self.is_comparison_next() {
                    self.parse_field_filter(name)
                } else if self.is_in_next() {
                    // field IN ("a", "b", ...) パターン
                    self.advance(); // フィールド名を消費します
                    self.advance(); // IN を消費します
                    self.skip_in_value_list();
                    Some(SearchExpr::FreeText(name))
                } else {
                    self.advance();
                    Some(SearchExpr::FreeText(name))
                }
            }
            TokenKind::Integer(_) | TokenKind::Float(_) => {
                let tok = self.advance();
                let text = match &tok.kind {
                    TokenKind::Integer(n) => n.to_string(),
                    TokenKind::Float(n) => n.to_string(),
                    _ => unreachable!(),
                };
                Some(SearchExpr::FreeText(text))
            }
            TokenKind::BacktickMacro(ref content) => {
                let content = content.clone();
                self.advance();
                Some(SearchExpr::FreeText(content))
            }
            TokenKind::TemplateVar(ref name) => {
                let name = name.clone();
                self.advance();
                Some(SearchExpr::FreeText(name))
            }
            TokenKind::Error(ref msg) => {
                let msg = msg.clone();
                let span = self.current_span();
                self.advance();
                self.errors
                    .push(ParseError::new(format!("lexer error: {}", msg), span));
                None
            }
            _ => {
                let span = self.current_span();
                self.errors.push(ParseError::new(
                    format!("unexpected token in search expression: {:?}", self.peek()),
                    span,
                ));
                None
            }
        }
    }

    fn is_comparison_next(&self) -> bool {
        let next = self
            .tokens
            .get(self.pos + 1)
            .map(|t| &t.kind)
            .unwrap_or(&TokenKind::Eof);

        matches!(
            next,
            TokenKind::Eq
                | TokenKind::NotEq
                | TokenKind::Lt
                | TokenKind::LtEq
                | TokenKind::Gt
                | TokenKind::GtEq
        )
    }

    /// `pos + 1` が `IN` (大文字小文字不問) かどうか判定します。
    fn is_in_next(&self) -> bool {
        matches!(
            self.tokens.get(self.pos + 1).map(|t| &t.kind),
            Some(TokenKind::Identifier(name)) if name.eq_ignore_ascii_case("IN")
        )
    }

    /// `IN (value1, value2, ...)` の値リストをスキップします。
    fn skip_in_value_list(&mut self) {
        if matches!(self.peek(), TokenKind::LParen) {
            self.advance(); // `(` を消費します
            while !matches!(self.peek(), TokenKind::RParen | TokenKind::Eof) {
                self.advance();
            }
            if matches!(self.peek(), TokenKind::RParen) {
                self.advance(); // `)` を消費します
            }
        }
    }

    fn parse_field_filter(&mut self, field: String) -> Option<SearchExpr> {
        self.advance(); // フィールド名を消費します
        let op_tok = self.advance();
        let op = match op_tok.kind {
            TokenKind::Eq => CompareOp::Eq,
            TokenKind::NotEq => CompareOp::NotEq,
            TokenKind::Lt => CompareOp::Lt,
            TokenKind::LtEq => CompareOp::LtEq,
            TokenKind::Gt => CompareOp::Gt,
            TokenKind::GtEq => CompareOp::GtEq,
            _ => {
                self.errors.push(ParseError::new(
                    format!("expected comparison operator, found {:?}", op_tok.kind),
                    op_tok.span,
                ));
                return None;
            }
        };

        let value = self.parse_filter_value()?;
        Some(SearchExpr::FieldFilter { field, op, value })
    }

    fn parse_filter_value(&mut self) -> Option<FilterValue> {
        match self.peek().clone() {
            TokenKind::StringLiteral(ref s) => {
                let s = s.clone();
                self.advance();
                Some(FilterValue::String(s))
            }
            TokenKind::Minus => {
                // 負の値 (例: earliest=-3h, -7d@d)
                self.advance();
                match self.peek().clone() {
                    TokenKind::Integer(n) => {
                        self.advance();
                        Some(FilterValue::Number(-(n as f64)))
                    }
                    TokenKind::Float(n) => {
                        self.advance();
                        Some(FilterValue::Number(-n))
                    }
                    TokenKind::Identifier(ref name) => {
                        let name = name.clone();
                        self.advance();
                        Some(FilterValue::Field(format!("-{}", name)))
                    }
                    _ => Some(FilterValue::Field("-".to_string())),
                }
            }
            TokenKind::Integer(n) => {
                self.advance();
                Some(FilterValue::Number(n as f64))
            }
            TokenKind::Float(n) => {
                self.advance();
                Some(FilterValue::Number(n))
            }
            TokenKind::Wildcard(ref w) => {
                let w = w.clone();
                self.advance();
                Some(FilterValue::Wildcard(w))
            }
            TokenKind::Star => {
                self.advance();
                Some(FilterValue::Wildcard("*".to_string()))
            }
            TokenKind::True => {
                self.advance();
                Some(FilterValue::Bool(true))
            }
            TokenKind::False => {
                self.advance();
                Some(FilterValue::Bool(false))
            }
            TokenKind::Identifier(ref name) => {
                let name = name.clone();
                self.advance();
                Some(FilterValue::Field(name))
            }
            TokenKind::TemplateVar(ref name) => {
                let name = name.clone();
                self.advance();
                Some(FilterValue::Field(name))
            }
            _ => {
                let span = self.current_span();
                self.errors.push(ParseError::new(
                    format!("expected value, found {:?}", self.peek()),
                    span,
                ));
                None
            }
        }
    }

    /// 暗黙の AND: パイプ、EOF、閉じ括弧、閉じ角括弧、OR の場合は暗黙 AND ではありません
    fn is_implicit_and(&self) -> bool {
        !matches!(
            self.peek(),
            TokenKind::Pipe
                | TokenKind::Eof
                | TokenKind::RParen
                | TokenKind::RBracket
                | TokenKind::Or
                | TokenKind::And
                | TokenKind::Error(_)
        ) && self.can_start_search_primary()
    }

    fn can_start_search_primary(&self) -> bool {
        matches!(
            self.peek(),
            TokenKind::Identifier(_)
                | TokenKind::StringLiteral(_)
                | TokenKind::Integer(_)
                | TokenKind::Float(_)
                | TokenKind::Star
                | TokenKind::Wildcard(_)
                | TokenKind::LParen
                | TokenKind::LBracket
                | TokenKind::Not
                | TokenKind::BacktickMacro(_)
                | TokenKind::TemplateVar(_)
        )
    }

    // ---- コマンドパース ----

    fn parse_command_stage(&mut self, start: usize) -> Option<PipelineStage> {
        let name_tok = self.advance();
        let name = match &name_tok.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => {
                self.errors.push(ParseError::new(
                    format!("expected command name, found {:?}", name_tok.kind),
                    name_tok.span,
                ));
                return None;
            }
        };

        let mut arguments = Vec::new();
        let mut by_clause = None;
        let mut as_clause = None;

        // コマンド引数をパースします
        loop {
            // パイプ、EOF、閉じ角括弧で終了します
            if matches!(
                self.peek(),
                TokenKind::Pipe | TokenKind::Eof | TokenKind::RBracket
            ) {
                break;
            }

            // by 節
            if matches!(self.peek(), TokenKind::By) {
                self.advance(); // by を消費します
                by_clause = Some(self.parse_field_list());
                continue;
            }

            // as 節
            if matches!(self.peek(), TokenKind::As) {
                self.advance(); // as を消費します
                if let TokenKind::Identifier(ref alias) = self.peek().clone() {
                    let alias = alias.clone();
                    self.advance();
                    as_clause = Some(alias);
                }
                continue;
            }

            // カンマをスキップします (table host, src_ip 形式)
            if matches!(self.peek(), TokenKind::Comma) {
                self.advance();
                continue;
            }

            // バッククォートマクロはスキップします
            if matches!(self.peek(), TokenKind::BacktickMacro(_)) {
                self.advance();
                continue;
            }

            // 名前付き引数 (name=value) または位置引数をパースします
            if let Some(arg) = self.parse_command_arg() {
                arguments.push(arg);
            } else {
                // エラーリカバリ: パイプまでスキップします
                break;
            }
        }

        let end = self.current_span().start;

        Some(PipelineStage {
            kind: StageKind::Command(Command {
                name,
                arguments,
                by_clause,
                as_clause,
                span: Span::new(start, end),
            }),
            span: Span::new(start, end),
        })
    }

    fn parse_field_list(&mut self) -> Vec<String> {
        let mut fields = Vec::new();
        while let TokenKind::Identifier(ref name) = self.peek().clone() {
            let name = name.clone();
            self.advance();
            fields.push(name);
            if matches!(self.peek(), TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        fields
    }

    fn parse_command_arg(&mut self) -> Option<CommandArg> {
        // 名前付き引数の判定: `ident=`
        if let TokenKind::Identifier(ref name) = self.peek().clone() {
            let next = self
                .tokens
                .get(self.pos + 1)
                .map(|t| &t.kind)
                .unwrap_or(&TokenKind::Eof);
            if matches!(next, TokenKind::Eq) {
                let name = name.clone();
                self.advance(); // 名前を消費します
                self.advance(); // `=` を消費します
                let value = self.parse_expr()?;
                return Some(CommandArg::Named { name, value });
            }
        }

        // 位置引数
        let expr = self.parse_expr()?;
        Some(CommandArg::Positional(expr))
    }

    // ---- 式パース ----

    fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Option<Expr> {
        let left = self.parse_additive()?;

        // 比較演算子がある場合は CompareExpr を生成します
        if matches!(
            self.peek(),
            TokenKind::Eq
                | TokenKind::NotEq
                | TokenKind::Lt
                | TokenKind::LtEq
                | TokenKind::Gt
                | TokenKind::GtEq
        ) {
            let op = match self.advance().kind {
                TokenKind::Eq => CompareOp::Eq,
                TokenKind::NotEq => CompareOp::NotEq,
                TokenKind::Lt => CompareOp::Lt,
                TokenKind::LtEq => CompareOp::LtEq,
                TokenKind::Gt => CompareOp::Gt,
                TokenKind::GtEq => CompareOp::GtEq,
                _ => unreachable!(),
            };
            let right = self.parse_additive()?;
            return Some(Expr::CompareExpr {
                left: Box::new(left),
                op,
                right: Box::new(right),
            });
        }

        Some(left)
    }

    fn parse_additive(&mut self) -> Option<Expr> {
        let mut left = self.parse_multiplicative()?;

        while matches!(self.peek(), TokenKind::Plus | TokenKind::Minus) {
            let op = match self.advance().kind {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_multiplicative()?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Some(left)
    }

    fn parse_multiplicative(&mut self) -> Option<Expr> {
        let mut left = self.parse_unary()?;

        while matches!(
            self.peek(),
            TokenKind::Star | TokenKind::Slash | TokenKind::Percent | TokenKind::Dot
        ) {
            let op = match self.advance().kind {
                TokenKind::Star => BinaryOp::Mul,
                TokenKind::Slash => BinaryOp::Div,
                TokenKind::Percent => BinaryOp::Mod,
                TokenKind::Dot => BinaryOp::Concat,
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Some(left)
    }

    fn parse_unary(&mut self) -> Option<Expr> {
        match self.peek() {
            TokenKind::Minus => {
                self.advance();
                let operand = self.parse_primary()?;
                Some(Expr::UnaryOp {
                    op: UnaryOp::Neg,
                    operand: Box::new(operand),
                })
            }
            TokenKind::Bang | TokenKind::Not => {
                self.advance();
                let operand = self.parse_primary()?;
                Some(Expr::UnaryOp {
                    op: UnaryOp::Not,
                    operand: Box::new(operand),
                })
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        match self.peek().clone() {
            TokenKind::Integer(n) => {
                self.advance();
                Some(Expr::Number(n as f64))
            }
            TokenKind::Float(n) => {
                self.advance();
                Some(Expr::Number(n))
            }
            TokenKind::StringLiteral(ref s) => {
                let s = s.clone();
                self.advance();
                Some(Expr::String(s))
            }
            TokenKind::True => {
                self.advance();
                Some(Expr::Bool(true))
            }
            TokenKind::False => {
                self.advance();
                Some(Expr::Bool(false))
            }
            TokenKind::Star => {
                self.advance();
                Some(Expr::Wildcard("*".to_string()))
            }
            TokenKind::Wildcard(ref w) => {
                let w = w.clone();
                self.advance();
                Some(Expr::Wildcard(w))
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(&TokenKind::RParen);
                Some(expr)
            }
            TokenKind::LBracket => {
                // サブサーチ
                self.advance();
                let saved = self.after_pipe;
                self.after_pipe = true;
                let query = self.parse_query()?;
                self.after_pipe = saved;
                self.expect(&TokenKind::RBracket);
                Some(Expr::SubSearch(Box::new(query)))
            }
            TokenKind::Identifier(ref name) => {
                let name = name.clone();
                // 関数呼び出しの判定: `ident(`
                if self.tokens.get(self.pos + 1).map(|t| &t.kind) == Some(&TokenKind::LParen) {
                    self.parse_function_call(name)
                } else {
                    self.advance();
                    Some(Expr::Field(name))
                }
            }
            TokenKind::TemplateVar(ref name) => {
                let name = name.clone();
                self.advance();
                Some(Expr::Field(name))
            }
            TokenKind::BacktickMacro(ref content) => {
                let content = content.clone();
                self.advance();
                Some(Expr::Field(content))
            }
            // コマンド引数内でキーワードがフィールド名として使われる場合
            TokenKind::By => {
                self.advance();
                Some(Expr::Field("by".to_string()))
            }
            TokenKind::As => {
                self.advance();
                Some(Expr::Field("as".to_string()))
            }
            TokenKind::And => {
                self.advance();
                Some(Expr::Field("AND".to_string()))
            }
            TokenKind::Or => {
                self.advance();
                Some(Expr::Field("OR".to_string()))
            }
            TokenKind::Error(ref msg) => {
                let msg = msg.clone();
                let span = self.current_span();
                self.advance();
                self.errors
                    .push(ParseError::new(format!("lexer error: {}", msg), span));
                None
            }
            _ => {
                let span = self.current_span();
                self.errors.push(ParseError::new(
                    format!("unexpected token in expression: {:?}", self.peek()),
                    span,
                ));
                None
            }
        }
    }

    fn parse_function_call(&mut self, name: String) -> Option<Expr> {
        let start = self.current_span().start;
        self.advance(); // 関数名を消費します
        self.advance(); // `(` を消費します

        let mut arguments = Vec::new();

        while !matches!(self.peek(), TokenKind::RParen | TokenKind::Eof) {
            // 名前付き引数の判定: `ident=`
            if let TokenKind::Identifier(ref arg_name) = self.peek().clone() {
                let next = self
                    .tokens
                    .get(self.pos + 1)
                    .map(|t| &t.kind)
                    .unwrap_or(&TokenKind::Eof);
                if matches!(next, TokenKind::Eq) {
                    let arg_name = arg_name.clone();
                    self.advance(); // 名前を消費します
                    self.advance(); // `=` を消費します
                    let value = self.parse_expr()?;
                    arguments.push(FunctionArg::Named {
                        name: arg_name,
                        value,
                    });
                    if matches!(self.peek(), TokenKind::Comma) {
                        self.advance();
                    }
                    continue;
                }
            }

            let expr = self.parse_expr()?;
            arguments.push(FunctionArg::Positional(expr));

            if matches!(self.peek(), TokenKind::Comma) {
                self.advance();
            }
        }

        let end_tok = self.expect(&TokenKind::RParen);
        let end = end_tok
            .map(|t| t.span.end)
            .unwrap_or(self.current_span().start);

        Some(Expr::FunctionCall(FunctionCall {
            name,
            arguments,
            span: Span::new(start, end),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse(input: &str) -> (Option<Query>, Vec<ParseError>) {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let parser = Parser::new(tokens);
        parser.parse()
    }

    fn parse_ok(input: &str) -> Query {
        let (query, errors) = parse(input);
        assert!(
            errors.is_empty(),
            "unexpected errors for '{}': {:?}",
            input,
            errors
        );
        query.unwrap()
    }

    fn parse_errors(input: &str) -> Vec<ParseError> {
        let (_, errors) = parse(input);
        errors
    }

    // ---- 基本テスト ----

    #[test]
    fn test_empty_query() {
        let query = parse_ok("");
        assert!(query.stages.is_empty());
    }

    #[test]
    fn test_simple_search() {
        let query = parse_ok("error");
        assert_eq!(query.stages.len(), 1);
        assert!(matches!(
            &query.stages[0].kind,
            StageKind::Search(SearchExpr::FreeText(t)) if t == "error"
        ));
    }

    #[test]
    fn test_wildcard_search() {
        let query = parse_ok("*");
        assert_eq!(query.stages.len(), 1);
        assert!(matches!(
            &query.stages[0].kind,
            StageKind::Search(SearchExpr::FreeText(t)) if t == "*"
        ));
    }

    #[test]
    fn test_field_filter() {
        let query = parse_ok("status=200");
        assert_eq!(query.stages.len(), 1);
        if let StageKind::Search(SearchExpr::FieldFilter { field, op, value }) =
            &query.stages[0].kind
        {
            assert_eq!(field, "status");
            assert_eq!(*op, CompareOp::Eq);
            assert_eq!(*value, FilterValue::Number(200.0));
        } else {
            panic!("expected FieldFilter, got {:?}", query.stages[0].kind);
        }
    }

    #[test]
    fn test_field_filter_string() {
        let query = parse_ok(r#"host="server01""#);
        if let StageKind::Search(SearchExpr::FieldFilter { field, op, value }) =
            &query.stages[0].kind
        {
            assert_eq!(field, "host");
            assert_eq!(*op, CompareOp::Eq);
            assert_eq!(*value, FilterValue::String("server01".to_string()));
        } else {
            panic!("expected FieldFilter");
        }
    }

    #[test]
    fn test_field_filter_not_eq() {
        let query = parse_ok("status!=200");
        if let StageKind::Search(SearchExpr::FieldFilter { op, .. }) = &query.stages[0].kind {
            assert_eq!(*op, CompareOp::NotEq);
        } else {
            panic!("expected FieldFilter");
        }
    }

    #[test]
    fn test_field_filter_gt() {
        let query = parse_ok("status>200");
        if let StageKind::Search(SearchExpr::FieldFilter { op, .. }) = &query.stages[0].kind {
            assert_eq!(*op, CompareOp::Gt);
        } else {
            panic!("expected FieldFilter");
        }
    }

    #[test]
    fn test_and_expression() {
        let query = parse_ok("status=200 AND host=server01");
        assert_eq!(query.stages.len(), 1);
        assert!(matches!(
            &query.stages[0].kind,
            StageKind::Search(SearchExpr::And(_, _))
        ));
    }

    #[test]
    fn test_or_expression() {
        let query = parse_ok("status=200 OR status=404");
        assert_eq!(query.stages.len(), 1);
        assert!(matches!(
            &query.stages[0].kind,
            StageKind::Search(SearchExpr::Or(_, _))
        ));
    }

    #[test]
    fn test_not_expression() {
        let query = parse_ok("NOT status=200");
        assert_eq!(query.stages.len(), 1);
        assert!(matches!(
            &query.stages[0].kind,
            StageKind::Search(SearchExpr::Not(_))
        ));
    }

    #[test]
    fn test_grouped_expression() {
        let query = parse_ok("(status=200 OR status=404)");
        assert_eq!(query.stages.len(), 1);
        assert!(matches!(
            &query.stages[0].kind,
            StageKind::Search(SearchExpr::Grouped(_))
        ));
    }

    #[test]
    fn test_implicit_and() {
        let query = parse_ok("status=200 host=server01");
        assert_eq!(query.stages.len(), 1);
        assert!(matches!(
            &query.stages[0].kind,
            StageKind::Search(SearchExpr::And(_, _))
        ));
    }

    #[test]
    fn test_explicit_search_command() {
        let query = parse_ok("search status=200");
        assert_eq!(query.stages.len(), 1);
        assert!(matches!(
            &query.stages[0].kind,
            StageKind::Search(SearchExpr::FieldFilter { .. })
        ));
    }

    // ---- コマンドテスト ----

    #[test]
    fn test_simple_command_after_pipe() {
        let query = parse_ok("status=200 | stats count");
        assert_eq!(query.stages.len(), 2);
        if let StageKind::Command(cmd) = &query.stages[1].kind {
            assert_eq!(cmd.name, "stats");
            assert_eq!(cmd.arguments.len(), 1);
        } else {
            panic!("expected Command");
        }
    }

    #[test]
    fn test_command_with_by_clause() {
        let query = parse_ok("status=200 | stats count by host");
        if let StageKind::Command(cmd) = &query.stages[1].kind {
            assert_eq!(cmd.name, "stats");
            assert_eq!(cmd.by_clause, Some(vec!["host".to_string()]));
        } else {
            panic!("expected Command");
        }
    }

    #[test]
    fn test_command_with_multiple_by_fields() {
        let query = parse_ok("status=200 | stats count by host, src_ip");
        if let StageKind::Command(cmd) = &query.stages[1].kind {
            assert_eq!(
                cmd.by_clause,
                Some(vec!["host".to_string(), "src_ip".to_string()])
            );
        } else {
            panic!("expected Command");
        }
    }

    #[test]
    fn test_command_with_named_arg() {
        let query = parse_ok("status=200 | head limit=10");
        if let StageKind::Command(cmd) = &query.stages[1].kind {
            assert_eq!(cmd.name, "head");
            assert!(matches!(
                &cmd.arguments[0],
                CommandArg::Named { name, value } if name == "limit" && *value == Expr::Number(10.0)
            ));
        } else {
            panic!("expected Command");
        }
    }

    #[test]
    fn test_pipeline() {
        let query = parse_ok("status=200 | stats count by host");
        assert_eq!(query.stages.len(), 2);
        assert!(matches!(&query.stages[0].kind, StageKind::Search(_)));
        assert!(matches!(&query.stages[1].kind, StageKind::Command(_)));
    }

    #[test]
    fn test_three_stage_pipeline() {
        let query = parse_ok("status=200 | stats count by host | sort count");
        assert_eq!(query.stages.len(), 3);
    }

    #[test]
    fn test_eval_command() {
        let query = parse_ok("status=200 | eval total = count * 2");
        if let StageKind::Command(cmd) = &query.stages[1].kind {
            assert_eq!(cmd.name, "eval");
        } else {
            panic!("expected Command");
        }
    }

    #[test]
    fn test_table_command() {
        let query = parse_ok("status=200 | table host, src_ip, status");
        if let StageKind::Command(cmd) = &query.stages[1].kind {
            assert_eq!(cmd.name, "table");
            assert_eq!(cmd.arguments.len(), 3);
        } else {
            panic!("expected Command");
        }
    }

    #[test]
    fn test_rename_command_with_as() {
        let query = parse_ok("status=200 | rename src as source");
        if let StageKind::Command(cmd) = &query.stages[1].kind {
            assert_eq!(cmd.name, "rename");
        } else {
            panic!("expected Command");
        }
    }

    // ---- 関数呼び出しテスト ----

    #[test]
    fn test_function_call_in_command() {
        let query = parse_ok("status=200 | stats count(status)");
        if let StageKind::Command(cmd) = &query.stages[1].kind {
            assert_eq!(cmd.name, "stats");
            assert_eq!(cmd.arguments.len(), 1);
        } else {
            panic!("expected Command");
        }
    }

    #[test]
    fn test_nested_function_call() {
        let query = parse_ok("status=200 | eval total = if(status>200, 1, 0)");
        if let StageKind::Command(cmd) = &query.stages[1].kind {
            assert_eq!(cmd.name, "eval");
        } else {
            panic!("expected Command");
        }
    }

    // ---- サブサーチテスト ----

    #[test]
    fn test_subsearch() {
        let query = parse_ok("[search status=200]");
        assert_eq!(query.stages.len(), 1);
        assert!(matches!(
            &query.stages[0].kind,
            StageKind::Search(SearchExpr::SubSearch(_))
        ));
    }

    // ---- エラーテスト ----

    #[test]
    fn test_syntax_error_unclosed_paren() {
        let errors = parse_errors("(status=200");
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_syntax_error_multibyte() {
        let errors = parse_errors("これはテスト | stats count");
        assert!(!errors.is_empty());
    }

    // ---- wildcard テスト ----

    #[test]
    fn test_wildcard_field_value() {
        let query = parse_ok("status=error*");
        if let StageKind::Search(SearchExpr::FieldFilter { value, .. }) = &query.stages[0].kind {
            assert_eq!(*value, FilterValue::Wildcard("error*".to_string()));
        } else {
            panic!("expected FieldFilter with wildcard");
        }
    }

    // ---- 複雑なクエリ ----

    #[test]
    fn test_complex_pipeline() {
        let query =
            parse_ok(r#"status=200 host="web*" | stats count by src_ip | sort count | head 10"#);
        assert_eq!(query.stages.len(), 4);
    }

    #[test]
    fn test_and_or_precedence() {
        // AND は OR より結合が強いです
        let query = parse_ok("a=1 OR b=2 AND c=3");
        // OR(a=1, AND(b=2, c=3)) となるべきです
        assert!(matches!(
            &query.stages[0].kind,
            StageKind::Search(SearchExpr::Or(_, _))
        ));
    }

    #[test]
    fn test_search_with_string_freetext() {
        let query = parse_ok(r#""login failed""#);
        assert!(matches!(
            &query.stages[0].kind,
            StageKind::Search(SearchExpr::FreeText(t)) if t == "login failed"
        ));
    }

    #[test]
    fn test_filter_with_wildcard_value() {
        let query = parse_ok("host=*web*");
        if let StageKind::Search(SearchExpr::FieldFilter { value, .. }) = &query.stages[0].kind {
            assert_eq!(*value, FilterValue::Wildcard("*web*".to_string()));
        } else {
            panic!("expected FieldFilter with wildcard");
        }
    }

    #[test]
    fn test_multiple_pipelines_complex() {
        let query = parse_ok("status=404 | where count > 5 | table host, count");
        assert_eq!(query.stages.len(), 3);
    }

    #[test]
    fn test_command_with_function_and_by() {
        let query = parse_ok("status=200 | stats avg(response_time) by host");
        if let StageKind::Command(cmd) = &query.stages[1].kind {
            assert_eq!(cmd.name, "stats");
            assert!(cmd.by_clause.is_some());
        } else {
            panic!("expected Command");
        }
    }

    #[test]
    fn test_backtick_macro_in_search() {
        let query = parse_ok("`this is a comment` status=200");
        assert_eq!(query.stages.len(), 1);
        // バッククォートマクロは FreeText として search 式に含まれ、
        // 暗黙の AND で FieldFilter と結合されます
        assert!(matches!(
            &query.stages[0].kind,
            StageKind::Search(SearchExpr::And(_, _))
        ));
    }

    #[test]
    fn test_where_command_with_comparison() {
        let query = parse_ok("status=200 | where count > 10");
        if let StageKind::Command(cmd) = &query.stages[1].kind {
            assert_eq!(cmd.name, "where");
        } else {
            panic!("expected Command");
        }
    }

    #[test]
    fn test_dedup_command() {
        let query = parse_ok("status=200 | dedup host");
        if let StageKind::Command(cmd) = &query.stages[1].kind {
            assert_eq!(cmd.name, "dedup");
        } else {
            panic!("expected Command");
        }
    }

    #[test]
    fn test_sort_command() {
        let query = parse_ok("status=200 | sort count");
        if let StageKind::Command(cmd) = &query.stages[1].kind {
            assert_eq!(cmd.name, "sort");
        } else {
            panic!("expected Command");
        }
    }

    #[test]
    fn test_fields_command() {
        let query = parse_ok("status=200 | fields host, src_ip, status");
        if let StageKind::Command(cmd) = &query.stages[1].kind {
            assert_eq!(cmd.name, "fields");
            assert_eq!(cmd.arguments.len(), 3);
        } else {
            panic!("expected Command");
        }
    }

    #[test]
    fn test_search_with_not_field_filter() {
        let query = parse_ok("NOT status=200");
        assert!(matches!(
            &query.stages[0].kind,
            StageKind::Search(SearchExpr::Not(_))
        ));
    }

    #[test]
    fn test_freetext_then_pipe_command() {
        let query = parse_ok("error | stats count");
        assert_eq!(query.stages.len(), 2);
        assert!(matches!(
            &query.stages[0].kind,
            StageKind::Search(SearchExpr::FreeText(t)) if t == "error"
        ));
        assert!(matches!(&query.stages[1].kind, StageKind::Command(_)));
    }

    #[test]
    fn test_comparison_in_function_args() {
        let query = parse_ok("status=200 | eval x = if(count > 10, 1, 0)");
        assert_eq!(query.stages.len(), 2);
    }

    #[test]
    fn test_stats_with_function_as() {
        let query = parse_ok("status=200 | stats count as total");
        if let StageKind::Command(cmd) = &query.stages[1].kind {
            assert_eq!(cmd.name, "stats");
            assert_eq!(cmd.as_clause, Some("total".to_string()));
        } else {
            panic!("expected Command");
        }
    }

    #[test]
    fn test_multiple_freetext_implicit_and() {
        let query = parse_ok("error warning");
        assert_eq!(query.stages.len(), 1);
        assert!(matches!(
            &query.stages[0].kind,
            StageKind::Search(SearchExpr::And(_, _))
        ));
    }

    #[test]
    fn test_leading_pipe_generating_command() {
        // `| makeresults` のような先頭パイプ付き generating command をパースできることを確認します
        let query = parse_ok("| makeresults");
        assert_eq!(query.stages.len(), 1);
        if let StageKind::Command(cmd) = &query.stages[0].kind {
            assert_eq!(cmd.name, "makeresults");
        } else {
            panic!("expected Command, got {:?}", query.stages[0].kind);
        }
    }

    #[test]
    fn test_leading_pipe_with_pipeline() {
        // `| makeresults | eval x=1` のような先頭パイプ付きパイプラインをパースできることを確認します
        let query = parse_ok("| makeresults | eval x=1");
        assert_eq!(query.stages.len(), 2);
        if let StageKind::Command(cmd) = &query.stages[0].kind {
            assert_eq!(cmd.name, "makeresults");
        } else {
            panic!("expected Command for stage 0");
        }
        if let StageKind::Command(cmd) = &query.stages[1].kind {
            assert_eq!(cmd.name, "eval");
        } else {
            panic!("expected Command for stage 1");
        }
    }
}
