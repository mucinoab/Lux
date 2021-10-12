mod errors;
mod expr;
mod parser;
mod scanner;
mod token;

use crate::{errors::error, expr::print_ast, parser::Parser, scanner::*};

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
                Ok(expr) => println!("{}", print_ast(&expr)),
                Err(e) => error(file_name, source, &[e]),
            }
        }

        Err(errors) => error(file_name, source, &errors),
    }

    Ok(())
}

fn run_file(file: &str) -> Result<(), Error> {
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
    use crate::{
        expr::{print_ast, Expr, Value},
        token::{Token, TokenType},
    };

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn prints_ast() {
        let e: Expr = Expr::Binary {
            lhs: Box::new(Expr::Unary {
                operator: Token::new(TokenType::Minus, "-".into(), 1),
                rhs: Box::new(Expr::Literal(Value::Number(123.0))),
            }),
            tkn: Token::new(TokenType::Star, "*".into(), 1),
            rhs: Box::new(Expr::Grouping(Box::new(Expr::Literal(Value::Number(
                45.67,
            ))))),
        };

        assert!("(*(-123)(group45.67))" == print_ast(&e));
    }
}
