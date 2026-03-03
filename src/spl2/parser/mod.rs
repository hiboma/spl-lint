pub mod ast;

use crate::lexer::token::Span;
use crate::spl2::lexer::token::{Token, TokenKind};
use ast::*;

/// SPL2 パースエラーを表します。
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

/// SPL2 の構文解析器です。
pub(crate) struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    errors: Vec<ParseError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            errors: Vec::new(),
        }
    }

    /// クエリ文字列をパースして AST とエラーのリストを返します。
    pub fn parse(mut self) -> (Option<Spl2Query>, Vec<ParseError>) {
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

    fn peek_ahead(&self, offset: usize) -> &TokenKind {
        self.tokens
            .get(self.pos + offset)
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
        let tok = self.tokens.get(self.pos).cloned().unwrap_or_else(|| {
            Token::new(
                TokenKind::Eof,
                self.tokens
                    .last()
                    .map(|t| Span::new(t.span.end, t.span.end))
                    .unwrap_or(Span::new(0, 0)),
            )
        });
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: &TokenKind) -> Option<Token> {
        if self.peek() == expected {
            Some(self.advance())
        } else {
            self.errors.push(ParseError::new(
                format!("expected {:?}, found {:?}", expected, self.peek()),
                self.current_span(),
            ));
            None
        }
    }

    /// 現在のトークンが識別子であればその名前を返して進めます。
    /// キーワードも文脈に応じて識別子として使用できます。
    fn expect_identifier(&mut self) -> Option<String> {
        match self.peek().clone() {
            TokenKind::Identifier(name) => {
                self.advance();
                Some(name)
            }
            // キーワードを識別子として使用する場合
            _ if self.is_keyword_as_identifier() => {
                let name = self.keyword_to_string();
                self.advance();
                Some(name)
            }
            _ => {
                self.errors.push(ParseError::new(
                    format!("expected identifier, found {:?}", self.peek()),
                    self.current_span(),
                ));
                None
            }
        }
    }

    /// 現在のキーワードが識別子として使用可能かを判定します。
    fn is_keyword_as_identifier(&self) -> bool {
        matches!(
            self.peek(),
            TokenKind::From
                | TokenKind::Select
                | TokenKind::Where
                | TokenKind::Having
                | TokenKind::Limit
                | TokenKind::Offset
                | TokenKind::Join
                | TokenKind::Inner
                | TokenKind::Left
                | TokenKind::Outer
                | TokenKind::On
                | TokenKind::Asc
                | TokenKind::Desc
                | TokenKind::Distinct
                | TokenKind::In
                | TokenKind::Is
                | TokenKind::Like
                | TokenKind::Between
                | TokenKind::Exists
                | TokenKind::Into
                | TokenKind::Union
                | TokenKind::Null
                | TokenKind::True
                | TokenKind::False
        )
    }

    /// 現在のキーワードトークンを文字列に変換します。
    fn keyword_to_string(&self) -> String {
        match self.peek() {
            TokenKind::From => "from".to_string(),
            TokenKind::Select => "select".to_string(),
            TokenKind::Where => "where".to_string(),
            TokenKind::Having => "having".to_string(),
            TokenKind::Limit => "limit".to_string(),
            TokenKind::Offset => "offset".to_string(),
            TokenKind::Join => "join".to_string(),
            TokenKind::Inner => "inner".to_string(),
            TokenKind::Left => "left".to_string(),
            TokenKind::Outer => "outer".to_string(),
            TokenKind::On => "on".to_string(),
            TokenKind::Asc => "asc".to_string(),
            TokenKind::Desc => "desc".to_string(),
            TokenKind::Distinct => "distinct".to_string(),
            TokenKind::In => "in".to_string(),
            TokenKind::Is => "is".to_string(),
            TokenKind::Like => "like".to_string(),
            TokenKind::Between => "between".to_string(),
            TokenKind::Exists => "exists".to_string(),
            TokenKind::Into => "into".to_string(),
            TokenKind::Union => "union".to_string(),
            TokenKind::Null => "null".to_string(),
            TokenKind::True => "true".to_string(),
            TokenKind::False => "false".to_string(),
            _ => String::new(),
        }
    }

    /// "GROUP" の後に "BY" が続くかを確認します。
    fn is_group_by(&self) -> bool {
        if let TokenKind::Identifier(name) = self.peek() {
            if name.eq_ignore_ascii_case("group") {
                if let TokenKind::By = self.peek_ahead(1) {
                    return true;
                }
            }
        }
        false
    }

    /// "ORDER" の後に "BY" が続くかを確認します。
    fn is_order_by(&self) -> bool {
        if let TokenKind::Identifier(name) = self.peek() {
            if name.eq_ignore_ascii_case("order") {
                if let TokenKind::By = self.peek_ahead(1) {
                    return true;
                }
            }
        }
        false
    }

    // ---- パース ----

    fn parse_query(&mut self) -> Option<Spl2Query> {
        if matches!(self.peek(), TokenKind::Eof) {
            return None;
        }

        let start = self.current_span().start;
        let mut stages = Vec::new();

        // 最初のステージをパースします
        if let Some(stage) = self.parse_stage() {
            stages.push(stage);
        } else {
            return None;
        }

        // パイプで区切られた後続のステージをパースします
        while matches!(self.peek(), TokenKind::Pipe) {
            self.advance(); // | を消費します
            if let Some(stage) = self.parse_stage() {
                stages.push(stage);
            } else {
                break;
            }
        }

        let end = stages.last().map(|s| s.span.end).unwrap_or(start);
        Some(Spl2Query {
            stages,
            span: Span::new(start, end),
        })
    }

    fn parse_stage(&mut self) -> Option<Spl2PipelineStage> {
        let start = self.current_span().start;

        match self.peek() {
            TokenKind::From => {
                let stmt = self.parse_from_statement()?;
                let end = stmt.span.end;
                Some(Spl2PipelineStage {
                    kind: Spl2StageKind::FromStatement(stmt),
                    span: Span::new(start, end),
                })
            }
            TokenKind::Select => {
                let stmt = self.parse_select_statement()?;
                let end = stmt.span.end;
                Some(Spl2PipelineStage {
                    kind: Spl2StageKind::SelectStatement(stmt),
                    span: Span::new(start, end),
                })
            }
            _ => {
                let cmd = self.parse_command()?;
                let end = cmd.span.end;
                Some(Spl2PipelineStage {
                    kind: Spl2StageKind::Command(cmd),
                    span: Span::new(start, end),
                })
            }
        }
    }

    // ---- FROM 文 ----

    fn parse_from_statement(&mut self) -> Option<FromStatement> {
        let start = self.current_span().start;
        self.advance(); // FROM を消費します

        let dataset = self.expect_identifier()?;

        let mut joins = Vec::new();
        let mut where_clause = None;
        let mut group_by = None;
        let mut having = None;
        let mut select = None;
        let mut order_by = None;
        let mut limit = None;
        let mut offset = None;

        // SQL 句をパースします
        loop {
            match self.peek() {
                TokenKind::Join | TokenKind::Inner | TokenKind::Left => {
                    if let Some(join) = self.parse_join_clause() {
                        joins.push(join);
                    }
                }
                TokenKind::Where => {
                    self.advance();
                    where_clause = Some(Box::new(self.parse_expression()?));
                }
                _ if self.is_group_by() => {
                    self.advance(); // GROUP
                    self.advance(); // BY
                    group_by = Some(self.parse_expression_list()?);
                }
                TokenKind::Having => {
                    self.advance();
                    having = Some(Box::new(self.parse_expression()?));
                }
                TokenKind::Select => {
                    self.advance();
                    select = Some(self.parse_select_items()?);
                }
                _ if self.is_order_by() => {
                    self.advance(); // ORDER
                    self.advance(); // BY
                    order_by = Some(self.parse_order_by_items()?);
                }
                TokenKind::Limit => {
                    self.advance();
                    limit = Some(Box::new(self.parse_expression()?));
                }
                TokenKind::Offset => {
                    self.advance();
                    offset = Some(Box::new(self.parse_expression()?));
                }
                _ => break,
            }
        }

        let end = self.prev_end();
        Some(FromStatement {
            dataset,
            joins,
            where_clause,
            group_by,
            having,
            select,
            order_by,
            limit,
            offset,
            span: Span::new(start, end),
        })
    }

    // ---- SELECT 文 ----

    fn parse_select_statement(&mut self) -> Option<SelectStatement> {
        let start = self.current_span().start;
        self.advance(); // SELECT を消費します

        let distinct = if matches!(self.peek(), TokenKind::Distinct) {
            self.advance();
            true
        } else {
            false
        };

        let items = self.parse_select_items()?;

        let mut from = None;
        let mut joins = Vec::new();
        let mut where_clause = None;
        let mut group_by = None;
        let mut having = None;
        let mut order_by = None;
        let mut limit = None;
        let mut offset = None;

        if matches!(self.peek(), TokenKind::From) {
            self.advance();
            from = Some(self.expect_identifier()?);
        }

        loop {
            match self.peek() {
                TokenKind::Join | TokenKind::Inner | TokenKind::Left => {
                    if let Some(join) = self.parse_join_clause() {
                        joins.push(join);
                    }
                }
                TokenKind::Where => {
                    self.advance();
                    where_clause = Some(Box::new(self.parse_expression()?));
                }
                _ if self.is_group_by() => {
                    self.advance(); // GROUP
                    self.advance(); // BY
                    group_by = Some(self.parse_expression_list()?);
                }
                TokenKind::Having => {
                    self.advance();
                    having = Some(Box::new(self.parse_expression()?));
                }
                _ if self.is_order_by() => {
                    self.advance(); // ORDER
                    self.advance(); // BY
                    order_by = Some(self.parse_order_by_items()?);
                }
                TokenKind::Limit => {
                    self.advance();
                    limit = Some(Box::new(self.parse_expression()?));
                }
                TokenKind::Offset => {
                    self.advance();
                    offset = Some(Box::new(self.parse_expression()?));
                }
                _ => break,
            }
        }

        let end = self.prev_end();
        Some(SelectStatement {
            distinct,
            items,
            from,
            joins,
            where_clause,
            group_by,
            having,
            order_by,
            limit,
            offset,
            span: Span::new(start, end),
        })
    }

    fn parse_select_items(&mut self) -> Option<Vec<SelectItem>> {
        let mut items = Vec::new();
        items.push(self.parse_select_item()?);
        while matches!(self.peek(), TokenKind::Comma) {
            self.advance();
            items.push(self.parse_select_item()?);
        }
        Some(items)
    }

    fn parse_select_item(&mut self) -> Option<SelectItem> {
        // * だけの場合
        if matches!(self.peek(), TokenKind::Star) {
            self.advance();
            let alias = if matches!(self.peek(), TokenKind::As) {
                self.advance();
                Some(self.expect_identifier()?)
            } else {
                None
            };
            return Some(SelectItem {
                expr: Spl2Expr::Star,
                alias,
            });
        }

        let expr = self.parse_expression()?;
        let alias = if matches!(self.peek(), TokenKind::As) {
            self.advance();
            Some(self.expect_identifier()?)
        } else {
            None
        };
        Some(SelectItem { expr, alias })
    }

    fn parse_join_clause(&mut self) -> Option<JoinClause> {
        let join_type = match self.peek() {
            TokenKind::Inner => {
                self.advance();
                self.expect(&TokenKind::Join)?;
                JoinType::Inner
            }
            TokenKind::Left => {
                self.advance();
                if matches!(self.peek(), TokenKind::Outer) {
                    self.advance();
                    self.expect(&TokenKind::Join)?;
                    JoinType::LeftOuter
                } else {
                    self.expect(&TokenKind::Join)?;
                    JoinType::Left
                }
            }
            TokenKind::Join => {
                self.advance();
                JoinType::Inner
            }
            _ => return None,
        };

        let dataset = self.expect_identifier()?;

        let on_condition = if matches!(self.peek(), TokenKind::On) {
            self.advance();
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };

        Some(JoinClause {
            join_type,
            dataset,
            on_condition,
        })
    }

    fn parse_order_by_items(&mut self) -> Option<Vec<OrderByItem>> {
        let mut items = Vec::new();
        items.push(self.parse_order_by_item()?);
        while matches!(self.peek(), TokenKind::Comma) {
            self.advance();
            items.push(self.parse_order_by_item()?);
        }
        Some(items)
    }

    fn parse_order_by_item(&mut self) -> Option<OrderByItem> {
        let expr = self.parse_expression()?;
        let direction = match self.peek() {
            TokenKind::Asc => {
                self.advance();
                SortDirection::Asc
            }
            TokenKind::Desc => {
                self.advance();
                SortDirection::Desc
            }
            _ => SortDirection::Asc,
        };
        Some(OrderByItem { expr, direction })
    }

    // ---- コマンド ----

    fn parse_command(&mut self) -> Option<Spl2Command> {
        let start = self.current_span().start;

        let name = match self.peek().clone() {
            TokenKind::Identifier(name) => {
                self.advance();
                name
            }
            // where などのキーワードもコマンド名として使用できます
            TokenKind::Where => {
                self.advance();
                "where".to_string()
            }
            TokenKind::Join => {
                self.advance();
                "join".to_string()
            }
            TokenKind::Into => {
                self.advance();
                "into".to_string()
            }
            TokenKind::Union => {
                self.advance();
                "union".to_string()
            }
            _ => {
                self.errors.push(ParseError::new(
                    format!("expected command name, found {:?}", self.peek()),
                    self.current_span(),
                ));
                self.advance();
                return None;
            }
        };

        let mut arguments = Vec::new();
        let mut by_clause = None;
        let as_clause = None;

        // コマンド引数をパースします
        while !matches!(
            self.peek(),
            TokenKind::Pipe | TokenKind::Eof | TokenKind::RBracket
        ) {
            // BY 句
            if matches!(self.peek(), TokenKind::By) {
                self.advance();
                by_clause = Some(self.parse_expression_list().unwrap_or_default());
                continue;
            }

            // WHERE 句がステージレベルのキーワードとしてきた場合はブレイクします
            if matches!(
                self.peek(),
                TokenKind::Where | TokenKind::Having | TokenKind::Select
            ) && !arguments.is_empty()
            {
                break;
            }
            if self.is_group_by() || self.is_order_by() {
                break;
            }

            // 名前付き引数 (name = value) の検出
            if self.is_named_arg() {
                let arg_name = match self.peek().clone() {
                    TokenKind::Identifier(n) => n,
                    _ => self.keyword_to_string(),
                };
                self.advance(); // name
                self.advance(); // =
                if let Some(value) = self.parse_expression() {
                    arguments.push(Spl2CommandArg::Named {
                        name: arg_name,
                        value,
                    });
                }
                // カンマがあればスキップします
                if matches!(self.peek(), TokenKind::Comma) {
                    self.advance();
                }
                continue;
            }

            // 位置引数
            if let Some(expr) = self.parse_expression() {
                // AS 別名がある場合は読み取ります
                let alias = if matches!(self.peek(), TokenKind::As) {
                    self.advance();
                    self.expect_identifier()
                } else {
                    None
                };
                arguments.push(Spl2CommandArg::Positional(expr, alias));
            } else {
                // パースに失敗した場合、無限ループを防ぐためにトークンを進めます
                self.advance();
            }

            // カンマがあればスキップします (コマンド引数のカンマ区切り)
            if matches!(self.peek(), TokenKind::Comma) {
                self.advance();
            }
        }

        let end = self.prev_end();
        Some(Spl2Command {
            name,
            arguments,
            by_clause,
            as_clause,
            span: Span::new(start, end),
        })
    }

    fn is_named_arg(&self) -> bool {
        let is_name =
            matches!(self.peek(), TokenKind::Identifier(_)) || self.is_keyword_as_identifier();
        is_name && matches!(self.peek_ahead(1), TokenKind::Eq)
    }

    // ---- 式パース ----

    fn parse_expression(&mut self) -> Option<Spl2Expr> {
        self.parse_or_expr()
    }

    fn parse_expression_list(&mut self) -> Option<Vec<Spl2Expr>> {
        let mut exprs = Vec::new();
        exprs.push(self.parse_expression()?);
        while matches!(self.peek(), TokenKind::Comma) {
            self.advance();
            exprs.push(self.parse_expression()?);
        }
        Some(exprs)
    }

    /// OR 式をパースします。
    fn parse_or_expr(&mut self) -> Option<Spl2Expr> {
        let mut left = self.parse_xor_expr()?;
        while matches!(self.peek(), TokenKind::Or) {
            self.advance();
            let right = self.parse_xor_expr()?;
            left = Spl2Expr::Or(Box::new(left), Box::new(right));
        }
        Some(left)
    }

    /// XOR 式をパースします。
    fn parse_xor_expr(&mut self) -> Option<Spl2Expr> {
        let mut left = self.parse_and_expr()?;
        while matches!(self.peek(), TokenKind::Xor) {
            self.advance();
            let right = self.parse_and_expr()?;
            left = Spl2Expr::Xor(Box::new(left), Box::new(right));
        }
        Some(left)
    }

    /// AND 式をパースします。
    fn parse_and_expr(&mut self) -> Option<Spl2Expr> {
        let mut left = self.parse_not_expr()?;
        while matches!(self.peek(), TokenKind::And) {
            self.advance();
            let right = self.parse_not_expr()?;
            left = Spl2Expr::And(Box::new(left), Box::new(right));
        }
        Some(left)
    }

    /// NOT 式をパースします。
    fn parse_not_expr(&mut self) -> Option<Spl2Expr> {
        if matches!(self.peek(), TokenKind::Not) {
            self.advance();
            let expr = self.parse_not_expr()?;
            return Some(Spl2Expr::Not(Box::new(expr)));
        }
        if matches!(self.peek(), TokenKind::Bang) {
            self.advance();
            let expr = self.parse_not_expr()?;
            return Some(Spl2Expr::Not(Box::new(expr)));
        }
        self.parse_comparison_expr()
    }

    /// 比較式をパースします。
    fn parse_comparison_expr(&mut self) -> Option<Spl2Expr> {
        let mut left = self.parse_additive_expr()?;

        loop {
            match self.peek() {
                TokenKind::Eq => {
                    self.advance();
                    let right = self.parse_additive_expr()?;
                    left = Spl2Expr::CompareExpr {
                        left: Box::new(left),
                        op: Spl2CompareOp::Eq,
                        right: Box::new(right),
                    };
                }
                TokenKind::NotEq => {
                    self.advance();
                    let right = self.parse_additive_expr()?;
                    left = Spl2Expr::CompareExpr {
                        left: Box::new(left),
                        op: Spl2CompareOp::NotEq,
                        right: Box::new(right),
                    };
                }
                TokenKind::Lt => {
                    self.advance();
                    let right = self.parse_additive_expr()?;
                    left = Spl2Expr::CompareExpr {
                        left: Box::new(left),
                        op: Spl2CompareOp::Lt,
                        right: Box::new(right),
                    };
                }
                TokenKind::LtEq => {
                    self.advance();
                    let right = self.parse_additive_expr()?;
                    left = Spl2Expr::CompareExpr {
                        left: Box::new(left),
                        op: Spl2CompareOp::LtEq,
                        right: Box::new(right),
                    };
                }
                TokenKind::Gt => {
                    self.advance();
                    let right = self.parse_additive_expr()?;
                    left = Spl2Expr::CompareExpr {
                        left: Box::new(left),
                        op: Spl2CompareOp::Gt,
                        right: Box::new(right),
                    };
                }
                TokenKind::GtEq => {
                    self.advance();
                    let right = self.parse_additive_expr()?;
                    left = Spl2Expr::CompareExpr {
                        left: Box::new(left),
                        op: Spl2CompareOp::GtEq,
                        right: Box::new(right),
                    };
                }
                TokenKind::In => {
                    self.advance();
                    left = self.parse_in_list(left, false)?;
                }
                TokenKind::Not => {
                    // NOT IN, NOT BETWEEN, NOT LIKE
                    if matches!(self.peek_ahead(1), TokenKind::In) {
                        self.advance(); // NOT
                        self.advance(); // IN
                        left = self.parse_in_list(left, true)?;
                    } else if matches!(self.peek_ahead(1), TokenKind::Between) {
                        self.advance(); // NOT
                        self.advance(); // BETWEEN
                        left = self.parse_between(left, true)?;
                    } else if matches!(self.peek_ahead(1), TokenKind::Like) {
                        self.advance(); // NOT
                        self.advance(); // LIKE
                        let pattern = self.parse_additive_expr()?;
                        left = Spl2Expr::Like {
                            expr: Box::new(left),
                            pattern: Box::new(pattern),
                            negated: true,
                        };
                    } else {
                        break;
                    }
                }
                TokenKind::Between => {
                    self.advance();
                    left = self.parse_between(left, false)?;
                }
                TokenKind::Is => {
                    self.advance();
                    let negated = if matches!(self.peek(), TokenKind::Not) {
                        self.advance();
                        true
                    } else {
                        false
                    };
                    self.expect(&TokenKind::Null)?;
                    left = Spl2Expr::IsNull {
                        expr: Box::new(left),
                        negated,
                    };
                }
                TokenKind::Like => {
                    self.advance();
                    let pattern = self.parse_additive_expr()?;
                    left = Spl2Expr::Like {
                        expr: Box::new(left),
                        pattern: Box::new(pattern),
                        negated: false,
                    };
                }
                _ => break,
            }
        }

        Some(left)
    }

    fn parse_in_list(&mut self, expr: Spl2Expr, negated: bool) -> Option<Spl2Expr> {
        self.expect(&TokenKind::LParen)?;
        let values = self.parse_expression_list()?;
        self.expect(&TokenKind::RParen)?;
        Some(Spl2Expr::InList {
            expr: Box::new(expr),
            values,
            negated,
        })
    }

    fn parse_between(&mut self, expr: Spl2Expr, negated: bool) -> Option<Spl2Expr> {
        let low = self.parse_additive_expr()?;
        self.expect(&TokenKind::And)?;
        let high = self.parse_additive_expr()?;
        Some(Spl2Expr::Between {
            expr: Box::new(expr),
            low: Box::new(low),
            high: Box::new(high),
            negated,
        })
    }

    /// 加算・減算式をパースします。
    fn parse_additive_expr(&mut self) -> Option<Spl2Expr> {
        let mut left = self.parse_multiplicative_expr()?;
        while matches!(self.peek(), TokenKind::Plus | TokenKind::Minus) {
            let op = match self.peek() {
                TokenKind::Plus => Spl2BinaryOp::Add,
                TokenKind::Minus => Spl2BinaryOp::Sub,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_multiplicative_expr()?;
            left = Spl2Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Some(left)
    }

    /// 乗算・除算式をパースします。
    fn parse_multiplicative_expr(&mut self) -> Option<Spl2Expr> {
        let mut left = self.parse_unary_expr()?;
        while matches!(
            self.peek(),
            TokenKind::Star | TokenKind::Slash | TokenKind::Percent
        ) {
            let op = match self.peek() {
                TokenKind::Star => Spl2BinaryOp::Mul,
                TokenKind::Slash => Spl2BinaryOp::Div,
                TokenKind::Percent => Spl2BinaryOp::Mod,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_unary_expr()?;
            left = Spl2Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        // `.` による文字列連結 (SPL2 では非推奨)
        while matches!(self.peek(), TokenKind::Dot) {
            self.advance();
            let right = self.parse_unary_expr()?;
            left = Spl2Expr::BinaryOp {
                left: Box::new(left),
                op: Spl2BinaryOp::Concat,
                right: Box::new(right),
            };
        }
        Some(left)
    }

    /// 単項式をパースします。
    fn parse_unary_expr(&mut self) -> Option<Spl2Expr> {
        match self.peek() {
            TokenKind::Minus => {
                self.advance();
                let operand = self.parse_unary_expr()?;
                Some(Spl2Expr::UnaryOp {
                    op: Spl2UnaryOp::Neg,
                    operand: Box::new(operand),
                })
            }
            _ => self.parse_primary_expr(),
        }
    }

    /// プライマリ式をパースします。
    fn parse_primary_expr(&mut self) -> Option<Spl2Expr> {
        match self.peek().clone() {
            TokenKind::Integer(n) => {
                self.advance();
                Some(Spl2Expr::Number(n as f64))
            }
            TokenKind::Float(n) => {
                self.advance();
                Some(Spl2Expr::Number(n))
            }
            TokenKind::StringLiteral(s) => {
                self.advance();
                Some(Spl2Expr::String(s))
            }
            TokenKind::RawString(s) => {
                self.advance();
                Some(Spl2Expr::String(s))
            }
            TokenKind::True => {
                self.advance();
                Some(Spl2Expr::Bool(true))
            }
            TokenKind::False => {
                self.advance();
                Some(Spl2Expr::Bool(false))
            }
            TokenKind::Null => {
                self.advance();
                Some(Spl2Expr::Null)
            }
            TokenKind::SingleQuotedField(name) => {
                self.advance();
                Some(Spl2Expr::QuotedField(name))
            }
            TokenKind::Wildcard(w) => {
                self.advance();
                Some(Spl2Expr::Wildcard(w))
            }
            TokenKind::Star => {
                self.advance();
                Some(Spl2Expr::Star)
            }
            TokenKind::SearchLiteral(s) => {
                self.advance();
                Some(Spl2Expr::SearchLiteral(s))
            }
            TokenKind::Dollar => {
                self.advance();
                // $identifier パターン
                if let Some(name) = self.expect_identifier() {
                    // ラムダ式の判定: $param -> body
                    if matches!(self.peek(), TokenKind::Arrow) {
                        self.advance(); // ->
                        let body = self.parse_expression()?;
                        return Some(Spl2Expr::Lambda {
                            params: vec![name],
                            body: Box::new(body),
                        });
                    }
                    Some(Spl2Expr::ParameterRef(name))
                } else {
                    None
                }
            }
            TokenKind::LParen => {
                self.advance();

                // ラムダ式の判定: ($param, ...) -> body
                if self.is_lambda_params() {
                    let params = self.parse_lambda_params()?;
                    self.expect(&TokenKind::RParen)?;
                    self.expect(&TokenKind::Arrow)?;
                    let body = self.parse_expression()?;
                    return Some(Spl2Expr::Lambda {
                        params,
                        body: Box::new(body),
                    });
                }

                let expr = self.parse_expression()?;
                self.expect(&TokenKind::RParen)?;
                Some(Spl2Expr::Grouped(Box::new(expr)))
            }
            TokenKind::LBracket => {
                self.advance();
                // サブクエリ vs 配列リテラルの判定
                if self.is_subquery_start() {
                    let query = self.parse_query()?;
                    self.expect(&TokenKind::RBracket)?;
                    Some(Spl2Expr::SubQuery(Box::new(query)))
                } else {
                    // 配列リテラル
                    let mut elements = Vec::new();
                    if !matches!(self.peek(), TokenKind::RBracket) {
                        elements.push(self.parse_expression()?);
                        while matches!(self.peek(), TokenKind::Comma) {
                            self.advance();
                            if matches!(self.peek(), TokenKind::RBracket) {
                                break;
                            }
                            elements.push(self.parse_expression()?);
                        }
                    }
                    self.expect(&TokenKind::RBracket)?;
                    Some(Spl2Expr::ArrayLiteral(elements))
                }
            }
            TokenKind::LBrace => {
                self.advance();
                // オブジェクトリテラル
                let mut entries = Vec::new();
                if !matches!(self.peek(), TokenKind::RBrace) {
                    let key = self.expect_identifier()?;
                    self.expect(&TokenKind::Colon)?;
                    let value = self.parse_expression()?;
                    entries.push((key, value));
                    while matches!(self.peek(), TokenKind::Comma) {
                        self.advance();
                        if matches!(self.peek(), TokenKind::RBrace) {
                            break;
                        }
                        let key = self.expect_identifier()?;
                        self.expect(&TokenKind::Colon)?;
                        let value = self.parse_expression()?;
                        entries.push((key, value));
                    }
                }
                self.expect(&TokenKind::RBrace)?;
                Some(Spl2Expr::ObjectLiteral(entries))
            }
            TokenKind::Identifier(name) => {
                self.advance();
                // 関数呼び出しの判定
                if matches!(self.peek(), TokenKind::LParen) {
                    let call = self.parse_function_call(name)?;
                    Some(Spl2Expr::FunctionCall(call))
                } else {
                    Some(Spl2Expr::Field(name))
                }
            }
            TokenKind::Error(msg) => {
                self.errors.push(ParseError::new(
                    format!("lexer error: {}", msg),
                    self.current_span(),
                ));
                self.advance();
                None
            }
            _ if self.is_keyword_as_identifier() => {
                let name = self.keyword_to_string();
                self.advance();
                // 関数呼び出しの判定
                if matches!(self.peek(), TokenKind::LParen) {
                    let call = self.parse_function_call(name)?;
                    Some(Spl2Expr::FunctionCall(call))
                } else {
                    Some(Spl2Expr::Field(name))
                }
            }
            _ => {
                self.errors.push(ParseError::new(
                    format!("unexpected token: {:?}", self.peek()),
                    self.current_span(),
                ));
                None
            }
        }
    }

    fn parse_function_call(&mut self, name: String) -> Option<Spl2FunctionCall> {
        let start = self.current_span().start - name.len();
        self.advance(); // ( を消費します

        let mut arguments = Vec::new();
        if !matches!(self.peek(), TokenKind::RParen) {
            // 名前付き引数の検出
            if self.is_named_arg() {
                let arg_name = match self.peek().clone() {
                    TokenKind::Identifier(n) => n,
                    _ => self.keyword_to_string(),
                };
                self.advance();
                self.advance(); // =
                let value = self.parse_expression()?;
                arguments.push(Spl2FunctionArg::Named {
                    name: arg_name,
                    value,
                });
            } else {
                arguments.push(Spl2FunctionArg::Positional(self.parse_expression()?));
            }
            while matches!(self.peek(), TokenKind::Comma) {
                self.advance();
                if matches!(self.peek(), TokenKind::RParen) {
                    break;
                }
                if self.is_named_arg() {
                    let arg_name = match self.peek().clone() {
                        TokenKind::Identifier(n) => n,
                        _ => self.keyword_to_string(),
                    };
                    self.advance();
                    self.advance(); // =
                    let value = self.parse_expression()?;
                    arguments.push(Spl2FunctionArg::Named {
                        name: arg_name,
                        value,
                    });
                } else {
                    arguments.push(Spl2FunctionArg::Positional(self.parse_expression()?));
                }
            }
        }

        let rparen = self.expect(&TokenKind::RParen)?;
        let end = rparen.span.end;

        Some(Spl2FunctionCall {
            name,
            arguments,
            span: Span::new(start, end),
        })
    }

    /// サブクエリの開始かどうかを判定します。
    fn is_subquery_start(&self) -> bool {
        matches!(
            self.peek(),
            TokenKind::From | TokenKind::Select | TokenKind::Pipe
        ) || {
            // コマンド名 (既知コマンドまたは identifier の後にパイプや引数が続く場合)
            if let TokenKind::Identifier(name) = self.peek() {
                crate::spl2::linter::known_commands::is_known_spl2_command(name)
            } else {
                false
            }
        }
    }

    /// ラムダパラメータのリストかどうかを判定します。
    fn is_lambda_params(&self) -> bool {
        // ($param, ...) -> body パターンの検出
        if !matches!(self.peek(), TokenKind::Dollar) {
            return false;
        }
        // 前方参照でパターンを確認します
        let mut i = 0;
        loop {
            // $
            if !matches!(self.peek_ahead(i), TokenKind::Dollar) {
                return false;
            }
            i += 1;
            // identifier
            if !matches!(self.peek_ahead(i), TokenKind::Identifier(_)) {
                return false;
            }
            i += 1;
            // , が続く場合
            if matches!(self.peek_ahead(i), TokenKind::Comma) {
                i += 1;
                continue;
            }
            // ) -> が続く場合
            if matches!(self.peek_ahead(i), TokenKind::RParen) {
                return matches!(self.peek_ahead(i + 1), TokenKind::Arrow);
            }
            return false;
        }
    }

    fn parse_lambda_params(&mut self) -> Option<Vec<String>> {
        let mut params = Vec::new();
        self.expect(&TokenKind::Dollar)?;
        params.push(self.expect_identifier()?);
        while matches!(self.peek(), TokenKind::Comma) {
            self.advance();
            self.expect(&TokenKind::Dollar)?;
            params.push(self.expect_identifier()?);
        }
        Some(params)
    }

    fn prev_end(&self) -> usize {
        if self.pos > 0 {
            self.tokens
                .get(self.pos - 1)
                .map(|t| t.span.end)
                .unwrap_or(0)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spl2::lexer::Lexer;

    fn parse(input: &str) -> (Option<Spl2Query>, Vec<ParseError>) {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let parser = Parser::new(tokens);
        parser.parse()
    }

    #[test]
    fn test_empty_input() {
        let (query, errors) = parse("");
        assert!(query.is_none());
        assert!(errors.is_empty());
    }

    #[test]
    fn test_from_statement() {
        let (query, errors) = parse("FROM main WHERE status=200");
        assert!(errors.is_empty(), "errors: {:?}", errors);
        let query = query.unwrap();
        assert_eq!(query.stages.len(), 1);
        if let Spl2StageKind::FromStatement(ref stmt) = query.stages[0].kind {
            assert_eq!(stmt.dataset, "main");
            assert!(stmt.where_clause.is_some());
        } else {
            panic!("expected FromStatement");
        }
    }

    #[test]
    fn test_from_with_pipeline() {
        let (query, errors) = parse("FROM main WHERE status=200 | stats count() BY host");
        assert!(errors.is_empty(), "errors: {:?}", errors);
        let query = query.unwrap();
        assert_eq!(query.stages.len(), 2);
        assert!(matches!(
            query.stages[0].kind,
            Spl2StageKind::FromStatement(_)
        ));
        assert!(matches!(query.stages[1].kind, Spl2StageKind::Command(_)));
    }

    #[test]
    fn test_select_statement() {
        let (query, errors) =
            parse("SELECT count() AS cnt, host FROM main WHERE status=200 GROUP BY host");
        assert!(errors.is_empty(), "errors: {:?}", errors);
        let query = query.unwrap();
        assert_eq!(query.stages.len(), 1);
        if let Spl2StageKind::SelectStatement(ref stmt) = query.stages[0].kind {
            assert_eq!(stmt.items.len(), 2);
            assert_eq!(stmt.from, Some("main".to_string()));
            assert!(stmt.where_clause.is_some());
            assert!(stmt.group_by.is_some());
        } else {
            panic!("expected SelectStatement");
        }
    }

    #[test]
    fn test_command() {
        let (query, errors) = parse("FROM main | eval total = count * 2");
        assert!(errors.is_empty(), "errors: {:?}", errors);
        let query = query.unwrap();
        assert_eq!(query.stages.len(), 2);
        if let Spl2StageKind::Command(ref cmd) = query.stages[1].kind {
            assert_eq!(cmd.name, "eval");
        } else {
            panic!("expected Command");
        }
    }

    #[test]
    fn test_null_expression() {
        let (query, errors) = parse("FROM main WHERE field IS NULL");
        assert!(errors.is_empty(), "errors: {:?}", errors);
        let query = query.unwrap();
        if let Spl2StageKind::FromStatement(ref stmt) = query.stages[0].kind {
            assert!(matches!(
                stmt.where_clause.as_deref(),
                Some(Spl2Expr::IsNull { negated: false, .. })
            ));
        } else {
            panic!("expected FromStatement");
        }
    }

    #[test]
    fn test_is_not_null() {
        let (query, errors) = parse("FROM main WHERE field IS NOT NULL");
        assert!(errors.is_empty(), "errors: {:?}", errors);
        let query = query.unwrap();
        if let Spl2StageKind::FromStatement(ref stmt) = query.stages[0].kind {
            assert!(matches!(
                stmt.where_clause.as_deref(),
                Some(Spl2Expr::IsNull { negated: true, .. })
            ));
        } else {
            panic!("expected FromStatement");
        }
    }

    #[test]
    fn test_in_list() {
        let (query, errors) = parse("FROM main WHERE status IN (200, 201, 204)");
        assert!(errors.is_empty(), "errors: {:?}", errors);
        let query = query.unwrap();
        if let Spl2StageKind::FromStatement(ref stmt) = query.stages[0].kind {
            assert!(matches!(
                stmt.where_clause.as_deref(),
                Some(Spl2Expr::InList { negated: false, .. })
            ));
        } else {
            panic!("expected FromStatement");
        }
    }

    #[test]
    fn test_between() {
        let (query, errors) = parse("FROM main WHERE status BETWEEN 200 AND 299");
        assert!(errors.is_empty(), "errors: {:?}", errors);
        let query = query.unwrap();
        if let Spl2StageKind::FromStatement(ref stmt) = query.stages[0].kind {
            assert!(matches!(
                stmt.where_clause.as_deref(),
                Some(Spl2Expr::Between { negated: false, .. })
            ));
        } else {
            panic!("expected FromStatement");
        }
    }

    #[test]
    fn test_like() {
        let (query, errors) = parse("FROM main WHERE host LIKE \"web%\"");
        assert!(errors.is_empty(), "errors: {:?}", errors);
        let query = query.unwrap();
        if let Spl2StageKind::FromStatement(ref stmt) = query.stages[0].kind {
            assert!(matches!(
                stmt.where_clause.as_deref(),
                Some(Spl2Expr::Like { negated: false, .. })
            ));
        } else {
            panic!("expected FromStatement");
        }
    }

    #[test]
    fn test_array_literal() {
        let (_query, errors) = parse("FROM main | eval arr = [1, 2, 3]");
        assert!(errors.is_empty(), "errors: {:?}", errors);
    }

    #[test]
    fn test_object_literal() {
        let (_query, errors) = parse("FROM main | eval obj = {key: \"value\", num: 42}");
        assert!(errors.is_empty(), "errors: {:?}", errors);
    }

    #[test]
    fn test_lambda() {
        let (_query, errors) = parse("FROM main | eval result = map(arr, $x -> $x + 1)");
        assert!(errors.is_empty(), "errors: {:?}", errors);
    }

    #[test]
    fn test_parameter_ref() {
        let (_query, errors) = parse("FROM main | eval x = $param + 1");
        assert!(errors.is_empty(), "errors: {:?}", errors);
    }

    #[test]
    fn test_function_call() {
        let (query, errors) = parse("FROM main | stats count(), sum(bytes) BY host");
        assert!(errors.is_empty(), "errors: {:?}", errors);
        let query = query.unwrap();
        if let Spl2StageKind::Command(ref cmd) = query.stages[1].kind {
            assert_eq!(cmd.name, "stats");
            assert!(cmd.by_clause.is_some());
        } else {
            panic!("expected Command");
        }
    }

    #[test]
    fn test_order_by() {
        let (query, errors) = parse("FROM main ORDER BY count DESC, host ASC");
        assert!(errors.is_empty(), "errors: {:?}", errors);
        let query = query.unwrap();
        if let Spl2StageKind::FromStatement(ref stmt) = query.stages[0].kind {
            let order = stmt.order_by.as_ref().unwrap();
            assert_eq!(order.len(), 2);
            assert_eq!(order[0].direction, SortDirection::Desc);
            assert_eq!(order[1].direction, SortDirection::Asc);
        } else {
            panic!("expected FromStatement");
        }
    }

    #[test]
    fn test_join() {
        let (query, errors) =
            parse("FROM main LEFT JOIN lookup_table ON main.host = lookup_table.host");
        assert!(errors.is_empty(), "errors: {:?}", errors);
        let query = query.unwrap();
        if let Spl2StageKind::FromStatement(ref stmt) = query.stages[0].kind {
            assert_eq!(stmt.joins.len(), 1);
            assert_eq!(stmt.joins[0].join_type, JoinType::Left);
        } else {
            panic!("expected FromStatement");
        }
    }

    #[test]
    fn test_search_literal() {
        let (_query, errors) = parse("FROM main WHERE `status=200 AND host=web*`");
        assert!(errors.is_empty(), "errors: {:?}", errors);
    }

    #[test]
    fn test_select_distinct() {
        let (query, errors) = parse("SELECT DISTINCT host FROM main");
        assert!(errors.is_empty(), "errors: {:?}", errors);
        let query = query.unwrap();
        if let Spl2StageKind::SelectStatement(ref stmt) = query.stages[0].kind {
            assert!(stmt.distinct);
        } else {
            panic!("expected SelectStatement");
        }
    }

    #[test]
    fn test_xor_expression() {
        let (query, errors) = parse("FROM main WHERE a XOR b");
        assert!(errors.is_empty(), "errors: {:?}", errors);
        let query = query.unwrap();
        if let Spl2StageKind::FromStatement(ref stmt) = query.stages[0].kind {
            assert!(matches!(
                stmt.where_clause.as_deref(),
                Some(Spl2Expr::Xor(_, _))
            ));
        } else {
            panic!("expected FromStatement");
        }
    }

    #[test]
    fn test_limit_offset() {
        let (query, errors) = parse("FROM main LIMIT 10 OFFSET 5");
        assert!(errors.is_empty(), "errors: {:?}", errors);
        let query = query.unwrap();
        if let Spl2StageKind::FromStatement(ref stmt) = query.stages[0].kind {
            assert!(stmt.limit.is_some());
            assert!(stmt.offset.is_some());
        } else {
            panic!("expected FromStatement");
        }
    }
}
