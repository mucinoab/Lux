use crate::{
    environment::Environment,
    errors::CompileError,
    expr::{Expr, Value},
    statements::Statement,
    token::TokenType,
};

#[derive(Default)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    fn evaluate(&mut self, expr: Expr) -> Result<Value, CompileError> {
        // TODO Report error place, the place is in the token

        let value = match expr {
            Expr::Literal(value) => value,
            Expr::Grouping(group) => self.evaluate(*group)?,
            Expr::Unary { op, rhs } => {
                let rhs = self.evaluate(*rhs)?;

                // switch operator from token to tokenType
                match op._type {
                    TokenType::Minus => rhs.neg()?,
                    TokenType::Bang => Value::Boolean(!rhs.is_truthy()),
                    // TODO Report error
                    _ => unreachable!(),
                }
            }
            Expr::Binary { lhs, op, rhs } => {
                let lhs = self.evaluate(*lhs)?;
                let rhs = self.evaluate(*rhs)?;

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

                    _ => return Err(CompileError::Interpreter(0, 0, "Unexpected operator")),
                }
            }
            Expr::Variable(name) => self.environment.get(&name)?,
            Expr::Assign(name, expr) => {
                let value = self.evaluate(*expr)?;
                self.environment.assign(&name.lexeme, value.clone())?;

                value
            }
        };

        Ok(value)
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<(), CompileError> {
        for stmt in statements {
            match stmt {
                Statement::Print(value) => self.print_statement(value)?,
                Statement::Expresion(expr) => self.expresion_statement(expr)?,
                Statement::Var(token, expr) => {
                    let value = self.evaluate(expr)?;
                    self.environment.define(&token.lexeme, value)
                }
            }
        }

        Ok(())
    }

    pub fn expresion_statement(&mut self, s: Expr) -> Result<(), CompileError> {
        self.evaluate(s)?;

        Ok(())
    }

    pub fn print_statement(&mut self, s: Expr) -> Result<(), CompileError> {
        let v = self.evaluate(s)?;
        println!("{}", v);
        // TODO flush buffer?

        Ok(())
    }
}
