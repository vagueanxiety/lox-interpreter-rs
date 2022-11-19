use crate::expr::Expr;
use crate::token::Token;
use std::cell::RefCell;
use std::rc::Rc;

// Note:
// FunctionStmt is in Rc because it can be owned by Stmt
// and LoxFunction. It is in RefCell because Resolver needs to
// mutate exprs within FunctionStmt to save scope_offset. And the
// mutation is safe because only Resolver borrows it mutably and exclusively.
pub enum Stmt {
    ExprStmt(ExprStmt),
    PrintStmt(PrintStmt),
    VarStmt(VarStmt),
    BlockStmt(BlockStmt),
    IfStmt(IfStmt),
    WhileStmt(WhileStmt),
    FunctionStmt(Rc<RefCell<FunctionStmt>>),
    ReturnStmt(ReturnStmt),
    ClassStmt(ClassStmt),
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

pub struct ClassStmt {
    pub name: Token,
    pub methods: Vec<Rc<RefCell<FunctionStmt>>>,
}
