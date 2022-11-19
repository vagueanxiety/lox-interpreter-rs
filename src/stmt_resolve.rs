use crate::resolver::ClassType;
use crate::resolver::FunctionType;
use crate::resolver::ResolutionError;
use crate::resolver::Result;
use crate::statement::*;
use crate::{resolver::Resolver, statement::Stmt};
use std::mem;

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
            Stmt::ClassStmt(s) => s.resolve(resolver),
        }
    }
}

impl BlockStmt {
    fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        resolver.begin_scope();
        for s in self.statements.iter_mut() {
            s.resolve(resolver)?
        }
        resolver.end_scope();
        Ok(())
    }
}

impl VarStmt {
    fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        resolver.declare(&self.name)?;
        if let Some(ref mut initializer) = self.value {
            initializer.resolve(resolver)?;
        }
        resolver.define(&self.name);
        Ok(())
    }
}

impl FunctionStmt {
    fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        resolver.declare(&self.name)?;
        resolver.define(&self.name);
        self.resolve_fn(resolver, FunctionType::Fun)
    }

    fn resolve_fn(&mut self, resolver: &mut Resolver, mut fun_type: FunctionType) -> Result<()> {
        mem::swap(&mut fun_type, &mut resolver.current_fun);
        resolver.begin_scope();

        for p in &self.params {
            resolver.declare(p)?;
            resolver.define(p);
        }
        for s in self.body.iter_mut() {
            s.resolve(resolver)?;
        }

        resolver.end_scope();
        mem::swap(&mut fun_type, &mut resolver.current_fun);

        Ok(())
    }
}

impl ExprStmt {
    fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        self.expr.resolve(resolver)
    }
}

impl IfStmt {
    fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        self.condition.resolve(resolver)?;
        self.then_branch.resolve(resolver)?;
        if let Some(ref mut else_branch) = self.else_branch {
            else_branch.resolve(resolver)?;
        }
        Ok(())
    }
}

impl PrintStmt {
    fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        self.expr.resolve(resolver)
    }
}

impl ReturnStmt {
    fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        if resolver.current_fun == FunctionType::NonFun {
            return Err(ResolutionError::new(
                &self.keyword,
                "Can't return from top-level code.",
            ));
        }

        if let Some(ref mut value) = self.value {
            value.resolve(resolver)?;
        }
        Ok(())
    }
}

impl WhileStmt {
    fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        self.condition.resolve(resolver)?;
        self.body.resolve(resolver)?;
        Ok(())
    }
}

impl ClassStmt {
    fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        let mut current_cls = ClassType::Class;
        mem::swap(&mut current_cls, &mut resolver.current_cls);

        resolver.declare(&self.name)?;
        resolver.define(&self.name);

        resolver.begin_scope();
        resolver
            .peek()
            .expect("Empty scopes")
            .insert("this".to_string(), true);

        for fs in &self.methods {
            fs.borrow_mut().resolve_fn(resolver, FunctionType::Method)?;
        }

        resolver.end_scope();

        mem::swap(&mut current_cls, &mut resolver.current_cls);
        Ok(())
    }
}
