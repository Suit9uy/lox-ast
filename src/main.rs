mod error;
mod scanner;
use scanner::*;
mod token;
mod token_type;
use std::env::args;
use std::io::{self, BufRead};

use crate::error::LoxError;

pub fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        println!("Usage: lox-ast [Script]");

        std::process::exit(64);
    } else if args.len() == 1 {
        run_file(&args[1]).expect("Could not run file");
    } else {
        run_prompt();
    }

    println!("hello world, args is {:?}", args);
}

fn run_file(path: &String) -> io::Result<()> {
    let buf = std::fs::read_to_string(path)?;
    match run(buf) {
        Ok(_) => {}
        Err(m) => {
            m.report("".to_string());
            std::process::exit(65);
        }
    }

    Ok(())
}
fn run_prompt() {
    let stdin = io::stdin();
    println!("> ");
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            match run(line) {
                Ok(_) => {}
                Err(m) => {
                    m.report("".to_string());
                }
            }
        } else {
            break;
        };
    }
}

fn run(source: String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}

struct Lox {
    had_error: bool,
}
