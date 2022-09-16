use super::statement::ExprStmt;
use super::statement::PrintStmt;
use super::statement::VarStmt;
use std::fmt::Display;

impl Display for PrintStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(print {})", self.expr)
    }
}

impl Display for ExprStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(stmt {})", self.expr)
    }
}

impl Display for VarStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.expr {
            Some(ref e) => write!(f, "(new-var {} = {})", self.token.lexeme, e),
            None => write!(f, "(new-var {})", self.token.lexeme),
        }
    }
}
