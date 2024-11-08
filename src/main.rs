use std::env;
use std::fs;
use std::io::{self, Write};

mod scanner;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let mut s = scanner::Scanner::new(&file_contents);
                let (tokens, errors) = s.scan_tokens();
                for err in errors {
                    eprintln!("{}", err);
                }
                for token in tokens {
                    println!("{}", token.to_string());
                }
                if !errors.is_empty() {
                    std::process::exit(65);
                }
            } else {
                println!("EOF  null");
            }
        }
        "parse" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });
            let mut s = scanner::Scanner::new(&file_contents);
            let (tokens, errors) = s.scan_tokens();
            if !errors.is_empty() {
                std::process::exit(65);
            }
            let exprs = match parser::Parser::new(tokens).parse() {
                Some(exprs) => exprs,
                None => Vec::new()
            };
            for expr in exprs {
                println!("{}", expr);
            };
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
