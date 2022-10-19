use super::environment::Environment;
use super::environment::EnvironmentTree;
use super::expr_interpret::ExprInterpret;
use super::expr_interpret::Result;
use super::literal::Literal;
use super::statement::*;
use std::io::Write;

pub trait StmtInterpret {
    fn execute<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<()>;
}

impl StmtInterpret for PrintStmt {
    fn execute<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<()> {
        let value = self.expr.eval(env)?;
        write!(output, "{value}\n")?;
        Ok(())
    }
}

impl StmtInterpret for ExprStmt {
    fn execute<T: Write>(&self, env: &mut EnvironmentTree, _output: &mut T) -> Result<()> {
        self.expr.eval(env)?;
        Ok(())
    }
}

// TODO: ugh.. too much copying, maybe use mem::take?
impl StmtInterpret for VarStmt {
    fn execute<T: Write>(&self, env: &mut EnvironmentTree, _output: &mut T) -> Result<()> {
        match self.value {
            Some(ref e) => {
                let value = e.eval(env)?;
                env.define(self.name.lexeme.clone(), value);
            }
            None => env.define(self.name.lexeme.clone(), Literal::Empty),
        }
        Ok(())
    }
}

impl StmtInterpret for BlockStmt {
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

impl StmtInterpret for IfStmt {
    fn execute<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<()> {
        if self.condition.eval(env)?.is_truthy() {
            self.then_branch.execute(env, output)
        } else if let Some(ref s) = self.else_branch {
            s.execute(env, output)
        } else {
            Ok(())
        }
    }
}

impl StmtInterpret for WhileStmt {
    fn execute<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<()> {
        while self.condition.eval(env)?.is_truthy() {
            self.body.execute(env, output)?;
        }
        Ok(())
    }
}
