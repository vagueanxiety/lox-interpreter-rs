use super::error;
use super::expr::Expr;
use super::literal::Literal;
use super::token::Token;
use super::token::TokenType;

type Result<T> = std::result::Result<T, error::ParsingError>;

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

    pub fn parse(mut self) -> Expr {
        return self.expression().expect("Failed to parse tokens");
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        while self.match_one_of(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let token = self.previous().clone();
            let rhs = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: token,
                right: Box::new(rhs),
            }
        }

        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;
        while self.match_one_of(vec![
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let token = self.previous().clone();
            let rhs = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: token,
                right: Box::new(rhs),
            }
        }

        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;
        while self.match_one_of(vec![TokenType::PLUS, TokenType::MINUS]) {
            let token = self.previous().clone();
            let rhs = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: token,
                right: Box::new(rhs),
            }
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;
        while self.match_one_of(vec![TokenType::SLASH, TokenType::STAR]) {
            let token = self.previous().clone();
            let rhs = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: token,
                right: Box::new(rhs),
            }
        }
        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.match_one_of(vec![TokenType::BANG, TokenType::MINUS]) {
            let token = self.previous().clone();
            let rhs = self.unary()?;
            return Ok(Expr::Unary {
                operator: token,
                right: Box::new(rhs),
            });
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr> {
        // booleans are implemented as keywords in the scanner :/
        if self.match_one(TokenType::FALSE) {
            return Ok(Expr::Literal {
                value: Literal::BoolLiteral(false),
            });
        }

        if self.match_one(TokenType::TRUE) {
            return Ok(Expr::Literal {
                value: Literal::BoolLiteral(true),
            });
        }

        // TODO: is clone bad?
        if self.match_one_of(vec![TokenType::NIL, TokenType::STRING, TokenType::NUMBER]) {
            return Ok(Expr::Literal {
                value: self.previous().literal.clone(),
            });
        }

        if self.match_one(TokenType::LEFT_PAREN) {
            let expr = self.expression()?;
            if !self.match_one(TokenType::RIGHT_PAREN) {
                error::report_token_err(self.peek(), "Expect ')' after expression.");
                return Err(error::ParsingError);
            }
            return Ok(Expr::Grouping {
                expr: Box::new(expr),
            });
        }

        error::report_token_err(self.peek(), "Expect expression.");
        return Err(error::ParsingError);
    }

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
