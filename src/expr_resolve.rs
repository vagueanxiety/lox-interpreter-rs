use crate::expr::*;
use crate::resolver::ClassType;
use crate::resolver::ResolutionError;
use crate::resolver::Resolver;
use crate::resolver::Result;

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
            Expr::GetExpr(expr) => expr.resolve(resolver),
            Expr::SetExpr(expr) => expr.resolve(resolver),
            Expr::ThisExpr(expr) => expr.resolve(resolver),
        }
    }
}

impl VarExpr {
    fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
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
    fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        self.value.resolve(resolver)?;
        self.scope_offset = resolver.resolve_local(&self.name);
        Ok(())
    }
}

impl BinaryExpr {
    fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        self.left.resolve(resolver)?;
        self.right.resolve(resolver)
    }
}

impl UnaryExpr {
    fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        self.right.resolve(resolver)
    }
}

impl CallExpr {
    fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        self.callee.resolve(resolver)?;
        for a in self.args.iter_mut() {
            a.resolve(resolver)?;
        }
        Ok(())
    }
}

impl GroupingExpr {
    fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        self.expr.resolve(resolver)
    }
}

impl LiteralExpr {
    fn resolve(&mut self, _resolver: &mut Resolver) -> Result<()> {
        Ok(())
    }
}

impl LogicalExpr {
    fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        self.left.resolve(resolver)?;
        self.right.resolve(resolver)
    }
}

impl GetExpr {
    fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        self.object.resolve(resolver)
    }
}

impl SetExpr {
    fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        self.object.resolve(resolver)?;
        self.value.resolve(resolver)
    }
}

impl ThisExpr {
    fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        if resolver.current_cls == ClassType::NonClass {
            return Err(ResolutionError::new(
                &self.keyword,
                "Can't use 'this' outside of a class.",
            ));
        }
        self.scope_offset = resolver.resolve_local(&self.keyword);
        Ok(())
    }
}
