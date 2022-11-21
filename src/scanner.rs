use crate::literal::Literal;
use crate::token::Token;
use crate::token::TokenType;
use std::error::Error;
use std::fmt;

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

#[derive(Debug)]
pub struct ScanningError {
    pub msg: String,
}

impl fmt::Display for ScanningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ScanningError: {}", self.msg)
    }
}

impl Error for ScanningError {}

impl ScanningError {
    pub fn new(line: usize, msg: &str) -> ScanningError {
        ScanningError {
            msg: format!("[line {}] Error: {}", line, msg),
        }
    }
}

type Result<T> = std::result::Result<T, ScanningError>;

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let source: Vec<char> = source.chars().collect();
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan(mut self) -> Result<Vec<Token>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            String::from(""),
            Literal::Empty,
            self.line,
        ));

        Ok(self.tokens)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<()> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN, Literal::Empty),
            ')' => self.add_token(TokenType::RIGHT_PAREN, Literal::Empty),
            '{' => self.add_token(TokenType::LEFT_BRACE, Literal::Empty),
            '}' => self.add_token(TokenType::RIGHT_BRACE, Literal::Empty),
            ',' => self.add_token(TokenType::COMMA, Literal::Empty),
            '.' => self.add_token(TokenType::DOT, Literal::Empty),
            '-' => self.add_token(TokenType::MINUS, Literal::Empty),
            '+' => self.add_token(TokenType::PLUS, Literal::Empty),
            ';' => self.add_token(TokenType::SEMICOLON, Literal::Empty),
            '*' => self.add_token(TokenType::STAR, Literal::Empty),
            '!' => {
                let t = if self.match_next('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };
                self.add_token(t, Literal::Empty);
            }
            '=' => {
                let t = if self.match_next('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_token(t, Literal::Empty);
            }
            '<' => {
                let t = if self.match_next('=') {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                };
                self.add_token(t, Literal::Empty);
            }
            '>' => {
                let t = if self.match_next('=') {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                };
                self.add_token(t, Literal::Empty);
            }
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH, Literal::Empty);
                }
            }
            ' ' | '\r' | '\t' => {
                // ignore whitespace
            }
            '\n' => {
                self.line += 1;
            }
            '"' => self.string()?,
            _ => {
                if Self::is_digit(c) {
                    self.number();
                } else if Self::is_alpha(c) {
                    self.identifier();
                } else {
                    return Err(ScanningError::new(self.line, "Unexpected character"));
                }
            }
        }
        Ok(())
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn add_token(&mut self, t: TokenType, l: Literal) {
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(t, text, l, self.line));
    }

    fn string(&mut self) -> Result<()> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(ScanningError::new(self.line, "Unterminated string"));
        }

        self.advance();

        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();

        self.add_token(TokenType::STRING, Literal::StringLiteral(value));
        Ok(())
    }

    fn is_digit(c: char) -> bool {
        ('0'..='9').contains(&c)
    }

    fn is_alpha(c: char) -> bool {
        ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c == '_'
    }

    fn is_alpha_numeric(c: char) -> bool {
        Self::is_alpha(c) || Self::is_digit(c)
    }

    fn number(&mut self) {
        while Self::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            self.advance();
            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }

        let value: f64 = self.source[self.start..self.current]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap();
        self.add_token(TokenType::NUMBER, Literal::NumberLiteral(value));
    }

    fn identifier(&mut self) {
        while Self::is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        match Self::get_keyword_token_type(&text) {
            Some(t) => {
                self.add_token(t, Literal::Empty);
            }
            None => {
                self.add_token(TokenType::IDENTIFIER, Literal::Empty);
            }
        }
    }

    fn get_keyword_token_type(s: &str) -> Option<TokenType> {
        // TODO: too lazy to use lazy static and think about lifetime :/
        match s {
            "and" => Some(TokenType::AND),
            "class" => Some(TokenType::CLASS),
            "else" => Some(TokenType::ELSE),
            "false" => Some(TokenType::FALSE),
            "for" => Some(TokenType::FOR),
            "fun" => Some(TokenType::FUN),
            "if" => Some(TokenType::IF),
            "nil" => Some(TokenType::NIL),
            "or" => Some(TokenType::OR),
            "print" => Some(TokenType::PRINT),
            "return" => Some(TokenType::RETURN),
            "super" => Some(TokenType::SUPER),
            "this" => Some(TokenType::THIS),
            "true" => Some(TokenType::TRUE),
            "var" => Some(TokenType::VAR),
            "while" => Some(TokenType::WHILE),
            _ => None,
        }
    }
}
