#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Illegal,
    EOF,

    // Identifiers + Literals
    Identifier,
    Int,
    String,

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    // Comparators
    Lt,
    Gt,
    Eq,
    NotEq,

    // Delimiters
    Comma,
    Colon,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    // Keywords
    Fn,
    Mut,
    True,
    False,
    If,
    Else,
    Return,
    While,
    
    // RESTORED TOKENS:
    Material,
    Context,
}