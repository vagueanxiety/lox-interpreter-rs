use super::expr::Expr;
use super::token::Token;
use std::rc::Rc;

pub enum Stmt {
    ExprStmt(ExprStmt),
    PrintStmt(PrintStmt),
    VarStmt(VarStmt),
    BlockStmt(BlockStmt),
    IfStmt(IfStmt),
    WhileStmt(WhileStmt),
    FunctionStmt(Rc<FunctionStmt>),
    ReturnStmt(ReturnStmt),
}

pub struct ExprStmt {
    pub expr: Box<Expr>,
}

pub struct PrintStmt {
    pub expr: Box<Expr>,
}

pub struct VarStmt {
    pub name: Token,
    pub value: Option<Box<Expr>>,
}

pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

pub struct IfStmt {
    pub condition: Box<Expr>,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

pub struct WhileStmt {
    pub condition: Box<Expr>,
    pub body: Box<Stmt>,
}

pub struct FunctionStmt {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

pub struct ReturnStmt {
    pub keyword: Token,
    pub value: Option<Box<Expr>>,
}
