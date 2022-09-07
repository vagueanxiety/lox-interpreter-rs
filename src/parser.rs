use super::expr::BinaryExpr;
use super::expr::Expr;
use super::expr::GroupingExpr;
use super::expr::LiteralExpr;
use super::expr::UnaryExpr;
use super::literal::Literal;
use super::token::Token;
use super::token::TokenType;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParsingError {
    pub msg: String,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for ParsingError {}

impl ParsingError {
    pub fn new(t: &Token, msg: &str) -> ParsingError {
        let full_msg = if t.token_type == TokenType::EOF {
            format!("[line {}] Error at end: {}", t.line, msg)
        } else {
            format!("[line {}] Error at {}: {}", t.line, t.lexeme, msg)
        };
        ParsingError { msg: full_msg }
    }
}

type Result<T> = std::result::Result<T, ParsingError>;

// TODO:
// contract/assumptions/invariants:
// - tokens has at least 2 elements
// - current always points to a valid element in tokens
// - first previous call only occurs after advance
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn check(&self, tt: TokenType) -> bool {
        self.peek().token_type == tt
    }

    fn is_at_end(&self) -> bool {
        self.check(TokenType::EOF)
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    // TODO: hmm could be better
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn match_one(&mut self, tt: TokenType) -> bool {
        if self.check(tt) {
            self.advance();
            return true;
        } else {
            return false;
        }
    }

    fn match_one_of(&mut self, tts: Vec<TokenType>) -> bool {
        for tt in tts {
            if self.check(tt) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    pub fn parse(mut self) -> Result<Box<dyn Expr>> {
        return self.expression();
    }

    fn expression(&mut self) -> Result<Box<dyn Expr>> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Box<dyn Expr>> {
        let mut expr = self.comparison()?;

        while self.match_one_of(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let token = self.previous().clone();
            let rhs = self.comparison()?;
            expr = Box::new(BinaryExpr {
                left: expr,
                operator: token,
                right: rhs,
            })
        }

        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Box<dyn Expr>> {
        let mut expr = self.term()?;
        while self.match_one_of(vec![
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let token = self.previous().clone();
            let rhs = self.term()?;
            expr = Box::new(BinaryExpr {
                left: expr,
                operator: token,
                right: rhs,
            })
        }

        return Ok(expr);
    }

    fn term(&mut self) -> Result<Box<dyn Expr>> {
        let mut expr = self.factor()?;
        while self.match_one_of(vec![TokenType::PLUS, TokenType::MINUS]) {
            let token = self.previous().clone();
            let rhs = self.factor()?;
            expr = Box::new(BinaryExpr {
                left: expr,
                operator: token,
                right: rhs,
            })
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Box<dyn Expr>> {
        let mut expr = self.unary()?;
        while self.match_one_of(vec![TokenType::SLASH, TokenType::STAR]) {
            let token = self.previous().clone();
            let rhs = self.unary()?;
            expr = Box::new(BinaryExpr {
                left: expr,
                operator: token,
                right: rhs,
            });
        }
        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Box<dyn Expr>> {
        if self.match_one_of(vec![TokenType::BANG, TokenType::MINUS]) {
            let token = self.previous().clone();
            let rhs = self.unary()?;
            return Ok(Box::new(UnaryExpr {
                operator: token,
                right: rhs,
            }));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Box<dyn Expr>> {
        // booleans are implemented as keywords in the scanner :/
        if self.match_one(TokenType::FALSE) {
            return Ok(Box::new(LiteralExpr {
                value: Literal::BoolLiteral(false),
            }));
        }

        if self.match_one(TokenType::TRUE) {
            return Ok(Box::new(LiteralExpr {
                value: Literal::BoolLiteral(true),
            }));
        }

        // TODO: is clone bad?
        if self.match_one_of(vec![TokenType::NIL, TokenType::STRING, TokenType::NUMBER]) {
            return Ok(Box::new(LiteralExpr {
                value: self.previous().literal.clone(),
            }));
        }

        if self.match_one(TokenType::LEFT_PAREN) {
            let expr = self.expression()?;
            if !self.match_one(TokenType::RIGHT_PAREN) {
                return Err(ParsingError::new(
                    self.peek(),
                    "Expect ')' after expression.",
                ));
            }
            return Ok(Box::new(GroupingExpr { expr }));
        }

        return Err(ParsingError::new(self.peek(), "Expect expression."));
    }

    #[allow(dead_code)]
    // TODO: unused for now
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SEMICOLON {
                return;
            }

            match self.peek().token_type {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => return,
                _ => {}
            }

            self.advance();
        }
    }
}
