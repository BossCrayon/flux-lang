mod token;
mod lexer;
mod ast;
mod parser;
mod object;
mod environment;
mod evaluator;
mod builtins;
mod code;
mod compiler; 
mod vm;

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

    // --- VM DEBUG START ---
    println!("--- VM DEBUG ---");
    
    // 1. Create a fake program: "1 + 2"
    let input = "if (true) { 10 } else { 20 }";
    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program();

    // 2. Compile it
    let mut comp = crate::compiler::Compiler::new();
    match comp.compile(program) {
        Ok(_) => {
            // 3. Run it in the VM!
            let mut machine = crate::vm::VM::new(comp);
            match machine.run() {
                Ok(_) => {
                    println!("VM Executed Successfully.");
                    // Check the stack. If you commented out OP_POP in compiler.rs, 
                    // this should show Some(Integer(3)).
                    println!("Stack Top: {:?}", machine.stack_top());
                },
                Err(e) => println!("VM Runtime Error: {}", e),
            }
        },
        Err(e) => println!("Compiler Error: {}", e),
    }
    println!("----------------");
    // --- VM DEBUG END ---

    // Now run the actual file using the old interpreter (for now)
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