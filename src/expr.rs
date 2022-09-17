use super::environment::Environment;
use super::expr_interpret::ExprInterpret;
use super::expr_interpret::Result;
use super::literal::Literal;
use super::token::Token;
use std::fmt::Display;

pub enum Expr {
    LiteralExpr(LiteralExpr),
    BinaryExpr(BinaryExpr),
    UnaryExpr(UnaryExpr),
    GroupingExpr(GroupingExpr),
    VarExpr(VarExpr),
    AssignExpr(AssignExpr),
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
    pub token: Token,
}

pub struct AssignExpr {
    pub token: Token,
    pub value: Box<Expr>,
}

// TODO: probably should use the crate enum_dispatch
impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::LiteralExpr(expr) => write!(f, "{}", expr),
            Expr::BinaryExpr(expr) => write!(f, "{}", expr),
            Expr::UnaryExpr(expr) => write!(f, "{}", expr),
            Expr::GroupingExpr(expr) => write!(f, "{}", expr),
            Expr::VarExpr(expr) => write!(f, "{}", expr),
            Expr::AssignExpr(expr) => write!(f, "{}", expr),
        }
    }
}

// TODO: probably should use the crate enum_dispatch
impl ExprInterpret for Expr {
    fn eval(&self, env: &mut Environment) -> Result<Literal> {
        match self {
            Expr::LiteralExpr(expr) => expr.eval(env),
            Expr::BinaryExpr(expr) => expr.eval(env),
            Expr::UnaryExpr(expr) => expr.eval(env),
            Expr::GroupingExpr(expr) => expr.eval(env),
            Expr::VarExpr(expr) => expr.eval(env),
            Expr::AssignExpr(expr) => expr.eval(env),
        }
    }
}
