use crate::{expr::Expr, token::Token};

pub enum Statement {
    Print(Expr),
    Expresion(Expr),
    Var(Token, Expr),
}
