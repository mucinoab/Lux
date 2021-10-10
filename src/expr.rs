use crate::token::Token;

// TODO name
pub type BExpr = Box<Expr>;

pub enum Expr {
    Binary { lhs: BExpr, tkn: Token, rhs: BExpr },
    Unary { operator: Token, rhs: BExpr },
    Literal(Value),
    Grouping(BExpr),
}

pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

pub fn print_ast(expr: &Expr) -> String {
    match expr {
        Expr::Binary { lhs, tkn, rhs } => parenthesize(&format!("{}", tkn), &[lhs, rhs]),
        Expr::Unary { operator, rhs } => parenthesize(&format!("{}", operator), &[rhs]),
        Expr::Literal(value) => match value {
            Value::String(v) => v.to_owned(),
            Value::Number(v) => v.to_string(),
            Value::Boolean(v) => v.to_string(),
            Value::Nil => String::from("nil"),
        },
        Expr::Grouping(e) => parenthesize("group", &[e]),
    }
}

fn parenthesize(name: &str, exprs: &[&Expr]) -> String {
    let mut result = format!("({}", name);

    for expr in exprs {
        result.push_str(&print_ast(expr));
    }

    result.push(')');

    result
}
