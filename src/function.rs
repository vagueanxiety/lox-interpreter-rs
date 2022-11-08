use super::environment::Environment;
use super::environment::EnvironmentTree;
use super::expr_interpret::Result;
use super::literal::Literal;
use super::statement::FunctionStmt;
use super::stmt_interpret::ExecError;
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

    pub fn call<T: Write>(
        &self,
        args: Vec<Rc<Literal>>,
        env: &mut EnvironmentTree,
        output: &mut T,
    ) -> Result<Rc<Literal>> {
        let prev = env.checkout(self.closure);

        env.push(Environment::new());
        for (i, p) in self.declaration.params.iter().enumerate() {
            env.define(p.lexeme.clone(), args[i].clone());
        }

        let mut return_value = Rc::new(Literal::Empty);
        for s in self.declaration.body.iter() {
            match s.execute(env, output) {
                Ok(_) => {}
                Err(ExecError::Return(value)) => {
                    return_value = value;
                    break;
                }
                Err(ExecError::RuntimeError(error)) => {
                    return Err(error);
                }
            }
        }
        env.pop();
        env.checkout(prev);

        Ok(return_value)
    }

    pub fn arity(&self) -> usize {
        self.declaration.params.len()
    }
}
