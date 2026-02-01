mod token;
mod lexer;
mod ast;
mod parser;
mod object;
mod environment;
mod evaluator;
mod builtins;

use std::env;
use std::fs;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::environment::Environment;
use crate::evaluator::eval_program;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: flux_compiler [filename.flux]");
        return;
    }
    run_file(&args[1]);
}

fn run_file(filename: &str) {
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => { println!("Error reading file"); return; }
    };
    
    let l = Lexer::new(contents);
    let mut p = Parser::new(l);
    let program = p.parse_program();

    if !p.errors.is_empty() {
        println!("Parser Errors:");
        for msg in p.errors { println!("\t{}", msg); }
        return;
    }

    let mut env = Environment::new();
    let tools = builtins::new_environment();
    for (name, tool) in tools { env.set(name, tool); }
    
    let result = eval_program(&program, &mut env);
    if result != crate::object::Object::Null {
        println!("{}", result);
    }
}