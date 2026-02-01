use crate::ast::BlockStatement;
use crate::environment::Environment;
use std::fmt;

pub type BuiltinFunction = fn(Vec<Object>) -> Object;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
    Return(Box<Object>),
    Material { name: String },
    Error(String),
    Function {
        parameters: Vec<String>,
        body: BlockStatement,
        env: Environment,
    },
    Builtin(BuiltinFunction),
    Array(Vec<Object>), 
    
    // NEW: String Object
    String(String),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Integer(val) => write!(f, "{}", val),
            Object::Boolean(val) => write!(f, "{}", val),
            Object::Null => write!(f, "null"),
            Object::Return(val) => write!(f, "{}", val),
            Object::Material { name } => write!(f, "Material({})", name),
            Object::Error(msg) => write!(f, "ERROR: {}", msg),
            Object::Function { .. } => write!(f, "fn(...)"),
            Object::Builtin(_) => write!(f, "builtin function"),
            Object::Array(elements) => {
                let mut strs = vec![];
                for el in elements { strs.push(format!("{}", el)); }
                write!(f, "[{}]", strs.join(", "))
            },
            
            // NEW: Print String
            Object::String(val) => write!(f, "{}", val),
        }
    }
}