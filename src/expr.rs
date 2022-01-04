use crate::{environment::Environment, errors::CompileError, statements::Statement, token::Token};

use std::{
    cell::RefCell,
    fmt::{Debug, Display, Formatter},
    rc::Rc,
};

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Logical(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Literal(Value),
    Grouping(Box<Expr>),
    Variable(Token),
    Assign(Token, Box<Expr>),
    Call(Box<Expr>, Token, Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Callable(Function),
    Nil,
}

// TODO improve errors: report error in the correct place, not just "(0, 0)"
impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Nil => false,
            _ => true,
        }
    }

    pub fn neg(self) -> Result<Value, CompileError> {
        let val = match self {
            Value::Number(v) => Value::Number(-v),
            // TODO auto cast numbers?
            Value::String(s) => match s.parse::<f64>() {
                Ok(v) => Value::Number(-v),
                Err(_) => return Err(CompileError::Interpreter((0, 0), "Not a number".into())),
            },
            _ => {
                return Err(CompileError::Interpreter(
                    (0, 0),
                    "No - for the given value.".into(),
                ))
            }
        };

        Ok(val)
    }

    pub fn mul(self, rhs: Self) -> Result<Value, CompileError> {
        let err = Err(CompileError::Interpreter(
            (0, 0),
            "No Mul for the given value".into(),
        ));

        match self {
            Value::Number(lhs) => match rhs {
                Value::Number(rhs) => Ok(Value::Number(lhs * rhs)),
                _ => err,
            },
            _ => err,
        }
    }

    pub fn div(self, rhs: Self) -> Result<Value, CompileError> {
        let err = Err(CompileError::Interpreter(
            (0, 0),
            "No Div for the given value".into(),
        ));

        // TODO handle x/0

        match self {
            Value::Number(lhs) => match rhs {
                Value::Number(rhs) => Ok(Value::Number(lhs / rhs)),
                _ => err,
            },
            _ => err,
        }
    }

    pub fn sub(self, rhs: Self) -> Result<Value, CompileError> {
        let err = Err(CompileError::Interpreter(
            (0, 0),
            "No Sub for the given value".into(),
        ));

        match self {
            Value::Number(lhs) => match rhs {
                Value::Number(rhs) => Ok(Value::Number(lhs - rhs)),
                _ => err,
            },
            _ => err,
        }
    }

    pub fn add(self, rhs: Self) -> Result<Value, CompileError> {
        let error_msg = match self {
            Value::Number(lhs) => match rhs {
                Value::Number(rhs) => return Ok(Value::Number(lhs + rhs)),
                Value::String(_) => "No Add for Number and String",
                _ => "No Add for the given values",
            },

            Value::String(lhs) => match rhs {
                Value::String(rhs) => return Ok(Value::String(lhs + &rhs)),
                Value::Number(_) => "No Add for String and Number",
                _ => "No Add for the given values",
            },

            _ => "No Add for the given values",
        };

        Err(CompileError::Interpreter((0, 0), error_msg.into()))
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Value::String(s) => write!(f, "{}", s),
            Value::Number(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Callable(c) => write!(f, "{:?}", c),
            Value::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Clone)]
pub enum Function {
    Native {
        arity: usize,
        body: Box<fn(&[Value]) -> Value>,
    },
    User {
        name: Token,
        params: Vec<Token>,
        body: Vec<Statement>,
        closure: Rc<RefCell<Environment>>,
    },
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Function::Native { .. } => write!(f, "<native function>"),
            Function::User { name, .. } => write!(f, "<fn {}>", name.lexeme),
        }
    }
}

impl PartialOrd for Function {
    fn partial_cmp(&self, _o: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }

    fn lt(&self, _o: &Self) -> bool {
        todo!()
    }

    fn le(&self, _o: &Self) -> bool {
        todo!()
    }

    fn gt(&self, _o: &Self) -> bool {
        todo!()
    }

    fn ge(&self, _o: &Self) -> bool {
        todo!()
    }
}

impl PartialEq for Function {
    fn eq(&self, _o: &Self) -> bool {
        todo!()
    }
}
