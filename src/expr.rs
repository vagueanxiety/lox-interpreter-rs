use crate::literal::Literal;
use crate::token::Token;

pub enum Expr {
    Literal(LiteralExpr),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Grouping(GroupingExpr),
    Var(VarExpr),
    Assign(AssignExpr),
    Logical(LogicalExpr),
    Call(CallExpr),
    Get(GetExpr),
    Set(SetExpr),
    This(ThisExpr),
    Super(SuperExpr),
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
    pub scope_offset: Option<usize>,
}

pub struct AssignExpr {
    pub name: Token,
    pub value: Box<Expr>,
    pub scope_offset: Option<usize>,
}

pub struct LogicalExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct CallExpr {
    pub callee: Box<Expr>,
    pub paren: Token,
    pub args: Vec<Expr>,
}

pub struct GetExpr {
    pub object: Box<Expr>,
    pub name: Token,
}

pub struct SetExpr {
    pub object: Box<Expr>,
    pub name: Token,
    pub value: Box<Expr>,
}

pub struct ThisExpr {
    pub keyword: Token,
    pub scope_offset: Option<usize>,
}

pub struct SuperExpr {
    pub keyword: Token,
    pub method: Token,
    pub scope_offset: Option<usize>,
}
