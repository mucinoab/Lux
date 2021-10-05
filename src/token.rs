#[derive(Debug, Clone)]
pub enum TokenType {
    /// Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    /// One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    /// Literals
    Identifier,
    String(String),
    Number(f64),

    /// Keywords
    If,
    Else,
    And,
    Or,
    False,
    True,
    For,
    While,
    Var,
    Nil,
    Fn,
    Class,
    Super,
    This,
    Print,
    Return,

    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    _type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(_type: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            _type,
            lexeme,
            line,
        }
    }
}
