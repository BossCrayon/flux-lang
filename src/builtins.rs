use crate::object::Object;
use std::collections::HashMap;

pub fn new_environment() -> HashMap<String, Object> {
    let mut builtins = HashMap::new();
    
    // Register "print"
    builtins.insert("print".to_string(), Object::Builtin(print_fn));
    
    // Register "render" (Mockup for Architect Mode)
    builtins.insert("render".to_string(), Object::Builtin(render_fn));
    
    builtins
}

// The 'print' function logic
fn print_fn(args: Vec<Object>) -> Object {
    for arg in args {
        println!("{}", arg);
    }
    Object::Null
}

// The 'render' function logic
fn render_fn(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error("render takes exactly 1 argument".to_string());
    }
    
    println!("--- RENDERING ENGINE STARTED ---");
    println!("Visualizing: {}", args[0]);
    println!("------------------------------");
    
    Object::Null
}