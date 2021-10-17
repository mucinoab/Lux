use crate::{
    errors::CompileError,
    expr::{Expr, Value},
    token::TokenType,
};

pub struct Interpreter;

impl Interpreter {
    fn evaluate(expr: Expr) -> Result<Value, CompileError> {
        // TODO Report error place, the place is in the token

        let value = match expr {
            Expr::Literal(v) => v,
            Expr::Grouping(g) => Interpreter::evaluate(*g)?,
            Expr::Unary { op, rhs } => {
                let rhs = Interpreter::evaluate(*rhs)?;

                // switch operator from token to tokenType
                match op._type {
                    TokenType::Minus => rhs.neg()?,
                    TokenType::Bang => Value::Boolean(!rhs.is_truthy()),
                    // TODO Report error
                    _ => unreachable!(),
                }
            }
            Expr::Binary { lhs, op, rhs } => {
                let lhs = Interpreter::evaluate(*lhs)?;
                let rhs = Interpreter::evaluate(*rhs)?;

                // switch operator from token to tokenType
                match op._type {
                    TokenType::Minus => lhs.sub(rhs)?,
                    TokenType::Plus => lhs.add(rhs)?,
                    TokenType::Slash => lhs.div(rhs)?,
                    TokenType::Star => lhs.mul(rhs)?,

                    TokenType::Greater => Value::Boolean(lhs > rhs),
                    TokenType::GreaterEqual => Value::Boolean(lhs >= rhs),
                    TokenType::Less => Value::Boolean(lhs < rhs),
                    TokenType::LessEqual => Value::Boolean(lhs <= rhs),

                    TokenType::Equal => Value::Boolean(lhs == rhs),
                    TokenType::BangEqual => Value::Boolean(lhs != rhs),

                    _ => return Err(CompileError::Interpreter(0, 0, "Unexpected op")),
                }
            }
        };

        Ok(value)
    }

    pub fn interpret(expr: Expr) -> Result<(), CompileError> {
        println!("{}", Interpreter::evaluate(expr)?);

        Ok(())
    }
}
