use super::environment::Environments;
use super::expr_interpret::ExprInterpret;
use super::expr_interpret::RuntimeError;
use super::literal::Literal;
use super::statement::*;
use std::io::Write;

pub trait StmtInterpret {
    fn execute<T: Write>(&self, env: &mut Environments, output: &mut T)
        -> Result<(), RuntimeError>;
}

impl StmtInterpret for PrintStmt {
    fn execute<T: Write>(
        &self,
        env: &mut Environments,
        output: &mut T,
    ) -> Result<(), RuntimeError> {
        let value = self.expr.eval(env)?;
        write!(output, "{value}\n")?;
        Ok(())
    }
}

impl StmtInterpret for ExprStmt {
    fn execute<T: Write>(
        &self,
        env: &mut Environments,
        _output: &mut T,
    ) -> Result<(), RuntimeError> {
        self.expr.eval(env)?;
        Ok(())
    }
}

// TODO: ugh.. too much copying, maybe use mem::take?
impl StmtInterpret for VarStmt {
    fn execute<T: Write>(
        &self,
        env: &mut Environments,
        _output: &mut T,
    ) -> Result<(), RuntimeError> {
        match self.expr {
            Some(ref e) => {
                let value = e.eval(env)?;
                env.define(self.token.lexeme.clone(), value);
            }
            None => env.define(self.token.lexeme.clone(), Literal::Empty),
        }
        Ok(())
    }
}

impl StmtInterpret for BlockStmt {
    fn execute<T: Write>(
        &self,
        env: &mut Environments,
        output: &mut T,
    ) -> Result<(), RuntimeError> {
        // Note that it is important to keep the invariant regarding environment
        // Otherwise it might accidentally pop the root env and panic afterwards
        env.push();
        for s in self.statements.iter() {
            s.execute(env, output)?
        }
        env.pop();
        Ok(())
    }
}
