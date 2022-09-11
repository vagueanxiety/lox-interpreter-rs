use super::expr::Expr;
use super::stmt_interpret::StmtInterpret;
use std::fmt::Display;

pub trait Stmt: StmtInterpret + Display {}
impl<T: StmtInterpret + Display> Stmt for T {}

pub struct ExprStmt {
    pub expr: Box<dyn Expr>,
}

pub struct PrintStmt {
    pub expr: Box<dyn Expr>,
}
