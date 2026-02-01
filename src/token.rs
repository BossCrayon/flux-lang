#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Keywords
    Fn, Mut, Let, If, Else, Return,
    True, False,
    Context, Material, Render,
    While, // NEW: The Loop Keyword

    // Data
    Identifier, Int, Float, String,

    // Operators
    Assign, Plus, Minus, Asterisk, Slash,
    Eq, NotEq, Lt, Gt,
    Colon, Arrow,
    Comma, 

    // Structure
    LParen, RParen, 
    LBrace, RBrace,
    LBracket, RBracket,
    
    EOF, Illegal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}