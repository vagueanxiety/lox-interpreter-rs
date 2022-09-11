use super::expr_interpret::ExprInterpret;
use super::literal::Literal;
use super::token::Token;
use std::fmt::Display;

pub trait Expr: Display + ExprInterpret {}
impl<T: Display + ExprInterpret> Expr for T {}

pub struct LiteralExpr {
    pub value: Literal,
}

pub struct BinaryExpr {
    pub left: Box<dyn Expr>,
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

pub struct GroupingExpr {
    pub expr: Box<dyn Expr>,
}
