use std::collections::HashMap;
use crate::object::Object;
use std::io::{self, Write};
use std::fs;
// Necessary imports to spawn a sub-compiler
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::environment::Environment;
use crate::evaluator::eval_program;

pub fn new_environment() -> HashMap<String, Object> {
    let mut store = HashMap::new();
    store.insert("print".to_string(), Object::Builtin(print_fn));
    store.insert("input".to_string(), Object::Builtin(input_fn));
    store.insert("len".to_string(), Object::Builtin(len_fn));
    store.insert("int".to_string(), Object::Builtin(int_fn));
    store.insert("read_file".to_string(), Object::Builtin(read_file_fn));
    store.insert("write_file".to_string(), Object::Builtin(write_file_fn));
    
    // NEW: The Import System
    store.insert("import".to_string(), Object::Builtin(import_fn));

    store
}

// --- Standard I/O ---

fn print_fn(args: Vec<Object>) -> Object {
    for arg in args { print!("{} ", arg); }
    println!("");
    Object::Null
}

fn input_fn(args: Vec<Object>) -> Object {
    if args.len() > 0 {
        print!("{}", args[0]);
        io::stdout().flush().unwrap();
    }
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    Object::String(buffer.trim().to_string())
}

// --- Data Tools ---

fn len_fn(args: Vec<Object>) -> Object {
    if args.len() != 1 { return Object::Error("len takes 1 arg".to_string()); }
    match &args[0] {
        Object::String(s) => Object::Integer(s.len() as i64),
        Object::Array(arr) => Object::Integer(arr.len() as i64),
        _ => Object::Error("unsupported type for len".to_string()),
    }
}

fn int_fn(args: Vec<Object>) -> Object {
    if args.len() != 1 { return Object::Error("int takes 1 arg".to_string()); }
    match &args[0] {
        Object::String(s) => match s.parse::<i64>() {
            Ok(v) => Object::Integer(v),
            Err(_) => Object::Error("parse error".to_string()),
        },
        Object::Integer(i) => Object::Integer(*i),
        _ => Object::Error("cannot cast to int".to_string()),
    }
}

// --- File System ---

fn read_file_fn(args: Vec<Object>) -> Object {
    if args.len() != 1 { return Object::Error("read_file takes 1 arg".to_string()); }
    if let Object::String(path) = &args[0] {
        match fs::read_to_string(path) {
            Ok(c) => Object::String(c),
            Err(_) => Object::String("".to_string()),
        }
    } else {
        Object::Error("path must be string".to_string())
    }
}

fn write_file_fn(args: Vec<Object>) -> Object {
    if args.len() != 2 { return Object::Error("write_file takes 2 args".to_string()); }
    let path = match &args[0] { Object::String(s) => s, _ => return Object::Error("path error".to_string()) };
    let content = match &args[1] { 
        Object::String(s) => s.clone(), 
        Object::Integer(i) => i.to_string(),
        _ => return Object::Error("content error".to_string()) 
    };
    match fs::write(path, content) {
        Ok(_) => Object::Boolean(true),
        Err(_) => Object::Boolean(false),
    }
}

// --- THE MODULE SYSTEM ---

fn import_fn(args: Vec<Object>) -> Object {
    if args.len() != 1 { return Object::Error("import takes 1 arg (filename)".to_string()); }
    
    let filename = match &args[0] {
        Object::String(s) => s,
        _ => return Object::Error("import path must be a string".to_string()),
    };

    // 1. Read the module file
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => return Object::Error(format!("Module '{}' not found", filename)),
    };

    // 2. Parse it
    let l = Lexer::new(contents);
    let mut p = Parser::new(l);
    let program = p.parse_program();

    if !p.errors.is_empty() {
        return Object::Error(format!("Parse errors in module {}: {:?}", filename, p.errors));
    }

    // 3. Evaluate it in a FRESH environment
    let mut env = Environment::new();
    
    // Inject standard tools so the module can use print/math/etc
    // (Recursively calls new_environment, so imports work inside imports!)
    let tools = new_environment();
    for (name, tool) in tools { env.set(name, tool); }

    let _result = eval_program(&program, &mut env);

    // 4. Export: Return the Environment as a HashMap
    env.to_hash()
}