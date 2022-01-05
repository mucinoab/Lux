#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub _type: TokenType,
    pub lexeme: String,
    pub place: (usize, usize),
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(
        _type: TokenType,
        lexeme: String,
        place: (usize, usize),
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            _type,
            lexeme,
            place,
            line,
            column,
        }
    }
}
