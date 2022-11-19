use crate::class::LoxClass;
use crate::environment::Environment;
use crate::environment::EnvironmentTree;
use crate::expr_interpret::RuntimeError;
use crate::function::LoxFunction;
use crate::literal::Literal;
use crate::statement::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::rc::Rc;

// TODO: can we shortcircuit success value to avoid this?
pub enum ExecError {
    RuntimeError(RuntimeError),
    Return(Rc<Literal>),
}

impl From<io::Error> for ExecError {
    fn from(error: io::Error) -> Self {
        ExecError::RuntimeError(RuntimeError {
            msg: format!("RuntimeError caused by an IO error: {error}"),
        })
    }
}

impl From<RuntimeError> for ExecError {
    fn from(error: RuntimeError) -> Self {
        ExecError::RuntimeError(error)
    }
}

pub type Result<T> = std::result::Result<T, ExecError>;

impl Stmt {
    pub fn execute<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<()> {
        match self {
            Stmt::ExprStmt(s) => s.execute(env, output),
            Stmt::PrintStmt(s) => s.execute(env, output),
            Stmt::VarStmt(s) => s.execute(env, output),
            Stmt::BlockStmt(s) => s.execute(env, output),
            Stmt::IfStmt(s) => s.execute(env, output),
            Stmt::WhileStmt(s) => s.execute(env, output),
            Stmt::ReturnStmt(s) => s.execute(env, output),
            Stmt::FunctionStmt(s) => FunctionStmt::execute(s, env, output),
            Stmt::ClassStmt(s) => s.execute(env, output),
        }
    }
}

impl PrintStmt {
    fn execute<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<()> {
        let value = self.expr.eval(env, output)?;
        write!(output, "{value}\n")?;
        Ok(())
    }
}

impl ExprStmt {
    fn execute<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<()> {
        self.expr.eval(env, output)?;
        Ok(())
    }
}

impl VarStmt {
    fn execute<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<()> {
        match self.value {
            Some(ref e) => {
                let value = e.eval(env, output)?;
                env.define(self.name.lexeme.clone(), value);
            }
            None => env.define(self.name.lexeme.clone(), Rc::new(Literal::Empty)),
        }
        Ok(())
    }
}

impl BlockStmt {
    fn execute<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<()> {
        // Note that it is important to keep the invariant regarding environment
        // Otherwise it might accidentally pop the root env and panic afterwards
        env.push(Environment::new());
        for s in self.statements.iter() {
            s.execute(env, output)?
        }
        env.pop();
        Ok(())
    }
}

impl IfStmt {
    fn execute<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<()> {
        if self.condition.eval(env, output)?.is_truthy() {
            self.then_branch.execute(env, output)
        } else if let Some(ref s) = self.else_branch {
            s.execute(env, output)
        } else {
            Ok(())
        }
    }
}

impl WhileStmt {
    fn execute<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<()> {
        while self.condition.eval(env, output)?.is_truthy() {
            self.body.execute(env, output)?;
        }
        Ok(())
    }
}

// TODO: would be nice if we can do Rc<RefCell<Self>>
// in general FunctionStmt is a special case that I should think about
// gettting rid of, while not losing much of its benefits if possible
impl FunctionStmt {
    fn execute<T: Write>(
        self_: &Rc<RefCell<FunctionStmt>>,
        env: &mut EnvironmentTree,
        _output: &mut T,
    ) -> Result<()> {
        let cur_env = env.keep_branch();
        let fun = LoxFunction::new(self_.clone(), cur_env);
        env.define(
            self_.borrow().name.lexeme.clone(),
            Rc::new(Literal::FunctionLiteral(fun)),
        );
        Ok(())
    }
}

impl ReturnStmt {
    fn execute<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<()> {
        if let Some(ref expr) = self.value {
            Err(ExecError::Return(expr.eval(env, output)?))
        } else {
            Err(ExecError::Return(Rc::new(Literal::Empty)))
        }
    }
}

impl ClassStmt {
    fn execute<T: Write>(&self, env: &mut EnvironmentTree, _output: &mut T) -> Result<()> {
        env.define(self.name.lexeme.clone(), Rc::new(Literal::Empty));

        // building methods
        let cur_env = env.keep_branch();
        let mut methods = HashMap::new();
        for fs in &self.methods {
            let method = LoxFunction::new(fs.clone(), cur_env);
            methods.insert(fs.borrow().name.lexeme.clone(), method);
        }

        let class = LoxClass::new(self.name.lexeme.clone(), methods);
        env.assign(
            &self.name,
            Rc::new(Literal::ClassLiteral(Rc::new(class))),
            Some(0),
        )?;
        Ok(())
    }
}
