use std::fmt;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

// 1. Define what can be a Key (Strings, Ints, Bools)
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum HashKey {
    Integer(i64),
    Boolean(bool),
    String(String),
}

// 2. The Main Object Enum (Added Hash variant)
#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    String(String),
    Return(Box<Object>),
    Error(String),
    Null,
    Function {
        parameters: Vec<String>,
        body: crate::ast::BlockStatement,
        env: crate::environment::Environment,
    },
    Builtin(fn(Vec<Object>) -> Object),
    Array(Vec<Object>),
    // NEW: The Hash Map
    Hash(HashMap<HashKey, Object>), 
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Integer(val) => write!(f, "{}", val),
            Object::Boolean(val) => write!(f, "{}", val),
            Object::String(val) => write!(f, "{}", val),
            Object::Return(val) => write!(f, "{}", val),
            Object::Error(val) => write!(f, "ERROR: {}", val),
            Object::Null => write!(f, "null"),
            Object::Function { .. } => write!(f, "fn(...)"),
            Object::Builtin(_) => write!(f, "[builtin function]"),
            Object::Array(elements) => {
                let params: Vec<String> = elements.iter().map(|e| e.to_string()).collect();
                write!(f, "[{}]", params.join(", "))
            },
            // NEW: Print format for Hashes
            Object::Hash(pairs) => {
                let mut str_pairs = Vec::new();
                for (key, value) in pairs {
                    let key_str = match key {
                        HashKey::Integer(i) => i.to_string(),
                        HashKey::Boolean(b) => b.to_string(),
                        HashKey::String(s) => format!("\"{}\"", s), // Quote string keys
                    };
                    str_pairs.push(format!("{}: {}", key_str, value));
                }
                write!(f, "{{{}}}", str_pairs.join(", "))
            },
        }
    }
}

// Helper: Try to convert an Object into a HashKey
pub fn get_hash_key(obj: &Object) -> Option<HashKey> {
    match obj {
        Object::Integer(i) => Some(HashKey::Integer(*i)),
        Object::Boolean(b) => Some(HashKey::Boolean(*b)),
        Object::String(s) => Some(HashKey::String(s.clone())),
        _ => None,
    }
}