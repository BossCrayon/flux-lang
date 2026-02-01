use crate::ast::{Program, Statement, Expression, BlockStatement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    Lowest, Equals, LessGreater, Sum, Product, Prefix, Call, Index,
}

fn token_precedence(t: &TokenType) -> Precedence {
    match t {
        TokenType::Eq | TokenType::NotEq => Precedence::Equals,
        TokenType::Lt | TokenType::Gt => Precedence::LessGreater,
        TokenType::Plus | TokenType::Minus => Precedence::Sum,
        TokenType::Asterisk | TokenType::Slash => Precedence::Product,
        TokenType::LParen => Precedence::Call,
        TokenType::LBracket => Precedence::Index, 
        _ => Precedence::Lowest,
    }
}

pub struct Parser {
    l: Lexer,
    cur_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let mut p = Parser {
            l,
            cur_token: Token { token_type: TokenType::EOF, literal: "".to_string() },
            peek_token: Token { token_type: TokenType::EOF, literal: "".to_string() },
            errors: vec![],
        };
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };
        while self.cur_token.token_type != TokenType::EOF {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.token_type {
            TokenType::Mut => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let token = self.cur_token.clone();
        if !self.expect_peek(TokenType::Identifier) { return None; }
        let name = self.cur_token.literal.clone();
        if !self.expect_peek(TokenType::Assign) { return None; }
        self.next_token();
        let value = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token.token_type == TokenType::Colon { self.next_token(); }
        Some(Statement::Let { token, name, value })
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let token = self.cur_token.clone();
        self.next_token();
        let value = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token.token_type == TokenType::Colon { self.next_token(); }
        Some(Statement::Return { token, value })
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let token = self.cur_token.clone();
        let expression = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token.token_type == TokenType::Colon { self.next_token(); }
        Some(Statement::ExpressionStatement { token, expression })
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let mut left = match self.cur_token.token_type {
            TokenType::Int => Some(Expression::IntegerLiteral(self.cur_token.literal.parse().unwrap())),
            TokenType::True => Some(Expression::Boolean(true)),
            TokenType::False => Some(Expression::Boolean(false)),
            TokenType::Identifier => Some(Expression::Identifier(self.cur_token.literal.clone())),
            
            // NEW: Parse Strings
            TokenType::String => Some(Expression::StringLiteral(self.cur_token.literal.clone())),
            
            TokenType::If => self.parse_if_expression(),
            TokenType::Fn => self.parse_function_literal(),
            TokenType::Material => self.parse_material_expression(),
            TokenType::LBracket => self.parse_array_literal(),
            TokenType::While => self.parse_while_expression(),
            TokenType::LParen => self.parse_grouped_expression(),
            _ => None,
        };

        if left.is_none() { return None; }

        while self.peek_token.token_type != TokenType::EOF && precedence < token_precedence(&self.peek_token.token_type) {
            match self.peek_token.token_type {
                TokenType::LParen => {
                    self.next_token();
                    left = self.parse_call_expression(left.unwrap());
                },
                TokenType::LBracket => {
                    self.next_token();
                    left = self.parse_index_expression(left.unwrap());
                },
                TokenType::Plus | TokenType::Minus | TokenType::Asterisk | TokenType::Slash |
                TokenType::Eq | TokenType::NotEq | TokenType::Lt | TokenType::Gt => {
                    self.next_token();
                    let operator = self.cur_token.literal.clone();
                    let prec = token_precedence(&self.cur_token.token_type);
                    self.next_token();
                    let right = self.parse_expression(prec)?;
                    left = Some(Expression::Infix { left: Box::new(left?), operator, right: Box::new(right) });
                }
                _ => return left,
            }
        }
        left
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token(); 
        let exp = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(TokenType::RParen) { return None; }
        Some(exp)
    }

    fn parse_while_expression(&mut self) -> Option<Expression> {
        self.next_token(); 
        let condition = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(TokenType::LBrace) { return None; }
        let body = self.parse_block_statement();
        Some(Expression::While { condition: Box::new(condition), body })
    }

    fn parse_index_expression(&mut self, left: Expression) -> Option<Expression> {
        self.next_token();
        let index = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(TokenType::RBracket) { return None; }
        Some(Expression::IndexExpression { left: Box::new(left), index: Box::new(index) })
    }

    fn parse_array_literal(&mut self) -> Option<Expression> {
        let mut elements = vec![];
        if self.peek_token.token_type != TokenType::RBracket {
            self.next_token();
            if let Some(expr) = self.parse_expression(Precedence::Lowest) { elements.push(expr); }
            while self.peek_token.token_type == TokenType::Comma {
                self.next_token(); self.next_token();
                if let Some(expr) = self.parse_expression(Precedence::Lowest) { elements.push(expr); }
            }
        }
        if !self.expect_peek(TokenType::RBracket) { return None; }
        Some(Expression::ArrayLiteral { elements })
    }

    fn parse_material_expression(&mut self) -> Option<Expression> { if !self.expect_peek(TokenType::Identifier) { return None; } let name = self.cur_token.literal.clone(); Some(Expression::Material { name }) }
    fn parse_if_expression(&mut self) -> Option<Expression> { self.next_token(); let c = self.parse_expression(Precedence::Lowest)?; if !self.expect_peek(TokenType::LBrace) { return None; } let cons = self.parse_block_statement(); let mut alt = None; if self.peek_token.token_type == TokenType::Else { self.next_token(); if !self.expect_peek(TokenType::LBrace) { return None; } alt = Some(self.parse_block_statement()); } Some(Expression::If { condition: Box::new(c), consequence: cons, alternative: alt }) }
    fn parse_function_literal(&mut self) -> Option<Expression> { if !self.expect_peek(TokenType::LParen) { return None; } let mut p = vec![]; if self.peek_token.token_type != TokenType::RParen { self.next_token(); p.push(self.cur_token.literal.clone()); while self.peek_token.token_type == TokenType::Comma { self.next_token(); self.next_token(); p.push(self.cur_token.literal.clone()); } } if !self.expect_peek(TokenType::RParen) { return None; } if !self.expect_peek(TokenType::LBrace) { return None; } let b = self.parse_block_statement(); Some(Expression::FunctionLiteral { parameters: p, body: b }) }
    fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> { let mut args = vec![]; if self.peek_token.token_type != TokenType::RParen { self.next_token(); args.push(self.parse_expression(Precedence::Lowest)?); while self.peek_token.token_type == TokenType::Comma { self.next_token(); self.next_token(); args.push(self.parse_expression(Precedence::Lowest)?); } } if !self.expect_peek(TokenType::RParen) { return None; } Some(Expression::CallExpression { function: Box::new(function), arguments: args }) }
    fn parse_block_statement(&mut self) -> BlockStatement { let mut s = vec![]; self.next_token(); while self.cur_token.token_type != TokenType::RBrace && self.cur_token.token_type != TokenType::EOF { if let Some(st) = self.parse_statement() { s.push(st); } self.next_token(); } BlockStatement { statements: s } }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token.token_type == t {
            self.next_token();
            true
        } else {
            self.errors.push(format!("Expected {:?}, got {:?}", t, self.peek_token.token_type));
            false
        }
    }
}