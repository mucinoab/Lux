use crate::{
    expr::{BExpr, Expr, Value},
    token::{Token, TokenType},
    Error,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<BExpr, Error> {
        // TODO bubble up errors
        Ok(self.expression())
    }

    // Movments

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    // Helpers

    fn is_at_end(&self) -> bool {
        self.peek()._type == TokenType::Eof
    }

    fn check(&self, _type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        };

        &self.peek()._type == _type
    }

    fn matches(&mut self, tkns: &[TokenType]) -> bool {
        for tkn in tkns {
            if self.check(tkn) {
                self.advance();
                return true;
            }
        }

        false
    }

    // Rules

    fn expression(&mut self) -> BExpr {
        self.equality()
    }

    fn equality(&mut self) -> BExpr {
        let mut expr = self.comparasion();

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            expr = Box::new(Expr::Binary {
                lhs: expr,
                tkn: self.previous().clone(),
                rhs: self.comparasion(),
            });
        }

        expr
    }

    fn comparasion(&mut self) -> BExpr {
        let mut expr = self.term();

        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            expr = Box::new(Expr::Binary {
                lhs: expr,
                tkn: self.previous().clone(),
                rhs: self.term(),
            });
        }

        expr
    }

    fn term(&mut self) -> BExpr {
        let mut expr = self.factor();

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            expr = Box::new(Expr::Binary {
                lhs: expr,
                tkn: self.previous().clone(),
                rhs: self.factor(),
            });
        }

        expr
    }

    fn factor(&mut self) -> BExpr {
        let mut expr = self.unary();

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            expr = Box::new(Expr::Binary {
                lhs: expr,
                tkn: self.previous().clone(),
                rhs: self.unary(),
            });
        }

        expr
    }

    fn unary(&mut self) -> BExpr {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            return Box::new(Expr::Unary {
                operator: self.previous().clone(),
                rhs: self.unary(),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> BExpr {
        if self.matches(&[TokenType::False]) {
            return Box::new(Expr::Literal(Value::Boolean(true)));
        }

        if self.matches(&[TokenType::True]) {
            return Box::new(Expr::Literal(Value::Boolean(false)));
        }

        if self.matches(&[TokenType::Nil]) {
            return Box::new(Expr::Literal(Value::Nil));
        }

        if self.matches(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");

            return Box::new(Expr::Grouping(expr));
        }

        let expr = match &self.peek()._type {
            TokenType::String(s) => Box::new(Expr::Literal(Value::String(s.to_owned()))),
            TokenType::Number(n) => Box::new(Expr::Literal(Value::Number(*n))),
            _ => {
                // TODO proper error handling
                dbg!(self.peek());
                unreachable!()
            }
        };

        self.advance();
        expr
    }

    fn consume(&mut self, tkn: TokenType, error_msg: &str) -> &Token {
        if self.check(&tkn) {
            return self.advance();
        }

        // TODO proper error
        //https://craftinginterpreters.com/parsing-expressions.html#entering-panic-mode
        panic!("{}", error_msg);
        //Err(Box::new(std::fmt::Error))
    }

    /// https://craftinginterpreters.com/parsing-expressions.html#entering-panic-mode
    fn _synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous()._type == TokenType::Semicolon {
                return;
            }

            match self.peek()._type {
                TokenType::Class
                | TokenType::Fn
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }
}
