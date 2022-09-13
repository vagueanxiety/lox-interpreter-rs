use super::expr::Expr;
use super::expr_interpret::RuntimeError;
use super::stmt_interpret::StmtInterpret;
use std::fmt::Display;

pub enum Stmt {
    ExprStmt(ExprStmt),
    PrintStmt(PrintStmt),
}

pub struct ExprStmt {
    pub expr: Box<Expr>,
}

pub struct PrintStmt {
    pub expr: Box<Expr>,
}

// TODO: probably should use the crate enum_dispatch
impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::ExprStmt(s) => write!(f, "{}", s),
            Stmt::PrintStmt(s) => write!(f, "{}", s),
        }
    }
}

// TODO: probably should use the crate enum_dispatch
impl StmtInterpret for Stmt {
    fn execute(&self) -> Result<(), RuntimeError> {
        match self {
            Stmt::ExprStmt(s) => s.execute(),
            Stmt::PrintStmt(s) => s.execute(),
        }
    }
}
