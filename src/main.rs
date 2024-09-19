mod lox;

use lox::lexer::Lexer;
use std::{
    env, fs,
    io::{self, Write},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<_>>();

    match args.get(0).map(|s| s.as_str()) {
        Some("target/debug/rslox") | Some("target/release/rslox") | Some("rslox") => {}
        _ => panic!("Expected the first argument to be the program name"),
    };

    match args.len() {
        1 => run_prompt()?,
        2 => run_file(&args[1])?,
        _ => eprintln!("Usage: rslox [script]"),
    };

    Ok(())
}

fn run_prompt() -> Result<(), io::Error> {
    loop {
        io::stdout().write_all(b"> ")?;
        io::stdout().flush()?;

        let mut source = String::new();
        io::stdin().read_line(&mut source)?;

        if let Err(e) = run(&source) {
            writeln!(io::stdout(), "{}", e)?;
        }
    }
}

fn run_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(path)?;
    run(&source)?;
    Ok(())
}

fn run(source: &str) -> Result<(), Box<dyn std::error::Error>> {
    for possible_token in Lexer::new(source) {
        match possible_token {
            Ok(token) => writeln!(io::stdout(), "{}", token)?,
            Err(error) => writeln!(io::stderr(), "{}", error)?,
        }
    }

    Ok(())
}
