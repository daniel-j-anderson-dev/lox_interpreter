use std::{
    env,
    error::Error,
    fs,
    io::{self, Write},
};

use lox::lexer::Lexer;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = env::args().collect::<Vec<_>>();

    if let Some(first_arg) = arguments.first()
        && first_arg.ends_with("lox")
    {
        Err("Expected the first argument to be the program name")?
    }

    match arguments.len() {
        1 => run_prompt()?,
        2 => run_file(&arguments[1])?,
        _ => eprintln!("Usage: lox [script]"),
    };

    Ok(())
}

fn run_prompt() -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    writeln!(stdout, "Lox REPL. type \"exit\" to quit")?;
    loop {
        let source = get_input("> ")?;
        print_tokens(&source, &mut stdout)?;
        if source.to_lowercase() == "exit" {
            break;
        }
    }
    Ok(())
}

fn run_file(path: &str) -> Result<(), io::Error> {
    let source = fs::read_to_string(path)?;
    print_tokens(&source, io::stdout())?;
    Ok(())
}

fn print_tokens(source: &str, mut output: impl Write) -> Result<(), io::Error> {
    for possible_token in Lexer::new(source) {
        match possible_token {
            Ok(token) => writeln!(output, "{}", token)?,
            Err(error) => writeln!(output, "{}", error)?,
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
