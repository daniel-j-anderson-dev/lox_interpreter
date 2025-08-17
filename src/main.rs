use lox::lexer::Lexer;
use std::{
    env, fs,
    io::{self, Write},
};

fn main() -> Result<(), io::Error> {
    let args = env::args().collect::<Vec<_>>();

    if !args.first().is_some_and(|s| s.ends_with("lox")) {
        panic!("Expected the first argument to be the program name")
    }

    match args.len() {
        1 => run_prompt()?,
        2 => run_file(&args[1])?,
        _ => eprintln!("Usage: lox [script]"),
    };

    Ok(())
}

fn run_prompt() -> Result<(), io::Error> {
    writeln!(io::stdout(), "Lox REPL. type \"exit\" to quit")?;
    loop {
        let source = get_input("> ")?;
        print_tokens(&source)?;
        if source.to_lowercase() == "exit" {
            break;
        }
    }
    Ok(())
}

fn run_file(path: &str) -> Result<(), io::Error> {
    let source = fs::read_to_string(path)?;
    print_tokens(&source)?;
    Ok(())
}

fn print_tokens(source: &str) -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    for possible_token in Lexer::new(source) {
        match possible_token {
            Ok(token) => writeln!(stdout, "{}", token)?,
            Err(error) => writeln!(stderr, "{}", error)?,
        }
    }
    Ok(())
}

fn get_input(prompt: &str) -> Result<String, io::Error> {
    {
        let mut stdout = io::stdout();
        stdout.write_all(prompt.as_bytes())?;
        stdout.flush()?;
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input.truncate(input.trim_end().len());

    Ok(input)
}
