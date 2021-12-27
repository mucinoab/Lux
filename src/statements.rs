use crate::{expr::Expr, token::Token};

#[derive(Clone, Debug)]
pub enum Statement {
    Print(Expr),
    Expresion(Expr),
    Var(Token, Expr),
    Block(Vec<Statement>),
    If(Expr, Box<Statement>, Option<Box<Statement>>),
    While(Expr, Box<Statement>),
}
