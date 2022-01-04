use crate::{
    errors::CompileError,
    expr::{Expr, Value},
    statements::Statement,
    token::{Token, TokenType},
};

pub type CompResult = Result<Box<Expr>, CompileError>;

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
            expr = Box::new(Expr::Binary(
                expr,
                self.previous().clone(),
                self.comparasion()?,
            ));
        }

        Ok(expr)
    }

    fn or(&mut self) -> CompResult {
        let mut expr = self.and()?;

        while self.matches(&[TokenType::Or]) {
            let op = self.previous().clone();
            let rhs = self.and()?;
            expr = Box::new(Expr::Logical(expr, op, rhs));
        }

        Ok(expr)
    }

    fn and(&mut self) -> CompResult {
        let mut expr = self.equality()?;

        while self.matches(&[TokenType::And]) {
            let op = self.previous().clone();
            let rhs = self.equality()?;
            expr = Box::new(Expr::Logical(expr, op, rhs));
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
            expr = Box::new(Expr::Binary(expr, self.previous().clone(), self.term()?));
        }

        Ok(expr)
    }

    fn term(&mut self) -> CompResult {
        let mut expr = self.factor()?;

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            expr = Box::new(Expr::Binary(expr, self.previous().clone(), self.factor()?));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> CompResult {
        let mut expr = self.unary()?;

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            expr = Box::new(Expr::Binary(expr, self.previous().clone(), self.unary()?));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> CompResult {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            return Ok(Box::new(Expr::Unary(
                self.previous().clone(),
                self.unary()?,
            )));
        }

        self.call()
    }

    fn call(&mut self) -> CompResult {
        let mut expr = self.primary()?;

        loop {
            if self.matches(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
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
                    tkn.place,
                    "Unexpected token while parsing".into(),
                ));
            }
        };

        self.advance();
        Ok(expr)
    }

    fn consume(&mut self, tkn: TokenType, error_msg: &str) -> Result<&Token, CompileError> {
        if self.check(&tkn) {
            Ok(self.advance())
        } else {
            //https://craftinginterpreters.com/parsing-expressions.html#entering-panic-mode
            let tkn = self.peek();
            Err(CompileError::Parser(tkn.place, error_msg.into()))
        }
    }

    // TODO synchronization in errors
    fn _synchronize(&mut self) {
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
        // TODO Require explicit blocks { } for [if, loops, functions]
        if self.matches(&[TokenType::For]) {
            return self.for_statement();
        }

        if self.matches(&[TokenType::If]) {
            return self.if_statement();
        }

        if self.matches(&[TokenType::Print]) {
            return self.print_statement();
        }

        if self.matches(&[TokenType::While]) {
            return self.while_statement();
        }

        if self.matches(&[TokenType::LeftBrace]) {
            return Ok(Statement::Block(self.block()?));
        }

        self.expression_statement()
    }

    fn declaration(&mut self) -> Result<Statement, CompileError> {
        if self.matches(&[TokenType::Fn]) {
            self.function("function")
        } else if self.matches(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn function(&mut self, kind: &'static str) -> Result<Statement, CompileError> {
        let name = self
            .consume(TokenType::Identifier, &format!("Expect {} name", kind))?
            .clone();

        self.consume(
            TokenType::LeftParen,
            &format!("Expect '(' after {}  name.", kind),
        )?;

        let mut parameters = Vec::new();

        if !self.check(&TokenType::RightParen) {
            parameters.push(
                self.consume(TokenType::Identifier, "Expect parameter name.")?
                    .clone(),
            );

            while self.matches(&[TokenType::Comma]) {
                // TODO add maximum limit of parameters
                parameters.push(
                    self.consume(TokenType::Identifier, "Expect parameter name.")?
                        .clone(),
                );
            }
        }

        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;
        self.consume(
            TokenType::LeftBrace,
            &format!("Expect '{{' before {} body.", kind),
        )?;

        Ok(Statement::Function(name, parameters, self.block()?))
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

    fn assignment(&mut self) -> Result<Box<Expr>, CompileError> {
        let expr = self.or()?;

        if self.matches(&[TokenType::Equal]) {
            let value = self.assignment()?;

            if let Expr::Variable(v) = *expr {
                return Ok(Box::new(Expr::Assign(v, value)));
            }

            let equals = self.previous();

            return Err(CompileError::Parser(
                equals.place,
                "Invalid assignment target".into(),
            ));
        }

        Ok(expr)
    }

    fn block(&mut self) -> Result<Vec<Statement>, CompileError> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;

        Ok(statements)
    }

    fn print_statement(&mut self) -> Result<Statement, CompileError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;

        Ok(Statement::Print(*value))
    }

    fn if_statement(&mut self) -> Result<Statement, CompileError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition")?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.matches(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Statement::If(*condition, then_branch, else_branch))
    }

    fn while_statement(&mut self) -> Result<Statement, CompileError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition")?;

        Ok(Statement::While(*condition, Box::new(self.statement()?)))
    }

    fn for_statement(&mut self) -> Result<Statement, CompileError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let init = if self.matches(&[TokenType::Semicolon]) {
            None
        } else if self.matches(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.matches(&[TokenType::Semicolon]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let increment = if !self.matches(&[TokenType::RightParen]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expect ')' after loop clauses.")?;

        // Desugaring: For -> While
        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = Statement::Block(vec![body, Statement::Expresion(*increment)]);
        }

        body = Statement::While(
            *condition.unwrap_or_else(|| Box::new(Expr::Literal(Value::Boolean(true)))),
            Box::new(body),
        );

        if let Some(init) = init {
            body = Statement::Block(vec![init, body]);
        }

        Ok(body)
    }

    fn expression_statement(&mut self) -> Result<Statement, CompileError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;

        Ok(Statement::Expresion(*value))
    }

    fn finish_call(&mut self, callee: Box<Expr>) -> CompResult {
        let mut arguments = Vec::new();

        if !self.check(&TokenType::RightParen) {
            arguments.push(*self.expression()?);

            while self.matches(&[TokenType::Comma]) {
                // TODO add maximum limit of arguments
                arguments.push(*self.expression()?);
            }
        }

        let paren = self
            .consume(TokenType::RightParen, "Expect ')' after arguments.")?
            .clone();

        Ok(Box::new(Expr::Call(callee, paren, arguments)))
    }
}
