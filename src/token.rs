use std::fmt;

//#[derive(Debug)]
pub enum Literal {
    Empty,
    StringLiteral(String),
    NumberLiteral(f64),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Literal::Empty => write!(f, "nil"),
            Literal::StringLiteral(ref s) => write!(f, "{}", s),
            Literal::NumberLiteral(n) => write!(f, "{}", n),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

pub struct Token {
    token_type: TokenType,
    pub lexeme: String,
    literal: Literal,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "token_type: {:?}, lexeme: {}, literal: {}",
            self.token_type, self.lexeme, self.literal
        )
    }
}
