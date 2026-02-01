use std::collections::HashMap;
use crate::object::Object;
use std::io::{self, Write};
use std::fs;

pub fn new_environment() -> HashMap<String, Object> {
    let mut store = HashMap::new();
    
    // 1. print(...)
    store.insert("print".to_string(), Object::Builtin(print_fn));
    
    // 2. input("Prompt"): Ask user for data
    store.insert("input".to_string(), Object::Builtin(input_fn));
    
    // 3. len(arr): Get size of array or string
    store.insert("len".to_string(), Object::Builtin(len_fn));
    
    // 4. int("10"): Convert string to number
    store.insert("int".to_string(), Object::Builtin(int_fn));

    // 5. File I/O
    store.insert("read_file".to_string(), Object::Builtin(read_file_fn));
    store.insert("write_file".to_string(), Object::Builtin(write_file_fn));

    store
}

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

fn read_file_fn(args: Vec<Object>) -> Object {
    if args.len() != 1 { return Object::Error("read_file takes 1 arg (path)".to_string()); }
    if let Object::String(path) = &args[0] {
        match fs::read_to_string(path) {
            Ok(content) => Object::String(content),
            Err(_) => Object::Error(format!("File not found: {}", path)),
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
        Object::String(s) => s,
        Object::Integer(i) => &i.to_string(),
        _ => return Object::Error("Second arg must be content string".to_string()),
    };

    match fs::write(path, content) {
        Ok(_) => Object::Boolean(true),
        Err(_) => Object::Boolean(false),
    }
}