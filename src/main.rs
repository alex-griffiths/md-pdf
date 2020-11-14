use std::env;
use std::fs;
use std::process::exit;

mod lexer;

use lexer::lex;

struct Cli<'a> {
    file_path: &'a String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} file", &args[0]);
        exit(1);
    }

    let args = Cli {
        file_path: &args[1],
    };

    let md_file = fs::read_to_string(args.file_path);

    match md_file {
        Ok(content) => {
            // Lex file
            let tokens = lex(&content);
            println!("{:#?}", tokens);
        }
        Err(error) => {
            eprintln!("Could not find file: {}", args.file_path);
            exit(1);
        }
    }
}
