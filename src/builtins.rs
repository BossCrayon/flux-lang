use std::collections::HashMap;
use crate::object::Object;
use std::io::{self, Write};
use std::fs;
// Necessary imports for the "Import" system (Sub-Compiler)
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::environment::Environment;
use crate::evaluator::eval_program;

// This function registers all the "Standard Library" functions
pub fn new_environment() -> HashMap<String, Object> {
    let mut store = HashMap::new();
    
    // 1. System I/O
    store.insert("print".to_string(), Object::Builtin(print_fn));
    store.insert("input".to_string(), Object::Builtin(input_fn));
    
    // 2. Data Helpers
    store.insert("len".to_string(), Object::Builtin(len_fn));
    store.insert("int".to_string(), Object::Builtin(int_fn));
    
    // 3. File System
    store.insert("read_file".to_string(), Object::Builtin(read_file_fn));
    store.insert("write_file".to_string(), Object::Builtin(write_file_fn));
    
    // 4. Array Tools
    store.insert("push".to_string(), Object::Builtin(push_fn));
    store.insert("first".to_string(), Object::Builtin(first_fn));
    store.insert("last".to_string(), Object::Builtin(last_fn));
    store.insert("rest".to_string(), Object::Builtin(rest_fn));

    // 5. Module System
    store.insert("import".to_string(), Object::Builtin(import_fn));

    store
}

// --- STANDARD I/O ---

fn print_fn(args: Vec<Object>) -> Object {
    for arg in args {
        print!("{} ", arg);
    }
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

// --- DATA TOOLS ---

fn len_fn(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error("len() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Object::String(s) => Object::Integer(s.len() as i64),
        Object::Array(arr) => Object::Integer(arr.len() as i64),
        _ => Object::Error("argument to len() not supported".to_string()),
    }
}

fn int_fn(args: Vec<Object>) -> Object {
    if args.len() != 1 { return Object::Error("int() takes 1 arg".to_string()); }
    match &args[0] {
        Object::String(s) => match s.parse::<i64>() {
            Ok(val) => Object::Integer(val),
            Err(_) => Object::Error(format!("Could not convert '{}' to int", s)),
        },
        Object::Integer(i) => Object::Integer(*i),
        _ => Object::Error("Cannot convert to int".to_string()),
    }
}

// --- FILE SYSTEM ---

fn read_file_fn(args: Vec<Object>) -> Object {
    if args.len() != 1 { return Object::Error("read_file takes 1 arg (path)".to_string()); }
    if let Object::String(path) = &args[0] {
        match fs::read_to_string(path) {
            Ok(content) => Object::String(content),
            Err(_) => Object::String("".to_string()), // Return empty string if missing (Safe Mode)
        }
    } else {
        Object::Error("Argument must be a string path".to_string())
    }
}

fn write_file_fn(args: Vec<Object>) -> Object {
    if args.len() != 2 { return Object::Error("write_file takes 2 args (path, content)".to_string()); }
    
    let path = match &args[0] {
        Object::String(s) => s,
        _ => return Object::Error("First arg must be path string".to_string()),
    };
    
    let content = match &args[1] {
        Object::String(s) => s.clone(),
        Object::Integer(i) => i.to_string(),
        _ => return Object::Error("Second arg must be content string".to_string()),
    };

    match fs::write(path, content) {
        Ok(_) => Object::Boolean(true),
        Err(_) => Object::Boolean(false),
    }
}

// --- ARRAY TOOLS ---

fn push_fn(args: Vec<Object>) -> Object {
    if args.len() != 2 { return Object::Error("push takes 2 args (array, element)".to_string()); }
    match (&args[0], &args[1]) {
        (Object::Array(arr), val) => {
            let mut new_arr = arr.clone();
            new_arr.push(val.clone());
            Object::Array(new_arr)
        },
        _ => Object::Error("First argument to push must be ARRAY".to_string()),
    }
}

fn first_fn(args: Vec<Object>) -> Object {
    if args.len() != 1 { return Object::Error("first takes 1 arg".to_string()); }
    match &args[0] {
        Object::Array(arr) => {
            if arr.len() > 0 { arr[0].clone() } else { Object::Null }
        },
        _ => Object::Error("Argument must be array".to_string()),
    }
}

fn last_fn(args: Vec<Object>) -> Object {
    if args.len() != 1 { return Object::Error("last takes 1 arg".to_string()); }
    match &args[0] {
        Object::Array(arr) => {
            if arr.len() > 0 { arr[arr.len() - 1].clone() } else { Object::Null }
        },
        _ => Object::Error("Argument must be array".to_string()),
    }
}

fn rest_fn(args: Vec<Object>) -> Object {
    if args.len() != 1 { return Object::Error("rest takes 1 arg".to_string()); }
    match &args[0] {
        Object::Array(arr) => {
            if arr.len() > 0 { 
                // Return everything except the first element
                Object::Array(arr[1..].to_vec()) 
            } else { 
                Object::Null 
            }
        },
        _ => Object::Error("Argument must be array".to_string()),
    }
}

// --- MODULE SYSTEM (IMPORTS) ---

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
    // We call new_environment() recursively here. 
    // This allows modules to import other modules!
    let tools = new_environment();
    for (name, tool) in tools { env.set(name, tool); }

    let _result = eval_program(&program, &mut env);

    // 4. Return the Environment as a HashMap (Export all variables)
    env.to_hash()
}