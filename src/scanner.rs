use crate::{
    errors::error,
    token::{Token, TokenType},
    Error,
};

use std::{str::Chars, str::FromStr};

use peekmore::{PeekMore, PeekMoreIterator};

pub struct Scanner<'s> {
    source_raw: &'s str,
    source: PeekMoreIterator<Chars<'s>>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl<'s> Scanner<'s> {
    pub fn new(source_raw: &'s str) -> Self {
        Self {
            source_raw,
            source: source_raw.chars().peekmore(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.column += 1;
        self.source.next()
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    fn scan_token(&mut self) -> Result<(), Error> {
        if let Some(c) = self.advance() {
            let token = match c {
                '(' => TokenType::LeftParen,
                ')' => TokenType::RightParen,

                '{' => TokenType::LeftBrace,
                '}' => TokenType::RightBrace,

                '.' => TokenType::Dot,
                ',' => TokenType::Comma,
                ';' => TokenType::Semicolon,

                '-' => TokenType::Minus,
                '+' => TokenType::Plus,
                '*' => TokenType::Star,
                '/' => {
                    if self.peek() == Some(&'/') {
                        // it is a comment, skip until end of line
                        while self.peek().is_some() && self.peek() != Some(&'\n') {
                            self.advance();
                        }

                        return Ok(());
                    }
                    // TODO comment blocks: /* ... */
                    TokenType::Slash
                }

                // One or two character tokens
                // TODO repetitive code
                '!' => {
                    if self.source.peek() == Some(&'=') {
                        self.advance();
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    }
                }

                '=' => {
                    if self.source.peek() == Some(&'=') {
                        self.advance();
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    }
                }

                '<' => {
                    if self.source.peek() == Some(&'=') {
                        self.advance();
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    }
                }

                '>' => {
                    if self.source.peek() == Some(&'=') {
                        self.advance();
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    }
                }

                // Skip whitespace
                ' ' | '\r' | '\t' => {
                    return Ok(());
                }

                '\n' => {
                    self.line += 1;
                    self.column = 1; // reset column

                    return Ok(());
                }

                // Strings
                '"' => {
                    self.string();
                    return Ok(());
                }

                // Numbers
                '0'..='9' => {
                    return self.number();
                }

                _ => {
                    // TODO
                    // Figure out errors
                    // keep scanning?
                    // report in batch, no one by one char

                    if c.is_alphabetic() {
                        self.identifier();
                        return Ok(());
                    } else {
                        dbg!(c);
                        error(self.line, "Unexpected character.");
                        return Err(Box::new(std::fmt::Error));
                    }
                }
            };

            self.add_token(token);
        }

        Ok(())
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Error> {
        while self.peek().is_some() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), 0));

        Ok(self.tokens.clone())
    }

    fn add_token(&mut self, token: TokenType) {
        let text = self.source_raw[self.start..self.current].into();
        self.tokens.push(Token::new(token, text, self.line));
    }

    fn string(&mut self) {
        while self.peek() != Some(&'"') {
            if self.peek() == Some(&'\n') {
                self.line += 1;
                self.column = 0;
            }

            self.advance();
        }

        if self.peek().is_none() {
            error(self.line, "Unterminated string.");
            return;
        }

        // The closing "
        self.advance();

        let value = self.source_raw[self.start + 1..self.current - 1].into();

        self.add_token(TokenType::String(value));
    }

    fn number(&mut self) -> Result<(), Error> {
        // TODO repetitive code
        while let Some(c) = self.peek() {
            if c.is_numeric() {
                self.advance();
            } else {
                break;
            }
        }

        // TODO repetitive code                                             v HACK
        if self.peek() == Some(&'.') && self.source.peek_nth(1).unwrap_or(&'*').is_numeric() {
            self.advance();
            while let Some(c) = self.peek() {
                if c.is_numeric() {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let n = f64::from_str(&self.source_raw[self.start..self.current])?;

        self.add_token(TokenType::Number(n));

        Ok(())
    }

    fn identifier(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() {
                self.advance();
            } else {
                break;
            }
        }

        let token = match &self.source_raw[self.start..self.current] {
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "and" => TokenType::And,
            "or" => TokenType::Or,
            "false" => TokenType::False,
            "true" => TokenType::True,
            "for" => TokenType::For,
            "while" => TokenType::While,
            "var" => TokenType::Var,
            "nil" => TokenType::Nil,
            "fn" => TokenType::Fn,
            "class" => TokenType::Class,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            _ => TokenType::Identifier,
        };

        self.add_token(token);
    }
}
