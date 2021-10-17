mod errors;
mod expr;
mod interpreter;
mod parser;
mod scanner;
mod token;

use crate::{errors::error, expr::print_ast, interpreter::Interpreter, parser::Parser, scanner::*};

use std::{
    cmp::Ordering,
    env,
    fs::read_to_string,
    io::{self, stdout, Write},
};

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let args: Vec<_> = env::args().into_iter().collect();

    match args.len().cmp(&2) {
        Ordering::Greater => todo!(),
        Ordering::Less => run_prompt()?,
        Ordering::Equal => run_file(&args[1])?,
    }

    Ok(())
}

fn run(file_name: &str, source: &str) -> Result<(), Error> {
    let mut scanner = Scanner::new(source);

    match scanner.scan_tokens() {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens);

            match parser.parse() {
                Ok(expr) => {
                    println!("{}", print_ast(&expr));

                    if let Err(e) = Interpreter::interpret(*expr) {
                        error(file_name, source, &[e]);
                    }
                }
                Err(e) => error(file_name, source, &[e]),
            }
        }

        Err(errors) => error(file_name, source, &errors),
    }

    Ok(())
}

fn run_file(file: &str) -> Result<(), Error> {
    // TODO report errors
    let source = read_to_string(file)?;
    run(file, &source)?;

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
                    run("repl", &line)?;
                    line.clear();
                }
            }

            Err(e) => return Err(Box::new(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::Value;

    #[test]
    fn add() {
        let lhs = Value::Number(1.0);
        let rhs = Value::Number(2.0);

        assert_eq!(lhs.add(rhs).unwrap(), Value::Number(3.0));

        let lhs = Value::String(String::from("a"));
        let rhs = Value::String(String::from("b"));

        assert_eq!(lhs.add(rhs).unwrap(), Value::String(String::from("ab")));
    }
}
