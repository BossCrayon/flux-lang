use std::io::{self, Write};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::evaluator::eval_program;
use crate::environment::Environment;
use crate::builtins;

const PROMPT: &str = ">> ";

pub fn start() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut env = Environment::new();

    // Load Tools ONCE so they persist between commands
    let tools = builtins::new_environment();
    for (name, tool) in tools {
        env.set(name, tool);
    }

    println!("Flux OS v0.6 (Interactive Shell)");
    println!("Type 'exit' to shut down.");
    println!("-------------------------------");

    loop {
        print!("{}", PROMPT);
        stdout.flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");

        if input.trim() == "exit" {
            println!("Shutting down...");
            break;
        }

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();

        if !p.errors.is_empty() {
            print_parser_errors(p.errors);
            continue;
        }

        let evaluated = eval_program(&program, &mut env);
        println!("{}", evaluated);
    }
}

fn print_parser_errors(errors: Vec<String>) {
    println!("  Whoops! We hit a snag:");
    for msg in errors {
        println!("\t{}", msg);
    }
}