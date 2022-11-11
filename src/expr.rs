use crate::literal::Literal;
use crate::token::Token;

pub enum Expr {
    LiteralExpr(LiteralExpr),
    BinaryExpr(BinaryExpr),
    UnaryExpr(UnaryExpr),
    GroupingExpr(GroupingExpr),
    VarExpr(VarExpr),
    AssignExpr(AssignExpr),
    LogicalExpr(LogicalExpr),
    CallExpr(CallExpr),
}

pub struct LiteralExpr {
    pub value: Literal,
}
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct GroupingExpr {
    pub expr: Box<Expr>,
}

pub struct VarExpr {
    pub name: Token,
}

pub struct AssignExpr {
    pub name: Token,
    pub value: Box<Expr>,
}

pub struct LogicalExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct CallExpr {
    pub callee: Box<Expr>,
    pub paren: Token,
    pub args: Vec<Box<Expr>>,
}
