use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let { name: String, value: Expression },
    Return(Expression),
    Expression(Expression),
    Function { name: String, parameters: Vec<String>, body: BlockStatement },
}

#[derive(Debug, Clone, PartialEq)]
pub struct HashLiteral {
    pub pairs: Vec<(Expression, Expression)>, 
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(String),
    IntegerLiteral(i64),
    StringLiteral(String),
    Boolean(bool),
    Prefix { operator: String, right: Box<Expression> },
    Infix { left: Box<Expression>, operator: String, right: Box<Expression> },
    If { condition: Box<Expression>, consequence: BlockStatement, alternative: Option<BlockStatement> },
    FunctionLiteral { parameters: Vec<String>, body: BlockStatement },
    // NOTE: We use "Call" (not CallExpression)
    Call { function: Box<Expression>, arguments: Vec<Expression> },
    // NOTE: We use Tuple Variant for Array (ArrayLiteral(Vec...))
    ArrayLiteral(Vec<Expression>),
    IndexExpression { left: Box<Expression>, index: Box<Expression> },
    While { condition: Box<Expression>, body: BlockStatement },
    HashLiteral(HashLiteral), 
}

// Display Implementation (for debugging/printing)
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Identifier(s) => write!(f, "{}", s),
            Expression::IntegerLiteral(i) => write!(f, "{}", i),
            Expression::StringLiteral(s) => write!(f, "\"{}\"", s),
            Expression::Boolean(b) => write!(f, "{}", b),
            Expression::Prefix { operator, right } => write!(f, "({}{})", operator, right),
            Expression::Infix { left, operator, right } => write!(f, "({} {} {})", left, operator, right),
            Expression::If { .. } => write!(f, "if ..."),
            Expression::FunctionLiteral { .. } => write!(f, "fn(...)"),
            Expression::Call { function, .. } => write!(f, "{}(...)", function),
            Expression::ArrayLiteral(elements) => write!(f, "[{:?}]", elements),
            Expression::IndexExpression { left, index } => write!(f, "({}[{}])", left, index),
            Expression::While { .. } => write!(f, "while ..."),
            Expression::HashLiteral(_) => write!(f, "{{...}}"),
        }
    }
}