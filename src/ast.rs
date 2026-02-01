use crate::token::Token;

pub trait Node {
    fn string(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let { token: Token, name: String, value: Expression },
    Return { token: Token, value: Expression },
    ExpressionStatement { token: Token, expression: Expression },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    IntegerLiteral(i64),
    Boolean(bool),
    Identifier(String),
    Prefix { operator: String, right: Box<Expression> },
    Infix { left: Box<Expression>, operator: String, right: Box<Expression> },
    If { condition: Box<Expression>, consequence: BlockStatement, alternative: Option<BlockStatement> },
    FunctionLiteral { parameters: Vec<String>, body: BlockStatement },
    CallExpression { function: Box<Expression>, arguments: Vec<Expression> },
    Material { name: String },
    ArrayLiteral { elements: Vec<Expression> },
    IndexExpression { left: Box<Expression>, index: Box<Expression> },
    While { condition: Box<Expression>, body: BlockStatement },
    
    // NEW: String Support
    StringLiteral(String),
}

impl Node for Statement {
    fn string(&self) -> String {
        match self {
            Statement::Let { name, value, .. } => format!("mut {} = {};", name, value.string()),
            Statement::Return { value, .. } => format!("return {};", value.string()),
            Statement::ExpressionStatement { expression, .. } => expression.string(),
        }
    }
}

impl Node for Expression {
    fn string(&self) -> String {
        match self {
            Expression::IntegerLiteral(val) => val.to_string(),
            Expression::Boolean(val) => val.to_string(),
            Expression::Identifier(val) => val.clone(),
            Expression::Prefix { operator, right } => format!("({}{})", operator, right.string()),
            Expression::Infix { left, operator, right } => format!("({} {} {})", left.string(), operator, right.string()),
            Expression::If { .. } => "if ...".to_string(),
            Expression::FunctionLiteral { .. } => "fn(...) { ... }".to_string(),
            Expression::CallExpression { function, .. } => format!("{}(...)", function.string()),
            Expression::Material { name } => format!("material {}", name),
            Expression::ArrayLiteral { elements } => {
                let mut out = String::new();
                out.push('[');
                let mut strs = vec![];
                for el in elements { strs.push(el.string()); }
                out.push_str(&strs.join(", "));
                out.push(']');
                out
            },
            Expression::IndexExpression { left, index } => format!("({}[{}])", left.string(), index.string()),
            Expression::While { .. } => "while ...".to_string(),
            
            // NEW: Print String
            Expression::StringLiteral(val) => val.clone(),
        }
    }
}

#[derive(PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn string(&self) -> String {
        self.statements.iter().map(|s| s.string()).collect()
    }
}