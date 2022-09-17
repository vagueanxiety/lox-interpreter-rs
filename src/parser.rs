use super::expr::*;
use super::literal::Literal;
use super::statement::*;
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
        write!(f, "ParsingError: {}", self.msg)
    }
}

impl Error for ParsingError {}

impl ParsingError {
    pub fn new(t: &Token, msg: &str) -> ParsingError {
        let full_msg = if t.token_type == TokenType::EOF {
            format!("[line {}] Error at end: {}", t.line, msg)
        } else {
            format!("[line {}] Error at '{}': {}", t.line, t.lexeme, msg)
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

    // TODO: since previous might fail, so it should not be called
    // directly by functions other than advance
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if self.is_at_end() {
            return self.peek();
        } else {
            self.current += 1;
            return self.previous();
        }
    }

    fn match_one(&mut self, tt: TokenType) -> Option<&Token> {
        if self.check(tt) {
            return Some(self.advance());
        } else {
            return None;
        }
    }

    fn match_one_of(&mut self, tts: Vec<TokenType>) -> Option<&Token> {
        for tt in tts {
            if self.check(tt) {
                return Some(self.advance());
            }
        }
        return None;
    }

    pub fn parse(mut self) -> Result<Vec<Stmt>> {
        let mut statements = vec![];
        while !self.is_at_end() {
            match self.declaration() {
                Ok(s) => statements.push(s),
                Err(e) => {
                    // still prints the error for now
                    eprintln!("{e}");
                    self.synchronize();
                    continue;
                }
            }
        }
        return Ok(statements);
    }

    fn declaration(&mut self) -> Result<Stmt> {
        match self.match_one(TokenType::VAR) {
            Some(_) => self.var_declaration(),
            None => self.statement(),
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        if let Some(t) = self.match_one(TokenType::IDENTIFIER) {
            let token = t.clone();

            let mut initializer = None;
            if self.match_one(TokenType::EQUAL).is_some() {
                initializer = Some(self.expression()?);
            }

            match self.match_one(TokenType::SEMICOLON) {
                Some(_) => {
                    return Ok(Stmt::VarStmt(VarStmt {
                        token,
                        expr: initializer,
                    }));
                }
                None => {
                    return Err(ParsingError::new(
                        self.peek(),
                        "Expect ';' after variable declaration.",
                    ));
                }
            }
        }
        return Err(ParsingError::new(self.peek(), "Expect variable name"));
    }

    fn statement(&mut self) -> Result<Stmt> {
        match self.match_one(TokenType::PRINT) {
            Some(_) => self.print_statement(),
            None => self.expr_statement(),
        }
    }

    fn print_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        match self.match_one(TokenType::SEMICOLON) {
            Some(_) => Ok(Stmt::PrintStmt(PrintStmt { expr })),
            None => Err(ParsingError::new(self.peek(), "Expect ';' after value.")),
        }
    }

    fn expr_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        match self.match_one(TokenType::SEMICOLON) {
            Some(_) => Ok(Stmt::ExprStmt(ExprStmt { expr })),
            None => Err(ParsingError::new(
                self.peek(),
                "Expect ';' after expression.",
            )),
        }
    }

    fn expression(&mut self) -> Result<Box<Expr>> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.comparison()?;

        while let Some(t) = self.match_one_of(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let token = t.clone();
            let rhs = self.comparison()?;
            expr = Box::new(Expr::BinaryExpr(BinaryExpr {
                left: expr,
                operator: token,
                right: rhs,
            }))
        }

        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.term()?;
        while let Some(t) = self.match_one_of(vec![
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let token = t.clone();
            let rhs = self.term()?;
            expr = Box::new(Expr::BinaryExpr(BinaryExpr {
                left: expr,
                operator: token,
                right: rhs,
            }))
        }

        return Ok(expr);
    }

    fn term(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.factor()?;
        while let Some(t) = self.match_one_of(vec![TokenType::PLUS, TokenType::MINUS]) {
            let token = t.clone();
            let rhs = self.factor()?;
            expr = Box::new(Expr::BinaryExpr(BinaryExpr {
                left: expr,
                operator: token,
                right: rhs,
            }))
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.unary()?;
        while let Some(t) = self.match_one_of(vec![TokenType::SLASH, TokenType::STAR]) {
            let token = t.clone();
            let rhs = self.unary()?;
            expr = Box::new(Expr::BinaryExpr(BinaryExpr {
                left: expr,
                operator: token,
                right: rhs,
            }));
        }
        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Box<Expr>> {
        match self.match_one_of(vec![TokenType::BANG, TokenType::MINUS]) {
            Some(t) => {
                let token = t.clone();
                let rhs = self.unary()?;
                return Ok(Box::new(Expr::UnaryExpr(UnaryExpr {
                    operator: token,
                    right: rhs,
                })));
            }
            None => self.primary(),
        }
    }

    fn primary(&mut self) -> Result<Box<Expr>> {
        // booleans are implemented as keywords in the scanner :/
        if self.match_one(TokenType::FALSE).is_some() {
            return Ok(Box::new(Expr::LiteralExpr(LiteralExpr {
                value: Literal::BoolLiteral(false),
            })));
        }

        if self.match_one(TokenType::TRUE).is_some() {
            return Ok(Box::new(Expr::LiteralExpr(LiteralExpr {
                value: Literal::BoolLiteral(true),
            })));
        }

        // literal values
        if let Some(t) =
            self.match_one_of(vec![TokenType::NIL, TokenType::STRING, TokenType::NUMBER])
        {
            return Ok(Box::new(Expr::LiteralExpr(LiteralExpr {
                value: t.literal.clone(),
            })));
        }

        // variables
        if let Some(t) = self.match_one(TokenType::IDENTIFIER) {
            return Ok(Box::new(Expr::VarExpr(VarExpr { token: t.clone() })));
        }

        // grouping
        if self.match_one(TokenType::LEFT_PAREN).is_some() {
            let expr = self.expression()?;
            match self.match_one(TokenType::RIGHT_PAREN) {
                Some(_) => return Ok(Box::new(Expr::GroupingExpr(GroupingExpr { expr }))),
                None => {
                    return Err(ParsingError::new(
                        self.peek(),
                        "Expect ')' after expression.",
                    ))
                }
            }
        }

        return Err(ParsingError::new(self.peek(), "Expect expression."));
    }

    fn synchronize(&mut self) {
        let mut t = self.advance();
        while t.token_type != TokenType::SEMICOLON && !self.is_at_end() {
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

            t = self.advance();
        }
    }
}
