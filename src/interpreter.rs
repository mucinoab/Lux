use crate::{
    environment::Environment,
    errors::CompileError,
    expr::{Expr, Function, Value},
    statements::Statement,
    token::{Token, TokenType},
};

use std::{cell::RefCell, rc::Rc, time::SystemTime};

pub struct Interpreter {
    /// A pointer to the outermost global environment
    environment: Rc<RefCell<Environment>>,
    globals: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new(env: Environment) -> Self {
        let environment = Rc::new(RefCell::new(env));
        let globals = environment.clone();

        globals.borrow_mut().define(
            &Token {
                _type: TokenType::Fn,
                lexeme: "clock".into(),
                place: (0, 0),
                line: 0,
                column: 0,
            },
            Value::Callable(Function::Native {
                arity: 0,
                body: Box::new(|_| {
                    Value::Number(
                        SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_millis() as f64,
                    )
                }),
            }),
        );

        Self {
            globals,
            environment,
        }
    }

    pub fn default() -> Self {
        Self::new(Environment::default())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, CompileError> {
        // TODO Report error place, the place is in the token

        let value = match expr {
            Expr::Literal(value) => value.clone(),
            Expr::Grouping(group) => self.evaluate(group)?,
            Expr::Unary(op, rhs) => {
                let rhs = self.evaluate(rhs)?;

                // switch operator from token to tokenType
                match op._type {
                    TokenType::Minus => rhs.neg()?,
                    TokenType::Bang => Value::Boolean(!rhs.is_truthy()),
                    // TODO Report error
                    _ => unreachable!(),
                }
            }
            Expr::Binary(lhs, op, rhs) => {
                let lhs = self.evaluate(lhs)?;
                let rhs = self.evaluate(rhs)?;

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

                    _ => {
                        return Err(CompileError::Interpreter(
                            op.place,
                            "Unexpected operator".into(),
                        ))
                    }
                }
            }
            Expr::Logical(lhs, op, rhs) => {
                let left = self.evaluate(lhs)?;

                if op._type == TokenType::Or {
                    if left.is_truthy() {
                        return Ok(left);
                    }
                } else if !left.is_truthy() {
                    return Ok(left);
                }

                self.evaluate(rhs)?
            }
            Expr::Variable(name) => self.environment.borrow_mut().get(name)?,
            Expr::Assign(name, expr) => {
                let value = self.evaluate(expr)?;
                self.environment.borrow_mut().assign(name, value.clone())?;

                value
            }
            Expr::Call(calle, paren, unevaluated_args) => {
                let calle = self.evaluate(calle)?;

                let mut arguments = Vec::with_capacity(unevaluated_args.len());
                for arg in unevaluated_args {
                    arguments.push(self.evaluate(arg)?);
                }

                if let Value::Callable(f) = calle {
                    // TODO are function calls sound? Maybe.
                    match f {
                        Function::Native { arity, body } => {
                            check_arity(paren, arity, arguments.len())?;
                            body(&arguments)
                        }
                        Function::User {
                            name: _,
                            params,
                            body,
                            closure,
                        } => {
                            check_arity(paren, params.len(), arguments.len())?;

                            for (param, argument) in params.iter().zip(arguments.drain(..)) {
                                self.environment.borrow_mut().define(param, argument);
                            }

                            self.execute_block(&body, Some(closure.take()))?;
                            Value::Nil
                        }
                    }
                } else {
                    return Err(CompileError::Interpreter(
                        paren.place,
                        "Not a callable object.".into(),
                    ));
                }
            }
        };

        Ok(value)
    }

    pub fn interpret(&mut self, statements: &[Statement]) -> Result<(), CompileError> {
        for stmt in statements {
            match stmt {
                Statement::Print(value) => self.print_statement(value)?,
                Statement::Expresion(expr) => self.expresion_statement(expr)?,
                Statement::Var(token, expr) => {
                    let value = self.evaluate(expr)?;
                    self.environment.borrow_mut().define(token, value)
                }
                Statement::Block(statements) => self.execute_block(statements, None)?,
                Statement::If(condition, then_branch, maybe_else_branch) => {
                    if self.evaluate(condition)?.is_truthy() {
                        self.interpret(&[*then_branch.clone()])?;
                    } else if let Some(else_branch) = maybe_else_branch {
                        self.interpret(&[*else_branch.clone()])?;
                    }
                }
                Statement::While(condition, body) => {
                    while self.evaluate(condition)?.is_truthy() {
                        self.interpret(&[*body.clone()])?;
                    }
                }
                Statement::Function(name, params, body) => {
                    let function = Value::Callable(Function::User {
                        name: name.clone(),
                        params: params.clone(),
                        body: body.clone(),
                        closure: self.globals.clone(), // TODO: Point to the outer most environment
                    });

                    self.environment.borrow_mut().define(name, function);
                }
            }
        }

        Ok(())
    }

    fn expresion_statement(&mut self, s: &Expr) -> Result<(), CompileError> {
        self.evaluate(s)?;

        Ok(())
    }

    fn print_statement(&mut self, s: &Expr) -> Result<(), CompileError> {
        let v = self.evaluate(s)?;
        println!("{}", v);
        // TODO flush buffer?

        Ok(())
    }

    fn execute_block(
        &mut self,
        statements: &[Statement],
        env: Option<Environment>,
    ) -> Result<(), CompileError> {
        self.environment.borrow_mut().push_scope(env);
        self.interpret(statements)?;
        self.environment.borrow_mut().pop_scope();

        Ok(())
    }
}

fn check_arity(fn_name: &Token, params: usize, arguments: usize) -> Result<(), CompileError> {
    if params != arguments {
        Err(CompileError::Interpreter(
            fn_name.place,
            format!("Expected {} arguments but got {}.", params, arguments),
        ))
    } else {
        Ok(())
    }
}
