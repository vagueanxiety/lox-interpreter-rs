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

// TODO: probably should use the crate enum_dispatch
impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::LiteralExpr(expr) => write!(f, "{}", expr),
            Expr::BinaryExpr(expr) => write!(f, "{}", expr),
            Expr::UnaryExpr(expr) => write!(f, "{}", expr),
            Expr::GroupingExpr(expr) => write!(f, "{}", expr),
        }
    }
}

// TODO: probably should use the crate enum_dispatch
impl ExprInterpret for Expr {
    fn eval(&self) -> Result<Literal> {
        match self {
            Expr::LiteralExpr(expr) => expr.eval(),
            Expr::BinaryExpr(expr) => expr.eval(),
            Expr::UnaryExpr(expr) => expr.eval(),
            Expr::GroupingExpr(expr) => expr.eval(),
        }
    }
}
