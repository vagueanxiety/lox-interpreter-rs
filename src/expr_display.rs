use crate::expr::*;
use crate::literal::Literal;
use std::fmt::Display;

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
            Expr::LogicalExpr(expr) => write!(f, "{}", expr),
            Expr::CallExpr(expr) => write!(f, "{}", expr),
        }
    }
}

impl Display for LiteralExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value {
            Literal::Empty => write!(f, "({})", self.value),
            Literal::StringLiteral(_) => write!(f, "(string \"{}\")", self.value),
            Literal::NumberLiteral(_) => write!(f, "(number {})", self.value),
            Literal::BoolLiteral(_) => write!(f, "(bool {})", self.value),
            Literal::FunctionLiteral(_) => write!(f, "(function {})", self.value),
            Literal::NativeFunctionLiteral(_) => write!(f, "(native-function {})", self.value),
        }
    }
}

impl Display for BinaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.operator.lexeme, self.left, self.right)
    }
}

impl Display for UnaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.operator.lexeme, self.right)
    }
}

impl Display for GroupingExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(group {})", self.expr)
    }
}

impl Display for VarExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(var {})", self.name.lexeme)
    }
}

impl Display for AssignExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(assign {} {})", self.name.lexeme, self.value)
    }
}

impl Display for LogicalExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.operator.lexeme, self.left, self.right)
    }
}

impl Display for CallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arg_string = self
            .args
            .iter()
            .fold(String::new(), |acc, a| acc + &a.to_string() + " ");
        let arg_string = arg_string.trim_end();

        write!(f, "(call {} ({}))", self.callee, arg_string)
    }
}
