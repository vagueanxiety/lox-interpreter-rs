use super::environment::EnvironmentTree;
use super::expr_interpret::Result;
use super::literal::Literal;
use super::statement::FunctionStmt;
use indextree::NodeId;
use std::fmt::Display;
use std::io::Write;
use std::rc::Rc;

// TODO: native function (e.g. clock)
// native function class parameterized by call back?

#[derive(Clone)]
pub struct LoxFunction {
    pub declaration: Rc<FunctionStmt>,
    pub closure: NodeId,
}

impl PartialEq for LoxFunction {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.declaration, &other.declaration) && self.closure == other.closure
    }
}

impl Display for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.declaration)
    }
}

impl LoxFunction {
    pub fn new(declaration: Rc<FunctionStmt>, closure: NodeId) -> LoxFunction {
        LoxFunction {
            declaration,
            closure,
        }
    }

    // TODO: return
    pub fn call<T: Write>(
        &self,
        args: Vec<Rc<Literal>>,
        env: &mut EnvironmentTree,
        output: &mut T,
    ) -> Result<()> {
        let prev = env.checkout(self.closure);
        for (i, p) in self.declaration.params.iter().enumerate() {
            env.define(p.lexeme.clone(), args[i].clone());
        }

        for s in self.declaration.body.iter() {
            s.execute(env, output)?
        }
        env.checkout(prev);
        Ok(())
    }

    pub fn arity(&self) -> usize {
        self.declaration.params.len()
    }
}
