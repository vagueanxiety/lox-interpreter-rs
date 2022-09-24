use super::statement::*;
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
        match self.value {
            Some(ref e) => write!(f, "(new-var {} = {})", self.name.lexeme, e),
            None => write!(f, "(new-var {})", self.name.lexeme),
        }
    }
}

impl Display for BlockStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stmt_string = String::new();
        for s in &self.statements {
            stmt_string = format!("{}{}\n", stmt_string, s);
        }
        write!(f, "(block-start\n{}block-end)", stmt_string)
    }
}

impl Display for IfStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.else_branch {
            Some(ref s) => write!(
                f,
                "(if {}\n{}\nelse\n{}\n)",
                self.condition, self.then_branch, s
            ),
            None => write!(f, "(if {}\n{}\n)", self.condition, self.then_branch),
        }
    }
}
