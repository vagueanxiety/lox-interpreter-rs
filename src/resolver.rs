use crate::statement::Stmt;
use crate::token::Token;
use std::collections::HashMap;
use std::{error::Error, fmt};

type Scope = HashMap<String, bool>;

#[derive(PartialEq)]
pub enum FunctionType {
    NonFun,
    Fun,
    Method,
    Initializer,
}

#[derive(PartialEq)]
pub enum ClassType {
    NonClass,
    Class,
}

#[derive(Debug)]
pub struct ResolutionError {
    pub msg: String,
}

impl fmt::Display for ResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ResolutionError: {}", self.msg)
    }
}

impl ResolutionError {
    pub fn new(t: &Token, msg: &str) -> ResolutionError {
        let full_msg = format!("[line {}] {}", t.line, msg);
        ResolutionError { msg: full_msg }
    }
}

impl Error for ResolutionError {}

pub type Result<T> = std::result::Result<T, ResolutionError>;

pub struct Resolver {
    scopes: Vec<Scope>,
    pub current_fun: FunctionType,
    pub current_cls: ClassType,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            scopes: vec![],
            current_fun: FunctionType::NonFun,
            current_cls: ClassType::NonClass,
        }
    }

    pub fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn end_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn declare(&mut self, name: &Token) -> Result<()> {
        if let Some(s) = self.peek() {
            if s.contains_key(&name.lexeme) {
                return Err(ResolutionError::new(
                    name,
                    "Already a variable with this name in this scope.",
                ));
            }
            s.insert(name.lexeme.clone(), false);
        }
        Ok(())
    }

    pub fn define(&mut self, name: &Token) {
        if let Some(s) = self.peek() {
            s.insert(name.lexeme.clone(), true);
        }
    }

    pub fn get(&mut self, name: &Token) -> Option<&bool> {
        let s = self.peek()?;
        s.get(&name.lexeme)
    }

    // theoretically we should have a &self equivalent
    // this method should be used *publicly* only by ClassStmt to define 'this'
    pub fn peek(&mut self) -> Option<&mut Scope> {
        if self.scopes.is_empty() {
            None
        } else {
            let i = self.scopes.len() - 1;
            Some(&mut self.scopes[i])
        }
    }

    pub fn resolve_local(&self, name: &Token) -> Option<usize> {
        for (i, s) in self.scopes.iter().rev().enumerate() {
            if s.contains_key(&name.lexeme) {
                return Some(i);
            }
        }
        None
    }

    pub fn resolve(mut self, statements: &mut [Stmt]) -> Result<()> {
        for s in statements.iter_mut() {
            s.resolve(&mut self)?;
        }
        Ok(())
    }
}
