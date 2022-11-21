use crate::expr::*;
use crate::literal::Literal;
use crate::statement::*;
use crate::token::Token;
use crate::token::TokenType;
use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use std::io;
use std::io::Write;
use std::mem;
use std::rc::Rc;

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

    fn advance(&mut self) -> Token {
        if self.is_at_end() {
            mem::take(&mut self.tokens[self.current])
        } else {
            self.current += 1;
            mem::take(&mut self.tokens[self.current - 1])
        }
    }

    fn match_one(&mut self, tt: TokenType) -> Option<Token> {
        if self.check(tt) {
            Some(self.advance())
        } else {
            None
        }
    }

    fn match_one_of(&mut self, tts: Vec<TokenType>) -> Option<Token> {
        for tt in tts {
            if self.check(tt) {
                return Some(self.advance());
            }
        }
        None
    }

    // TODO: could've reused match_one once Rust's borrow checker
    // allows NLL problem-case-3-conditional-control-flow-across-functions
    fn expect_one(&mut self, tt: TokenType, error_msg: &str) -> Result<Token> {
        if self.check(tt) {
            Ok(self.advance())
        } else {
            Err(ParsingError::new(self.peek(), error_msg))
        }
    }

    pub fn parse<T: Write>(mut self, error_output: &mut T) -> Result<Vec<Stmt>> {
        let mut statements = vec![];
        while !self.is_at_end() {
            match self.declaration() {
                Ok(s) => statements.push(s),
                // parser now synchronizes after *all* parsing errors
                // alternatively, parser can just abort and propogates
                // them up the call chain
                Err(e) => {
                    // still prints the error for now
                    writeln!(error_output, "{e}")?;
                    self.synchronize();
                    continue;
                }
            }
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt> {
        if self.match_one(TokenType::CLASS).is_some() {
            self.class_declaration()
        } else if self.match_one(TokenType::FUN).is_some() {
            self.fun_declaration("function")
        } else if self.match_one(TokenType::VAR).is_some() {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn class_declaration(&mut self) -> Result<Stmt> {
        let name = self.expect_one(TokenType::IDENTIFIER, "Expect class name.")?;

        let superclass = if self.match_one(TokenType::LESS).is_some() {
            let sc_name = self.expect_one(TokenType::IDENTIFIER, "Expect superclass name.")?;
            Some(VarExpr {
                name: sc_name,
                scope_offset: None,
            })
        } else {
            None
        };

        self.expect_one(TokenType::LEFT_BRACE, "Expect '{' before class body.")?;
        let mut methods = vec![];
        while !self.check(TokenType::RIGHT_BRACE) && !self.is_at_end() {
            methods.push(Rc::new(RefCell::new(self.function("method")?)));
        }
        self.expect_one(TokenType::RIGHT_BRACE, "Expect '}' after class body.")?;

        Ok(Stmt::ClassStmt(ClassStmt {
            name,
            methods,
            superclass,
        }))
    }

    fn function(&mut self, kind: &str) -> Result<FunctionStmt> {
        let name = self.expect_one(TokenType::IDENTIFIER, &format!("Expect {} name.", kind))?;
        self.expect_one(
            TokenType::LEFT_PAREN,
            &format!("Expect '(' afeter {} name.", kind),
        )?;

        let mut params = vec![];
        if !self.check(TokenType::RIGHT_PAREN) {
            loop {
                if params.len() >= 255 {
                    return Err(ParsingError::new(
                        self.peek(),
                        "Can't have more than 255 parameters.",
                    ));
                }
                params.push(self.expect_one(TokenType::IDENTIFIER, "Expect parameter name")?);
                if self.match_one(TokenType::COMMA).is_none() {
                    break;
                }
            }
        }

        self.expect_one(TokenType::RIGHT_PAREN, "Expect ')' after parameters.")?;
        self.expect_one(
            TokenType::LEFT_BRACE,
            &format!("Expect '{{' before {} body.", kind),
        )?;

        let mut body = vec![];
        while !self.check(TokenType::RIGHT_BRACE) && !self.is_at_end() {
            body.push(self.declaration()?);
        }
        self.expect_one(TokenType::RIGHT_BRACE, "Expect '}' after body.")?;
        Ok(FunctionStmt { name, params, body })
    }

    fn fun_declaration(&mut self, kind: &str) -> Result<Stmt> {
        let fun = self.function(kind)?;
        Ok(Stmt::FunctionStmt(Rc::new(RefCell::new(fun))))
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        let token = self.expect_one(TokenType::IDENTIFIER, "Expect variable name")?;
        let mut initializer = None;
        if self.match_one(TokenType::EQUAL).is_some() {
            initializer = Some(self.expression()?);
        }

        self.expect_one(
            TokenType::SEMICOLON,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Stmt::VarStmt(VarStmt {
            name: token,
            value: initializer,
        }))
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
        } else if let Some(token) = self.match_one(TokenType::RETURN) {
            self.return_statement(token)
        } else if self.match_one(TokenType::LEFT_BRACE).is_some() {
            self.block_statement()
        } else {
            self.expr_statement()
        }
    }

    fn return_statement(&mut self, keyword: Token) -> Result<Stmt> {
        let mut value = None;
        if !self.check(TokenType::SEMICOLON) {
            value = Some(self.expression()?)
        }

        self.expect_one(TokenType::SEMICOLON, "Expect ';' after return value.")?;
        Ok(Stmt::ReturnStmt(ReturnStmt { keyword, value }))
    }

    fn for_statetment(&mut self) -> Result<Stmt> {
        self.expect_one(TokenType::LEFT_PAREN, "Expect '(' after 'for'.")?;

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
        self.expect_one(TokenType::SEMICOLON, "Expect ';' after loop condition.")?;

        let increment = if self.check(TokenType::RIGHT_PAREN) {
            None
        } else {
            Some(self.expression()?)
        };
        self.expect_one(TokenType::RIGHT_PAREN, "Expect ')' after for clauses.")?;

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
        self.expect_one(TokenType::LEFT_PAREN, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.expect_one(TokenType::RIGHT_PAREN, "Expect ')' after while condition.")?;
        let body = Box::new(self.statement()?);
        Ok(Stmt::WhileStmt(WhileStmt { condition, body }))
    }

    fn if_statement(&mut self) -> Result<Stmt> {
        self.expect_one(TokenType::LEFT_PAREN, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.expect_one(TokenType::RIGHT_PAREN, "Expect ')' after if condition.")?;

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
        self.expect_one(TokenType::RIGHT_BRACE, "Expect '}' after block.")?;
        Ok(Stmt::BlockStmt(BlockStmt { statements }))
    }

    fn print_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.expect_one(TokenType::SEMICOLON, "Expect ';' after value.")?;
        Ok(Stmt::PrintStmt(PrintStmt { expr }))
    }

    fn expr_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.expect_one(TokenType::SEMICOLON, "Expect ';' after expression.")?;
        Ok(Stmt::ExprStmt(ExprStmt { expr }))
    }

    fn expression(&mut self) -> Result<Box<Expr>> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Box<Expr>> {
        let expr = self.or()?;
        if let Some(token) = self.match_one(TokenType::EQUAL) {
            match *expr {
                Expr::VarExpr(e) => {
                    let value = self.assignment()?;
                    return Ok(Box::new(Expr::AssignExpr(AssignExpr {
                        name: e.name,
                        value,
                        scope_offset: None,
                    })));
                }
                Expr::GetExpr(e) => {
                    let value = self.assignment()?;
                    return Ok(Box::new(Expr::SetExpr(SetExpr {
                        object: e.object,
                        name: e.name,
                        value,
                    })));
                }
                _ => {
                    return Err(ParsingError::new(&token, "Invalid assignment target."));
                }
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.and()?;
        while let Some(token) = self.match_one(TokenType::OR) {
            let rhs = self.and()?;
            expr = Box::new(Expr::LogicalExpr(LogicalExpr {
                left: expr,
                operator: token,
                right: rhs,
            }))
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.equality()?;
        while let Some(token) = self.match_one(TokenType::AND) {
            let rhs = self.equality()?;
            expr = Box::new(Expr::LogicalExpr(LogicalExpr {
                left: expr,
                operator: token,
                right: rhs,
            }))
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.comparison()?;

        while let Some(token) =
            self.match_one_of(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL])
        {
            let rhs = self.comparison()?;
            expr = Box::new(Expr::BinaryExpr(BinaryExpr {
                left: expr,
                operator: token,
                right: rhs,
            }))
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.term()?;
        while let Some(token) = self.match_one_of(vec![
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let rhs = self.term()?;
            expr = Box::new(Expr::BinaryExpr(BinaryExpr {
                left: expr,
                operator: token,
                right: rhs,
            }))
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.factor()?;
        while let Some(token) = self.match_one_of(vec![TokenType::PLUS, TokenType::MINUS]) {
            let rhs = self.factor()?;
            expr = Box::new(Expr::BinaryExpr(BinaryExpr {
                left: expr,
                operator: token,
                right: rhs,
            }))
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.unary()?;
        while let Some(token) = self.match_one_of(vec![TokenType::SLASH, TokenType::STAR]) {
            let rhs = self.unary()?;
            expr = Box::new(Expr::BinaryExpr(BinaryExpr {
                left: expr,
                operator: token,
                right: rhs,
            }));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<Expr>> {
        if let Some(token) = self.match_one_of(vec![TokenType::BANG, TokenType::MINUS]) {
            let rhs = self.unary()?;
            Ok(Box::new(Expr::UnaryExpr(UnaryExpr {
                operator: token,
                right: rhs,
            })))
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.primary()?;
        loop {
            if self.match_one(TokenType::LEFT_PAREN).is_some() {
                expr = self.finish_call(expr)?;
            } else if self.match_one(TokenType::DOT).is_some() {
                let name =
                    self.expect_one(TokenType::IDENTIFIER, "Expect property name after '.'.")?;
                expr = Box::new(Expr::GetExpr(GetExpr { object: expr, name }))
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Box<Expr>) -> Result<Box<Expr>> {
        let mut args = vec![];
        if !self.check(TokenType::RIGHT_PAREN) {
            loop {
                if args.len() >= 255 {
                    return Err(ParsingError::new(
                        self.peek(),
                        "Can't have more than 255 arguments.",
                    ));
                }
                args.push(*(self.expression()?));
                if self.match_one(TokenType::COMMA).is_none() {
                    break;
                }
            }
        }

        let paren = self.expect_one(TokenType::RIGHT_PAREN, "Expect ')' after arguments.")?;
        Ok(Box::new(Expr::CallExpr(CallExpr {
            callee,
            paren,
            args,
        })))
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
        if let Some(token) =
            self.match_one_of(vec![TokenType::NIL, TokenType::STRING, TokenType::NUMBER])
        {
            return Ok(Box::new(Expr::LiteralExpr(LiteralExpr {
                value: token.literal,
            })));
        }

        // super
        if let Some(token) = self.match_one(TokenType::SUPER) {
            self.expect_one(TokenType::DOT, "Expect '.' after 'super'.")?;
            let method =
                self.expect_one(TokenType::IDENTIFIER, "Expect superclass method name.")?;
            return Ok(Box::new(Expr::SuperExpr(SuperExpr {
                keyword: token,
                method,
                scope_offset: None,
            })));
        }

        // this
        if let Some(token) = self.match_one(TokenType::THIS) {
            return Ok(Box::new(Expr::ThisExpr(ThisExpr {
                keyword: token,
                scope_offset: None,
            })));
        }

        // variables
        if let Some(token) = self.match_one(TokenType::IDENTIFIER) {
            return Ok(Box::new(Expr::VarExpr(VarExpr {
                name: token,
                scope_offset: None,
            })));
        }

        // grouping
        if self.match_one(TokenType::LEFT_PAREN).is_some() {
            let expr = self.expression()?;
            self.expect_one(TokenType::RIGHT_PAREN, "Expect ')' after expression.")?;
            return Ok(Box::new(Expr::GroupingExpr(GroupingExpr { expr })));
        }

        Err(ParsingError::new(self.peek(), "Expect expression."))
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
