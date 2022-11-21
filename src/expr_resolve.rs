use crate::expr::*;
use crate::resolver::ClassType;
use crate::resolver::ResolutionError;
use crate::resolver::Resolver;
use crate::resolver::Result;

impl Expr {
    pub fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        match self {
            Expr::Literal(expr) => expr.resolve(resolver),
            Expr::Binary(expr) => expr.resolve(resolver),
            Expr::Unary(expr) => expr.resolve(resolver),
            Expr::Grouping(expr) => expr.resolve(resolver),
            Expr::Var(expr) => expr.resolve(resolver),
            Expr::Assign(expr) => expr.resolve(resolver),
            Expr::Logical(expr) => expr.resolve(resolver),
            Expr::Call(expr) => expr.resolve(resolver),
            Expr::Get(expr) => expr.resolve(resolver),
            Expr::Set(expr) => expr.resolve(resolver),
            Expr::This(expr) => expr.resolve(resolver),
            Expr::Super(expr) => expr.resolve(resolver),
        }
    }
}

impl VarExpr {
    pub fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        if let Some(&b) = resolver.get(&self.name) {
            if !b {
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

impl GetExpr {
    pub fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        self.object.resolve(resolver)
    }
}

impl SetExpr {
    pub fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        self.object.resolve(resolver)?;
        self.value.resolve(resolver)
    }
}

impl ThisExpr {
    pub fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
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

impl SuperExpr {
    pub fn resolve(&mut self, resolver: &mut Resolver) -> Result<()> {
        if resolver.current_cls == ClassType::NonClass {
            return Err(ResolutionError::new(
                &self.keyword,
                "Can't use 'super' outside of a class.",
            ));
        } else if resolver.current_cls != ClassType::Subclass {
            return Err(ResolutionError::new(
                &self.keyword,
                "Can't use 'super' in a class with no superclass.",
            ));
        }

        self.scope_offset = resolver.resolve_local(&self.keyword);
        Ok(())
    }
}
