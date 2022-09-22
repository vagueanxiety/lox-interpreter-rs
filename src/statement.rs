use super::environment::Environments;
use super::expr::Expr;
use super::expr_interpret::RuntimeError;
use super::stmt_interpret::StmtInterpret;
use super::token::Token;
use std::fmt::Display;
use std::io::Write;

pub enum Stmt {
    ExprStmt(ExprStmt),
    PrintStmt(PrintStmt),
    VarStmt(VarStmt),
    BlockStmt(BlockStmt),
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

pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

// TODO: probably should use the crate enum_dispatch
impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::ExprStmt(s) => write!(f, "{}", s),
            Stmt::PrintStmt(s) => write!(f, "{}", s),
            Stmt::VarStmt(s) => write!(f, "{}", s),
            Stmt::BlockStmt(s) => write!(f, "{}", s),
        }
    }
}

// TODO: probably should use the crate enum_dispatch
impl StmtInterpret for Stmt {
    fn execute<T: Write>(
        &self,
        env: &mut Environments,
        output: &mut T,
    ) -> Result<(), RuntimeError> {
        match self {
            Stmt::ExprStmt(s) => s.execute(env, output),
            Stmt::PrintStmt(s) => s.execute(env, output),
            Stmt::VarStmt(s) => s.execute(env, output),
            Stmt::BlockStmt(s) => s.execute(env, output),
        }
    }
}
