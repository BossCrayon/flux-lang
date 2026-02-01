use crate::token::{Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        } else {
            return self.input[self.read_position];
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        // v2.0 UPGRADE: Comment Skipping
        // If we see '//', we consume characters until the line ends
        if self.ch == '/' && self.peek_char() == '/' {
            self.skip_comment();
            return self.next_token();
        }

        if is_letter(self.ch) {
            let literal = self.read_identifier();
            let token_type = lookup_ident(&literal);
            return Token { token_type, literal };
        } else if is_digit(self.ch) {
            let literal = self.read_number();
            return Token { token_type: TokenType::Int, literal };
        }

        let tok = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    self.new_token(TokenType::Eq, "==")
                } else {
                    self.new_token(TokenType::Assign, "=")
                }
            },
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    self.new_token(TokenType::NotEq, "!=")
                } else {
                    self.new_token(TokenType::Illegal, "!")
                }
            },
            '"' => {
                let str_lit = self.read_string();
                self.new_token(TokenType::String, &str_lit)
            },
            '+' => self.new_token(TokenType::Plus, "+"),
            '-' => self.new_token(TokenType::Minus, "-"),
            '*' => self.new_token(TokenType::Asterisk, "*"),
            '/' => self.new_token(TokenType::Slash, "/"),
            '<' => self.new_token(TokenType::Lt, "<"),
            '>' => self.new_token(TokenType::Gt, ">"),
            ',' => self.new_token(TokenType::Comma, ","),
            ':' => self.new_token(TokenType::Colon, ":"),
            '(' => self.new_token(TokenType::LParen, "("),
            ')' => self.new_token(TokenType::RParen, ")"),
            '{' => self.new_token(TokenType::LBrace, "{"),
            '}' => self.new_token(TokenType::RBrace, "}"),
            '[' => self.new_token(TokenType::LBracket, "["),
            ']' => self.new_token(TokenType::RBracket, "]"),
            '\0' => self.new_token(TokenType::EOF, ""),
            _ => self.new_token(TokenType::Illegal, ""),
        };

        self.read_char();
        tok
    }

    fn skip_comment(&mut self) {
        // Keep reading until newline or end of file
        while self.ch != '\n' && self.ch != '\0' {
            self.read_char();
        }
        self.skip_whitespace();
    }

    fn read_string(&mut self) -> String {
        let pos = self.position + 1; 
        loop {
            self.read_char();
            if self.ch == '"' || self.ch == '\0' { break; }
        }
        self.input[pos..self.position].iter().collect()
    }

    fn new_token(&self, token_type: TokenType, literal: &str) -> Token {
        Token { token_type, literal: literal.to_string() }
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while is_letter(self.ch) { self.read_char(); }
        self.input[pos..self.position].iter().collect()
    }

    fn read_number(&mut self) -> String {
        let pos = self.position;
        while is_digit(self.ch) { self.read_char(); }
        self.input[pos..self.position].iter().collect()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() { self.read_char(); }
    }
}

fn is_letter(ch: char) -> bool { ch.is_alphabetic() || ch == '_' }
fn is_digit(ch: char) -> bool { ch.is_numeric() }

fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "fn" => TokenType::Fn,
        "mut" => TokenType::Mut,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "return" => TokenType::Return,
        "material" => TokenType::Material,
        "context" => TokenType::Context,
        "while" => TokenType::While,
        _ => TokenType::Identifier,
    }
}