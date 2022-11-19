use crate::statement::*;
use std::fmt::Display;

// TODO: probably should use the crate enum_dispatch
impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::ExprStmt(s) => write!(f, "{}", s),
            Stmt::PrintStmt(s) => write!(f, "{}", s),
            Stmt::VarStmt(s) => write!(f, "{}", s),
            Stmt::BlockStmt(s) => write!(f, "{}", s),
            Stmt::IfStmt(s) => write!(f, "{}", s),
            Stmt::WhileStmt(s) => write!(f, "{}", s),
            Stmt::ReturnStmt(s) => write!(f, "{}", s),
            Stmt::FunctionStmt(s) => write!(f, "{}", s.borrow()),
            Stmt::ClassStmt(s) => write!(f, "{}", s),
        }
    }
}

// TODO: the output of the ast printer is not pretty right now, we need to
// keep track of indentation information to do pretty print
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
            Some(ref e) => write!(f, "(new-var {} {})", self.name.lexeme, e),
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

impl Display for WhileStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(while {}\n{}\n)", self.condition, self.body)
    }
}

impl Display for FunctionStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut body_string = String::new();
        for s in &self.body {
            body_string = format!("{}\n{}", body_string, s);
        }

        let param_string = self
            .params
            .iter()
            .fold(String::new(), |acc, p| acc + &p.lexeme + " ");
        let param_string = param_string.trim_end();

        write!(
            f,
            "(fun-start {} ({}){}\nfun-end)",
            self.name.lexeme, param_string, body_string
        )
    }
}

impl Display for ReturnStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value {
            Some(ref value) => write!(f, "(return {})", value),
            None => write!(f, "(return nil)"),
        }
    }
}

impl Display for ClassStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut method_string = String::new();
        for m in &self.methods {
            method_string = format!("{}{}\n", method_string, m.borrow());
        }
        write!(
            f,
            "(class-start {}\n{}class-end)",
            self.name.lexeme, method_string
        )
    }
}
