use super::environment::Environment;
use super::expr_interpret::ExprInterpret;
use super::expr_interpret::RuntimeError;
use super::literal::Literal;
use super::statement::ExprStmt;
use super::statement::PrintStmt;
use super::statement::VarStmt;

pub trait StmtInterpret {
    fn execute(&self, env: &mut Environment) -> Result<(), RuntimeError>;
}

impl StmtInterpret for PrintStmt {
    fn execute(&self, env: &mut Environment) -> Result<(), RuntimeError> {
        let value = self.expr.eval(env)?;
        println!("{value}");
        Ok(())
    }
}

// TODO: ugh... can we make env immutable..
impl StmtInterpret for ExprStmt {
    fn execute(&self, env: &mut Environment) -> Result<(), RuntimeError> {
        self.expr.eval(env)?;
        Ok(())
    }
}

// TODO: ugh.. too much copying, maybe use mem::take?
impl StmtInterpret for VarStmt {
    fn execute(&self, env: &mut Environment) -> Result<(), RuntimeError> {
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
