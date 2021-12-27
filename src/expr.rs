use std::fmt::{Display, Formatter};

use crate::{errors::CompileError, token::Token};

// TODO name
pub type BExpr = Box<Expr>;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(BExpr, Token, BExpr),
    Logical(BExpr, Token, BExpr),
    Unary(Token, BExpr),
    Literal(Value),
    Grouping(BExpr),
    Variable(Token),
    Assign(Token, BExpr),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

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
                Err(_) => return Err(CompileError::Interpreter(0, 0, "Not a number")),
            },
            _ => return Err(CompileError::Interpreter(0, 0, "No - for the given value.")),
        };

        Ok(val)
    }

    pub fn mul(self, rhs: Self) -> Result<Value, CompileError> {
        let err = Err(CompileError::Interpreter(
            0,
            0,
            "No Mul for the given value",
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
            0,
            0,
            "No Div for the given value",
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
            0,
            0,
            "No Sub for the given value",
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

        Err(CompileError::Interpreter(0, 0, error_msg))
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Value::String(s) => write!(f, "{}", s),
            Value::Number(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
        }
    }
}

pub fn _print_ast(expr: &Expr) -> String {
    match expr {
        Expr::Binary(lhs, op, rhs) => _parenthesize(&format!("{}", op), &[lhs, rhs]),
        Expr::Unary(op, rhs) => _parenthesize(&format!("{}", op), &[rhs]),
        Expr::Literal(value) => match value {
            Value::String(v) => v.to_owned(),
            Value::Number(v) => v.to_string(),
            Value::Boolean(v) => v.to_string(),
            Value::Nil => String::from("nil"),
        },
        Expr::Grouping(e) => _parenthesize("group", &[e]),
        _ => todo!(),
    }
}

fn _parenthesize(name: &str, exprs: &[&Expr]) -> String {
    let mut result = format!("({}", name);

    for expr in exprs {
        result.push_str(&_print_ast(expr));
    }

    result.push(')');

    result
}
