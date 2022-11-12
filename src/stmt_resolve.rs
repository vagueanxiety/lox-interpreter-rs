use crate::expr_resolve::Result;
use crate::statement::*;
use crate::{resolver::Resolver, statement::Stmt};

impl Stmt {
    pub fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        match self {
            Stmt::ExprStmt(s) => s.resolve(resolver),
            Stmt::PrintStmt(s) => s.resolve(resolver),
            Stmt::VarStmt(s) => s.resolve(resolver),
            Stmt::BlockStmt(s) => s.resolve(resolver),
            Stmt::IfStmt(s) => s.resolve(resolver),
            Stmt::WhileStmt(s) => s.resolve(resolver),
            Stmt::ReturnStmt(s) => s.resolve(resolver),
            Stmt::FunctionStmt(s) => s.borrow_mut().resolve(resolver),
        }
    }
}

impl BlockStmt {
    pub fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        resolver.begin_scope();
        for s in self.statements.iter_mut() {
            s.resolve(resolver)?
        }
        resolver.end_scope();
        Ok(())
    }
}

impl VarStmt {
    pub fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        resolver.declare(&self.name);
        if let Some(ref mut initializer) = self.value {
            initializer.resolve(resolver)?;
        }
        resolver.define(&self.name);
        Ok(())
    }
}

impl FunctionStmt {
    pub fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        resolver.declare(&self.name);
        resolver.define(&self.name);
        self.resolve_fn(resolver)
    }

    fn resolve_fn(&mut self, resolver: &mut Resolver) -> Result<()> {
        resolver.begin_scope();
        for p in &self.params {
            resolver.declare(p);
            resolver.define(p);
        }
        for s in self.body.iter_mut() {
            s.resolve(resolver)?;
        }
        resolver.end_scope();
        Ok(())
    }
}

impl ExprStmt {
    pub fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        self.expr.resolve(resolver)
    }
}

impl IfStmt {
    pub fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        self.condition.resolve(resolver)?;
        self.then_branch.resolve(resolver)?;
        if let Some(ref mut else_branch) = self.else_branch {
            else_branch.resolve(resolver)?;
        }
        Ok(())
    }
}

impl PrintStmt {
    pub fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        self.expr.resolve(resolver)
    }
}

impl ReturnStmt {
    pub fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        if let Some(ref mut value) = self.value {
            value.resolve(resolver)?;
        }
        Ok(())
    }
}

impl WhileStmt {
    pub fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        self.condition.resolve(resolver)?;
        self.body.resolve(resolver)?;
        Ok(())
    }
}
