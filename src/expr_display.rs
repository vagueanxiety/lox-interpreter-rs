use crate::expr::*;
use crate::literal::Literal;
use std::fmt::Display;

// TODO: probably should use the crate enum_dispatch
impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal(expr) => write!(f, "{}", expr),
            Expr::Binary(expr) => write!(f, "{}", expr),
            Expr::Unary(expr) => write!(f, "{}", expr),
            Expr::Grouping(expr) => write!(f, "{}", expr),
            Expr::Var(expr) => write!(f, "{}", expr),
            Expr::Assign(expr) => write!(f, "{}", expr),
            Expr::Logical(expr) => write!(f, "{}", expr),
            Expr::Call(expr) => write!(f, "{}", expr),
            Expr::Get(expr) => write!(f, "{}", expr),
            Expr::Set(expr) => write!(f, "{}", expr),
            Expr::This(expr) => write!(f, "{}", expr),
            Expr::Super(expr) => write!(f, "{}", expr),
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
            _ => unreachable!(), // parser should contruct only literal expr that contain primitves
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

impl Display for GetExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(get-property {} {})", self.object, self.name.lexeme)
    }
}

impl Display for SetExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(set-property {} {} {})",
            self.object, self.name.lexeme, self.value
        )
    }
}

impl Display for ThisExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(this)")
    }
}

impl Display for SuperExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(super)")
    }
}
