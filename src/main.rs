mod errors;
mod scanner;
mod token;

use crate::scanner::*;

use std::{
    env,
    fs::read_to_string,
    io::{self, stdout, Write},
};

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let args: Vec<_> = env::args().into_iter().collect();

    if args.len() > 2 {
        todo!();
    } else if args.len() == 2 {
        run_file(&args[0])?;
    } else {
        run_prompt()?;
    }

    Ok(())
}

fn run(source: &str) -> Result<(), Error> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        dbg!(token);
    }

    Ok(())
}

fn run_file(file: &str) -> Result<(), Error> {
    let source = read_to_string(file)?;
    run(&source)?;

    Ok(())
}

fn run_prompt() -> Result<(), Error> {
    let mut line = String::new();

    loop {
        print!(">>> ");
        stdout().flush()?;

        match io::stdin().read_line(&mut line) {
            Ok(_) => {
                if line.is_empty() {
                    return Ok(());
                } else {
                    run(&line)?;
                    line.clear();
                }
            }

            Err(e) => return Err(Box::new(e)),
        }
    }
}
