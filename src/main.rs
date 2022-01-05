mod environment;
mod errors;
mod expr;
mod interpreter;
mod parser;
mod scanner;
mod statements;
mod token;

use crate::{errors::error, interpreter::Interpreter, parser::Parser, scanner::*};

use std::{
    cmp::Ordering,
    env,
    fs::read_to_string,
    io::{stdin, stdout, Write},
};

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let args: Vec<_> = env::args().collect();

    match args.len().cmp(&2) {
        Ordering::Less => run_prompt()?,
        Ordering::Equal => run_file(&args[1])?,
        Ordering::Greater => todo!(),
    }

    Ok(())
}

fn run_lines(file_name: &str, scanner: &mut Scanner, interpreter: &mut Interpreter) {
    match scanner.scan_tokens() {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens);

            match parser.parse() {
                Ok(expr) => {
                    if let Err(e) = interpreter.interpret(&expr) {
                        error(file_name, scanner.source_raw, &[e]);
                    }
                }
                Err(e) => error(file_name, scanner.source_raw, &[e]),
            }
        }

        Err(errors) => error(file_name, scanner.source_raw, &errors),
    }
}

fn run(file_name: &str, source: &str) {
    let mut scanner = Scanner::new(source);
    let mut interpreter = Interpreter::default();

    run_lines(file_name, &mut scanner, &mut interpreter)
}

fn run_file(file: &str) -> Result<(), Error> {
    let source = read_to_string(file)?;
    run(file, &source);

    Ok(())
}

fn run_prompt() -> Result<(), Error> {
    let mut line = String::new();
    let mut interpreter = Interpreter::default();
    let stdin = stdin();

    loop {
        print!(">>> ");
        stdout().flush()?;

        match stdin.read_line(&mut line) {
            Ok(_) => {
                let mut scanner = Scanner::new(&line);
                run_lines("repl", &mut scanner, &mut interpreter);
                line.clear();
            }

            Err(e) => return Err(Box::new(e)),
        }
    }
}
