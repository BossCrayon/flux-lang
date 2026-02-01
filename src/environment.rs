use std::collections::HashMap;
use crate::object::{Object, HashKey};

#[derive(Debug, PartialEq, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: Environment) -> Environment {
        Environment {
            store: HashMap::new(),
            outer: Some(Box::new(outer)),
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.store.get(name) {
            Some(obj) => Some(obj.clone()),
            None => match &self.outer {
                Some(outer) => outer.get(name),
                None => None,
            },
        }
    }

    pub fn set(&mut self, name: String, val: Object) -> Object {
        self.store.insert(name, val.clone());
        val
    }

    // NEW: Convert the Environment into a Hash Object
    // This allows us to return a "Module" as a simple HashMap of variables
    pub fn to_hash(&self) -> Object {
        let mut pairs = HashMap::new();
        for (key, value) in &self.store {
            let hash_key = HashKey::String(key.clone());
            pairs.insert(hash_key, value.clone());
        }
        Object::Hash(pairs)
    }
}