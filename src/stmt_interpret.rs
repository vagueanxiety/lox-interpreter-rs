use super::environment::Environment;
use super::environment::EnvironmentTree;
use super::expr_interpret::Result;
use super::expr_interpret::RuntimeError;
use super::function::LoxFunction;
use super::literal::Literal;
use super::statement::*;
use std::io::Write;
use std::rc::Rc;

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
        if let Some(cur_env) = env.keep_branch() {
            let fun = LoxFunction::new(self.clone(), cur_env);
            env.define(
                self.name.lexeme.clone(),
                Rc::new(Literal::FunctionLiteral(fun)),
            );
            Ok(())
        } else {
            Err(RuntimeError::new(
                &self.name,
                "Function definition has no surrounding environment",
            ))
        }
    }
}
