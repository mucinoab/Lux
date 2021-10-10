mod errors;
mod expr;
mod parser;
mod scanner;
mod token;

use parser::Parser;

use crate::{expr::print_ast, scanner::*};

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

    let mut parser = Parser::new(tokens);
    let expr = parser.parse()?;

    println!("{}", print_ast(&expr));

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
