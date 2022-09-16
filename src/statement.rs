use super::environment::Environment;
use super::expr::Expr;
use super::expr_interpret::RuntimeError;
use super::stmt_interpret::StmtInterpret;
use super::token::Token;
use std::fmt::Display;

pub enum Stmt {
    ExprStmt(ExprStmt),
    PrintStmt(PrintStmt),
    VarStmt(VarStmt),
}

pub struct ExprStmt {
    pub expr: Box<Expr>,
}

pub struct PrintStmt {
    pub expr: Box<Expr>,
}

pub struct VarStmt {
    pub token: Token,
    pub expr: Option<Box<Expr>>,
}

// TODO: probably should use the crate enum_dispatch
impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::ExprStmt(s) => write!(f, "{}", s),
            Stmt::PrintStmt(s) => write!(f, "{}", s),
            Stmt::VarStmt(s) => write!(f, "{}", s),
        }
    }
}

// TODO: probably should use the crate enum_dispatch
impl StmtInterpret for Stmt {
    fn execute(&self, env: &mut Environment) -> Result<(), RuntimeError> {
        match self {
            Stmt::ExprStmt(s) => s.execute(env),
            Stmt::PrintStmt(s) => s.execute(env),
            Stmt::VarStmt(s) => s.execute(env),
        }
    }
}
