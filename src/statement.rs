use super::environment::EnvironmentTree;
use super::expr::Expr;
use super::stmt_interpret::Result;
use super::token::Token;
use std::fmt::Display;
use std::io::Write;
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

// TODO: probably should use the crate enum_dispatch
impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::ExprStmt(s) => write!(f, "{}", s),
            Stmt::PrintStmt(s) => write!(f, "{}", s),
            Stmt::VarStmt(s) => write!(f, "{}", s),
            Stmt::BlockStmt(s) => write!(f, "{}", s),
            Stmt::IfStmt(s) => write!(f, "{}", s),
            Stmt::WhileStmt(s) => write!(f, "{}", s),
            Stmt::FunctionStmt(s) => write!(f, "{}", s),
            Stmt::ReturnStmt(s) => write!(f, "{}", s),
        }
    }
}

impl Stmt {
    pub fn execute<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<()> {
        match self {
            Stmt::ExprStmt(s) => s.execute(env, output),
            Stmt::PrintStmt(s) => s.execute(env, output),
            Stmt::VarStmt(s) => s.execute(env, output),
            Stmt::BlockStmt(s) => s.execute(env, output),
            Stmt::IfStmt(s) => s.execute(env, output),
            Stmt::WhileStmt(s) => s.execute(env, output),
            Stmt::FunctionStmt(s) => s.execute(env, output),
            Stmt::ReturnStmt(s) => s.execute(env, output),
        }
    }
}
