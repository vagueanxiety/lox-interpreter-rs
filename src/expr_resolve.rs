use crate::expr::*;
use crate::{resolver::Resolver, token::Token};
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ResolutionError {
    pub msg: String,
}

impl fmt::Display for ResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ResolutionError: {}", self.msg)
    }
}

impl ResolutionError {
    pub fn new(t: &Token, msg: &str) -> ResolutionError {
        let full_msg = format!("[line {}] {}", t.line, msg);
        ResolutionError { msg: full_msg }
    }
}

impl Error for ResolutionError {}

pub type Result<T> = std::result::Result<T, ResolutionError>;

impl Expr {
    pub fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        match self {
            Expr::LiteralExpr(expr) => expr.resolve(resolver),
            Expr::BinaryExpr(expr) => expr.resolve(resolver),
            Expr::UnaryExpr(expr) => expr.resolve(resolver),
            Expr::GroupingExpr(expr) => expr.resolve(resolver),
            Expr::VarExpr(expr) => expr.resolve(resolver),
            Expr::AssignExpr(expr) => expr.resolve(resolver),
            Expr::LogicalExpr(expr) => expr.resolve(resolver),
            Expr::CallExpr(expr) => expr.resolve(resolver),
        }
    }
}

impl VarExpr {
    pub fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        if let Some(&b) = resolver.get(&self.name) {
            if b == false {
                return Err(ResolutionError::new(
                    &self.name,
                    "Can't read local variable in its own initializer.",
                ));
            }
        }

        self.scope_offset = resolver.resolve_local(&self.name);
        Ok(())
    }
}

impl AssignExpr {
    pub fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        self.value.resolve(resolver)?;
        self.scope_offset = resolver.resolve_local(&self.name);
        Ok(())
    }
}

impl BinaryExpr {
    pub fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        self.left.resolve(resolver)?;
        self.right.resolve(resolver)
    }
}

impl UnaryExpr {
    pub fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        self.right.resolve(resolver)
    }
}

impl CallExpr {
    pub fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        self.callee.resolve(resolver)?;
        for a in self.args.iter_mut() {
            a.resolve(resolver)?;
        }
        Ok(())
    }
}

impl GroupingExpr {
    pub fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        self.expr.resolve(resolver)
    }
}

impl LiteralExpr {
    pub fn resolve(&mut self, _resolver: &mut Resolver) -> Result<()> {
        Ok(())
    }
}

impl LogicalExpr {
    pub fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        self.left.resolve(resolver)?;
        self.right.resolve(resolver)
    }
}
