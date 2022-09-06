use super::ast_display::AstDisplay;
use super::ast_interpret::AstInterpret;
use super::literal::Literal;
use super::token::Token;

pub trait Expr: AstDisplay + AstInterpret {}
impl<T: AstDisplay + AstInterpret> Expr for T {}

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
