use super::expr::*;
use super::literal::Literal;
use super::statement::*;
use super::token::Token;
use super::token::TokenType;
use std::error::Error;
use std::fmt;
use std::io;
use std::io::Write;

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

impl From<io::Error> for ParsingError {
    fn from(error: io::Error) -> Self {
        ParsingError {
            msg: format!("ParsingError caused by an IO error: {error}"),
        }
    }
}

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

// Note that tokens need to always end with an EOF token
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

    fn advance(&mut self) -> &Token {
        if self.is_at_end() {
            return self.peek();
        } else {
            self.current += 1;
            return &self.tokens[self.current - 1];
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

    fn consume_one(&mut self, tt: TokenType, error_msg: &str) -> Result<()> {
        if self.match_one(tt).is_none() {
            return Err(ParsingError::new(self.peek(), error_msg));
        }
        Ok(())
    }

    pub fn parse<T: Write>(mut self, error_output: &mut T) -> Result<Vec<Stmt>> {
        let mut statements = vec![];
        while !self.is_at_end() {
            match self.declaration() {
                Ok(s) => statements.push(s),
                // parser now synchronizes after *all* parsing errors
                // there could be errors after which parser just aborts
                // and propogates them up the call chain
                Err(e) => {
                    // still prints the error for now
                    write!(error_output, "{e}\n")?;
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

            self.consume_one(
                TokenType::SEMICOLON,
                "Expect ';' after variable declaration.",
            )?;

            return Ok(Stmt::VarStmt(VarStmt {
                name: token,
                value: initializer,
            }));
        }
        return Err(ParsingError::new(self.peek(), "Expect variable name"));
    }

    fn statement(&mut self) -> Result<Stmt> {
        if self.match_one(TokenType::IF).is_some() {
            self.if_statement()
        } else if self.match_one(TokenType::WHILE).is_some() {
            self.while_statetment()
        } else if self.match_one(TokenType::FOR).is_some() {
            self.for_statetment()
        } else if self.match_one(TokenType::PRINT).is_some() {
            self.print_statement()
        } else if self.match_one(TokenType::LEFT_BRACE).is_some() {
            self.block_statement()
        } else {
            self.expr_statement()
        }
    }

    fn for_statetment(&mut self) -> Result<Stmt> {
        self.consume_one(TokenType::LEFT_PAREN, "Expect '(' after 'for'.")?;

        let initializer = if self.match_one(TokenType::SEMICOLON).is_some() {
            None
        } else if self.match_one(TokenType::VAR).is_some() {
            Some(self.var_declaration()?)
        } else {
            Some(self.expr_statement()?)
        };

        let condition = if self.check(TokenType::SEMICOLON) {
            Box::new(Expr::LiteralExpr(LiteralExpr {
                value: Literal::BoolLiteral(true),
            }))
        } else {
            self.expression()?
        };
        self.consume_one(TokenType::SEMICOLON, "Expect ';' after loop condition.")?;

        let increment = if self.check(TokenType::RIGHT_PAREN) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume_one(TokenType::RIGHT_PAREN, "Expect ')' after for clauses.")?;

        // desugaring for loop into block statement + while loop
        let mut body = self.statement()?;

        // including increment right after while body
        if let Some(i) = increment {
            body = Stmt::BlockStmt(BlockStmt {
                statements: vec![body, Stmt::ExprStmt(ExprStmt { expr: i })],
            })
        }

        // constructing while
        let mut while_stmt = Stmt::WhileStmt(WhileStmt {
            condition,
            body: Box::new(body),
        });

        // including initializer right before while statement
        if let Some(i) = initializer {
            while_stmt = Stmt::BlockStmt(BlockStmt {
                statements: vec![i, while_stmt],
            })
        }

        Ok(while_stmt)
    }

    fn while_statetment(&mut self) -> Result<Stmt> {
        self.consume_one(TokenType::LEFT_PAREN, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume_one(TokenType::RIGHT_PAREN, "Expect ')' after while condition.")?;
        let body = Box::new(self.statement()?);
        Ok(Stmt::WhileStmt(WhileStmt { condition, body }))
    }

    fn if_statement(&mut self) -> Result<Stmt> {
        self.consume_one(TokenType::LEFT_PAREN, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume_one(TokenType::RIGHT_PAREN, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);
        let mut else_branch = None;
        if self.match_one(TokenType::ELSE).is_some() {
            else_branch = Some(Box::new(self.statement()?));
        }

        Ok(Stmt::IfStmt(IfStmt {
            condition,
            then_branch,
            else_branch,
        }))
    }

    fn block_statement(&mut self) -> Result<Stmt> {
        let mut statements = vec![];
        while !self.check(TokenType::RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume_one(TokenType::RIGHT_BRACE, "Expect '}' after block.")?;
        Ok(Stmt::BlockStmt(BlockStmt { statements }))
    }

    fn print_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume_one(TokenType::SEMICOLON, "Expect ';' after value.")?;
        Ok(Stmt::PrintStmt(PrintStmt { expr }))
    }

    fn expr_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume_one(TokenType::SEMICOLON, "Expect ';' after expression.")?;
        Ok(Stmt::ExprStmt(ExprStmt { expr }))
    }

    fn expression(&mut self) -> Result<Box<Expr>> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Box<Expr>> {
        let expr = self.or()?;
        if let Some(t) = self.match_one(TokenType::EQUAL) {
            match *expr {
                Expr::VarExpr(e) => {
                    let value = self.assignment()?;
                    return Ok(Box::new(Expr::AssignExpr(AssignExpr {
                        name: e.name,
                        value,
                    })));
                }
                _ => {
                    return Err(ParsingError::new(t, "Invalid assignment target."));
                }
            }
        }
        return Ok(expr);
    }

    fn or(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.and()?;
        while let Some(t) = self.match_one(TokenType::OR) {
            let token = t.clone();
            let rhs = self.and()?;
            expr = Box::new(Expr::LogicalExpr(LogicalExpr {
                left: expr,
                operator: token,
                right: rhs,
            }))
        }

        return Ok(expr);
    }

    fn and(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.equality()?;
        while let Some(t) = self.match_one(TokenType::AND) {
            let token = t.clone();
            let rhs = self.equality()?;
            expr = Box::new(Expr::LogicalExpr(LogicalExpr {
                left: expr,
                operator: token,
                right: rhs,
            }))
        }

        return Ok(expr);
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
            return Ok(Box::new(Expr::VarExpr(VarExpr { name: t.clone() })));
        }

        // grouping
        if self.match_one(TokenType::LEFT_PAREN).is_some() {
            let expr = self.expression()?;
            self.consume_one(TokenType::RIGHT_PAREN, "Expect ')' after expression.")?;
            return Ok(Box::new(Expr::GroupingExpr(GroupingExpr { expr })));
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
