use super::environment::Environment;
use super::environment::EnvironmentTree;
use super::expr_interpret::RuntimeError;
use super::function::LoxFunction;
use super::literal::Literal;
use super::statement::*;
use std::io;
use std::io::Write;
use std::rc::Rc;

// TODO: can we shortcircuit success value to avoid this?
pub enum ExecError {
    RuntimeError(RuntimeError),
    Return(Rc<Literal>),
}

pub type Result<T> = std::result::Result<T, ExecError>;

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

impl Stmt {
    pub fn execute<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<()> {
        match self {
            Stmt::ExprStmt(s) => s.execute(env, output),
            Stmt::PrintStmt(s) => s.execute(env, output),
            Stmt::VarStmt(s) => s.execute(env, output),
            Stmt::BlockStmt(s) => s.execute(env, output),
            Stmt::IfStmt(s) => s.execute(env, output),
            Stmt::WhileStmt(s) => s.execute(env, output),
            Stmt::FunctionStmt(s) => s.execute(env, output),
            Stmt::ReturnStmt(s) => s.execute(env, output),
        }
    }
}

impl PrintStmt {
    pub fn execute<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<()> {
        let value = self.expr.eval(env, output)?;
        write!(output, "{value}\n")?;
        Ok(())
    }
}

impl ExprStmt {
    pub fn execute<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<()> {
        self.expr.eval(env, output)?;
        Ok(())
    }
}

impl VarStmt {
    pub fn execute<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<()> {
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
    pub fn execute<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<()> {
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
    pub fn execute<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<()> {
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
    pub fn execute<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<()> {
        while self.condition.eval(env, output)?.is_truthy() {
            self.body.execute(env, output)?;
        }
        Ok(())
    }
}

impl FunctionStmt {
    pub fn execute<T: Write>(
        self: &Rc<Self>,
        env: &mut EnvironmentTree,
        _output: &mut T,
    ) -> Result<()> {
        let cur_env = env.keep_branch();
        let fun = LoxFunction::new(self.clone(), cur_env);
        env.define(
            self.name.lexeme.clone(),
            Rc::new(Literal::FunctionLiteral(fun)),
        );
        Ok(())
    }
}

impl ReturnStmt {
    pub fn execute<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<()> {
        if let Some(ref expr) = self.value {
            Err(ExecError::Return(expr.eval(env, output)?))
        } else {
            Err(ExecError::Return(Rc::new(Literal::Empty)))
        }
    }
}
