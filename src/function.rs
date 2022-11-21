use crate::environment::Environment;
use crate::environment::EnvironmentTree;
use crate::expr_interpret::Result;
use crate::literal::Literal;
use crate::statement::FunctionStmt;
use crate::stmt_interpret::ExecError;
use indextree::NodeId;
use std::cell::RefCell;
use std::fmt::Display;
use std::io::Write;
use std::rc::Rc;

// TODO: not sure if a Callable trait would be beneficial?
pub struct LoxFunction {
    declaration: Rc<RefCell<FunctionStmt>>,
    closure: NodeId,
    is_initializer: bool,
}

impl PartialEq for LoxFunction {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.declaration, &other.declaration) && self.closure == other.closure
    }
}

impl Display for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.declaration.borrow())
    }
}

impl LoxFunction {
    pub fn new(
        declaration: Rc<RefCell<FunctionStmt>>,
        closure: NodeId,
        is_initializer: bool,
    ) -> Self {
        LoxFunction {
            declaration,
            closure,
            is_initializer,
        }
    }

    pub fn call<T: Write>(
        &self,
        args: Vec<Rc<Literal>>,
        env: &mut EnvironmentTree,
        output: &mut T,
    ) -> Result<Rc<Literal>> {
        let prev = env.checkout(self.closure);
        let mut return_value = Rc::new(Literal::Empty);
        if self.is_initializer {
            return_value = env.get_at("this", 0).expect("Missing instance").clone()
        }

        env.push(Environment::new());
        for (i, p) in self.declaration.borrow().params.iter().enumerate() {
            env.define(p.lexeme.clone(), args[i].clone());
        }

        for s in self.declaration.borrow().body.iter() {
            match s.execute(env, output) {
                Ok(_) => {}
                Err(ExecError::Return(value)) => {
                    if !self.is_initializer {
                        return_value = value;
                    }
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
        self.declaration.borrow().params.len()
    }

    pub fn bind(&self, env: &mut EnvironmentTree, instance: Rc<Literal>) -> Self {
        let prev = env.checkout(self.closure);
        env.push(Environment::new());
        env.define("this".to_string(), instance);
        let cur = env.keep_branch();
        env.checkout(prev);
        LoxFunction::new(self.declaration.clone(), cur, self.is_initializer)
    }
}
