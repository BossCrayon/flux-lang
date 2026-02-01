mod token;
mod lexer;
mod ast;
mod parser;
mod object;
mod evaluator;
mod environment;
mod builtins;
mod repl;

use std::env;
use std::fs;
use lexer::Lexer;
use parser::Parser;
use evaluator::eval_program;
use environment::Environment;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Mode 1: Run a File
        let filename = &args[1];
        run_file(filename);
    } else {
        // Mode 2: Interactive Shell
        repl::start();
    }
}

fn run_file(filename: &str) {
    // 1. Read file
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            println!("Error: Could not read file '{}'", filename);
            return;
        }
    };

    // 2. Setup Env
    let mut env = Environment::new();
    let tools = builtins::new_environment();
    for (name, tool) in tools {
        env.set(name, tool);
    }

    // 3. Compile
    let l = Lexer::new(contents);
    let mut p = Parser::new(l);
    let program = p.parse_program();

    if !p.errors.is_empty() {
        println!("Parser Errors:");
        for msg in p.errors {
            println!("\t{}", msg);
        }
        return;
    }

    // 4. Run & PRINT THE RESULT
    let result = eval_program(&program, &mut env);
    
    // The Fix: If the result isn't "Null" (empty), print it!
    if result != object::Object::Null {
        println!("{}", result);
    }
}