use super::environment::Environments;
use super::expr_interpret::ExprInterpret;
use super::expr_interpret::RuntimeError;
use super::literal::Literal;
use super::statement::*;

pub trait StmtInterpret {
    fn execute(&self, env: &mut Environments) -> Result<(), RuntimeError>;
}

impl StmtInterpret for PrintStmt {
    fn execute(&self, env: &mut Environments) -> Result<(), RuntimeError> {
        let value = self.expr.eval(env)?;
        println!("{value}");
        Ok(())
    }
}

impl StmtInterpret for ExprStmt {
    fn execute(&self, env: &mut Environments) -> Result<(), RuntimeError> {
        self.expr.eval(env)?;
        Ok(())
    }
}

// TODO: ugh.. too much copying, maybe use mem::take?
impl StmtInterpret for VarStmt {
    fn execute(&self, env: &mut Environments) -> Result<(), RuntimeError> {
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
    fn execute(&self, env: &mut Environments) -> Result<(), RuntimeError> {
        // Note that it is important to keep the invariant regarding environment
        // Otherwise it might accidentally pop the root env and panic afterwards
        env.push();
        for s in self.statements.iter() {
            s.execute(env)?
        }
        env.pop();
        Ok(())
    }
}
