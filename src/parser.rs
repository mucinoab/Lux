use crate::{
    errors::CompileError,
    expr::{BExpr, Expr, Value},
    statements::Statement,
    token::{Token, TokenType},
};

pub type CompResult = Result<BExpr, CompileError>;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, CompileError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
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

    fn expression(&mut self) -> CompResult {
        self.assignment()
    }

    fn equality(&mut self) -> CompResult {
        let mut expr = self.comparasion()?;

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            expr = Box::new(Expr::Binary {
                lhs: expr,
                op: self.previous().clone(),
                rhs: self.comparasion()?,
            });
        }

        Ok(expr)
    }

    fn comparasion(&mut self) -> CompResult {
        let mut expr = self.term()?;

        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            expr = Box::new(Expr::Binary {
                lhs: expr,
                op: self.previous().clone(),
                rhs: self.term()?,
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> CompResult {
        let mut expr = self.factor()?;

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            expr = Box::new(Expr::Binary {
                lhs: expr,
                op: self.previous().clone(),
                rhs: self.factor()?,
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> CompResult {
        let mut expr = self.unary()?;

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            expr = Box::new(Expr::Binary {
                lhs: expr,
                op: self.previous().clone(),
                rhs: self.unary()?,
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> CompResult {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            return Ok(Box::new(Expr::Unary {
                op: self.previous().clone(),
                rhs: self.unary()?,
            }));
        }

        self.primary()
    }

    fn primary(&mut self) -> CompResult {
        // TODO Tidy up this
        if self.matches(&[TokenType::False]) {
            return Ok(Box::new(Expr::Literal(Value::Boolean(false))));
        }

        if self.matches(&[TokenType::True]) {
            return Ok(Box::new(Expr::Literal(Value::Boolean(true))));
        }

        if self.matches(&[TokenType::Nil]) {
            return Ok(Box::new(Expr::Literal(Value::Nil)));
        }

        if self.matches(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;

            return Ok(Box::new(Expr::Grouping(expr)));
        }

        if self.matches(&[TokenType::Identifier]) {
            return Ok(Box::new(Expr::Variable(self.previous().clone())));
        }

        let tkn = self.peek();

        let expr = match &tkn._type {
            TokenType::String(s) => Box::new(Expr::Literal(Value::String(s.to_owned()))),
            TokenType::Number(n) => Box::new(Expr::Literal(Value::Number(*n))),
            _ => {
                return Err(CompileError::Parser(
                    tkn.place.0,
                    tkn.place.1,
                    "Unexpected token while parsing",
                ));
            }
        };

        self.advance();
        Ok(expr)
    }

    fn consume(&mut self, tkn: TokenType, error_msg: &'static str) -> Result<&Token, CompileError> {
        if self.check(&tkn) {
            Ok(self.advance())
        } else {
            //https://craftinginterpreters.com/parsing-expressions.html#entering-panic-mode
            let tkn = self.peek();
            Err(CompileError::Parser(tkn.place.0, tkn.place.1, error_msg))
        }
    }

    fn synchronize(&mut self) {
        // https://craftinginterpreters.com/parsing-expressions.html#entering-panic-mode
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

    fn statement(&mut self) -> Result<Statement, CompileError> {
        // TODO refactor to avoid repetitive code
        if self.matches(&[TokenType::Print]) {
            let value = self.expression()?;
            self.consume(TokenType::Semicolon, "Expect ';' after value.")?;

            Ok(Statement::Print(*value))
        } else {
            let value = self.expression()?;
            self.consume(TokenType::Semicolon, "Expect ';' after value.")?;

            Ok(Statement::Expresion(*value))
        }
    }

    fn declaration(&mut self) -> Result<Statement, CompileError> {
        if self.matches(&[TokenType::Var]) {
            return match self.var_declaration() {
                Ok(s) => Ok(s),
                Err(_) => {
                    self.synchronize();
                    Err(CompileError::Parser(0, 0, "TODO"))
                }
            };
        }

        self.statement()
    }

    fn var_declaration(&mut self) -> Result<Statement, CompileError> {
        let name = self
            .consume(TokenType::Identifier, "Expect variable name.")?
            .clone();

        let mut value = Expr::Literal(Value::Nil);

        if self.matches(&[TokenType::Equal]) {
            value = *self.expression()?;
        }

        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;

        Ok(Statement::Var(name, value))
    }

    fn assignment(&mut self) -> Result<BExpr, CompileError> {
        let expr = self.equality()?;

        if self.matches(&[TokenType::Equal]) {
            let value = self.assignment()?;

            if let Expr::Variable(v) = *expr {
                return Ok(Box::new(Expr::Assign(v, value)));
            }

            let equals = self.previous();

            return Err(CompileError::Parser(
                // TODO ?
                equals.place.0,
                equals.place.1,
                "Invalid assignment target",
            ));
        }

        Ok(expr)
    }
}
