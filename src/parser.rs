use crate::token::{Token, TokenType};
use crate::ast::{Statement, Expression, BlockStatement, HashLiteral};

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    Lowest, Equals, LessGreater, Sum, Product, Prefix, Call, Index,
}

fn token_precedence(t: &TokenType) -> Precedence {
    match t {
        TokenType::Eq | TokenType::NotEq => Precedence::Equals,
        TokenType::Lt | TokenType::Gt => Precedence::LessGreater,
        TokenType::Plus | TokenType::Minus => Precedence::Sum,
        TokenType::Slash | TokenType::Asterisk => Precedence::Product,
        TokenType::LParen => Precedence::Call,
        TokenType::LBracket => Precedence::Index,
        _ => Precedence::Lowest,
    }
}

pub struct Parser {
    l: crate::lexer::Lexer,
    cur_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(mut l: crate::lexer::Lexer) -> Parser {
        let cur_token = l.next_token();
        let peek_token = l.next_token();
        Parser { l, cur_token, peek_token, errors: vec![] }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Vec<Statement> {
        let mut program = vec![];
        while self.cur_token.token_type != TokenType::EOF {
            if let Some(stmt) = self.parse_statement() {
                program.push(stmt);
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.token_type {
            TokenType::Mut => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            // NEW: Check for Assignment (Identifier followed by =)
            TokenType::Identifier => {
                if self.peek_token.token_type == TokenType::Assign {
                    return self.parse_assignment_statement();
                }
                self.parse_expression_statement()
            },
            _ => self.parse_expression_statement(),
        }
    }

    // NEW FUNCTION
    fn parse_assignment_statement(&mut self) -> Option<Statement> {
        // We are currently on the Identifier
        let name = self.cur_token.literal.clone();
        
        self.next_token(); // Move to '='
        self.next_token(); // Move to Value

        let value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token.token_type == TokenType::Semicolon {
            self.next_token();
        }

        Some(Statement::Assign { name, value })
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        self.next_token(); 
        let name = match self.cur_token.token_type {
            TokenType::Identifier => self.cur_token.literal.clone(),
            _ => return None,
        };
        self.next_token();
        if self.cur_token.token_type != TokenType::Assign { return None; }
        self.next_token();
        let value = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token.token_type == TokenType::Semicolon { self.next_token(); }
        Some(Statement::Let { name, value })
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();
        let value = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token.token_type == TokenType::Semicolon { self.next_token(); }
        Some(Statement::Return(value))
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expr = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token.token_type == TokenType::Semicolon { self.next_token(); }
        Some(Statement::Expression(expr))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        // 1. Prefix
        let left = match self.cur_token.token_type {
            TokenType::Identifier => Some(Expression::Identifier(self.cur_token.literal.clone())),
            TokenType::Int => Some(Expression::IntegerLiteral(self.cur_token.literal.parse().unwrap_or(0))),
            TokenType::String => Some(Expression::StringLiteral(self.cur_token.literal.clone())),
            TokenType::True => Some(Expression::Boolean(true)),
            TokenType::False => Some(Expression::Boolean(false)),
            TokenType::Bang | TokenType::Minus => self.parse_prefix_expression(),
            TokenType::LParen => self.parse_grouped_expression(),
            TokenType::If => self.parse_if_expression(),
            TokenType::Fn => self.parse_function_literal(),
            TokenType::LBracket => self.parse_array_literal(),
            TokenType::LBrace => self.parse_hash_literal(),
            TokenType::While => self.parse_while_expression(),
            _ => None,
        };

        if left.is_none() { return None; }
        let mut left_expr = left.unwrap();

        // 2. Infix
        while self.peek_token.token_type != TokenType::Semicolon && precedence < token_precedence(&self.peek_token.token_type) {
            match self.peek_token.token_type {
                TokenType::Plus | TokenType::Minus | TokenType::Slash | TokenType::Asterisk |
                TokenType::Eq | TokenType::NotEq | TokenType::Lt | TokenType::Gt => {
                    self.next_token();
                    left_expr = self.parse_infix_expression(left_expr)?;
                },
                TokenType::LParen => {
                    self.next_token();
                    left_expr = self.parse_call_expression(left_expr)?;
                },
                TokenType::LBracket => {
                    self.next_token();
                    left_expr = self.parse_index_expression(left_expr)?;
                },
                _ => return Some(left_expr),
            }
        }
        Some(left_expr)
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let operator = self.cur_token.literal.clone();
        self.next_token();
        let right = self.parse_expression(Precedence::Prefix)?;
        Some(Expression::Prefix { operator, right: Box::new(right) })
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let operator = self.cur_token.literal.clone();
        let precedence = token_precedence(&self.cur_token.token_type);
        self.next_token();
        let right = self.parse_expression(precedence)?;
        Some(Expression::Infix { left: Box::new(left), operator, right: Box::new(right) })
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();
        let exp = self.parse_expression(Precedence::Lowest);
        if self.peek_token.token_type == TokenType::RParen { self.next_token(); }
        exp
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        if !self.expect_peek(TokenType::LParen) { return None; }
        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(TokenType::RParen) { return None; }
        if !self.expect_peek(TokenType::LBrace) { return None; }
        let consequence = self.parse_block_statement();
        let mut alternative = None;
        if self.peek_token.token_type == TokenType::Else {
            self.next_token();
            if !self.expect_peek(TokenType::LBrace) { return None; }
            alternative = Some(self.parse_block_statement());
        }
        Some(Expression::If { condition: Box::new(condition), consequence, alternative })
    }

    fn parse_while_expression(&mut self) -> Option<Expression> {
        if !self.expect_peek(TokenType::LParen) { return None; }
        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(TokenType::RParen) { return None; }
        if !self.expect_peek(TokenType::LBrace) { return None; }
        let body = self.parse_block_statement();
        Some(Expression::While { condition: Box::new(condition), body })
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        self.next_token();
        let mut statements = vec![];
        while self.cur_token.token_type != TokenType::RBrace && self.cur_token.token_type != TokenType::EOF {
            if let Some(stmt) = self.parse_statement() { statements.push(stmt); }
            self.next_token();
        }
        BlockStatement { statements }
    }

    fn parse_function_literal(&mut self) -> Option<Expression> {
        if !self.expect_peek(TokenType::LParen) { return None; }
        let parameters = self.parse_function_parameters()?;
        if !self.expect_peek(TokenType::LBrace) { return None; }
        let body = self.parse_block_statement();
        Some(Expression::FunctionLiteral { parameters, body })
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<String>> {
        let mut identifiers = vec![];
        if self.peek_token.token_type == TokenType::RParen {
            self.next_token();
            return Some(identifiers);
        }
        self.next_token();
        identifiers.push(self.cur_token.literal.clone());
        while self.peek_token.token_type == TokenType::Comma {
            self.next_token();
            self.next_token();
            identifiers.push(self.cur_token.literal.clone());
        }
        if !self.expect_peek(TokenType::RParen) { return None; }
        Some(identifiers)
    }

    fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
        let arguments = self.parse_expression_list(TokenType::RParen)?;
        Some(Expression::Call { function: Box::new(function), arguments })
    }

    fn parse_array_literal(&mut self) -> Option<Expression> {
        let elements = self.parse_expression_list(TokenType::RBracket)?;
        Some(Expression::ArrayLiteral(elements))
    }

    fn parse_hash_literal(&mut self) -> Option<Expression> {
        let mut pairs = Vec::new();
        if self.peek_token.token_type == TokenType::RBrace {
            self.next_token(); self.next_token();
            return Some(Expression::HashLiteral(HashLiteral { pairs }));
        }
        self.next_token();
        loop {
            let key = self.parse_expression(Precedence::Lowest)?;
            if !self.expect_peek(TokenType::Colon) { return None; }
            self.next_token();
            let value = self.parse_expression(Precedence::Lowest)?;
            pairs.push((key, value));
            if self.peek_token.token_type == TokenType::RBrace { self.next_token(); break; }
            if !self.expect_peek(TokenType::Comma) { return None; }
            self.next_token();
        }
        Some(Expression::HashLiteral(HashLiteral { pairs }))
    }

    fn parse_expression_list(&mut self, end: TokenType) -> Option<Vec<Expression>> {
        let mut list = vec![];
        if self.peek_token.token_type == end {
            self.next_token();
            return Some(list);
        }
        self.next_token();
        list.push(self.parse_expression(Precedence::Lowest)?);
        while self.peek_token.token_type == TokenType::Comma {
            self.next_token();
            self.next_token();
            list.push(self.parse_expression(Precedence::Lowest)?);
        }
        if !self.expect_peek(end) { return None; }
        Some(list)
    }

    fn parse_index_expression(&mut self, left: Expression) -> Option<Expression> {
        self.next_token();
        let index = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(TokenType::RBracket) { return None; }
        Some(Expression::IndexExpression { left: Box::new(left), index: Box::new(index) })
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token.token_type == t {
            self.next_token();
            true
        } else {
            false
        }
    }
}