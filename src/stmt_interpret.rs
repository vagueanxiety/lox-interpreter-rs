use super::expr_interpret::RuntimeError;
use super::statement::ExprStmt;
use super::statement::PrintStmt;

pub trait StmtInterpret {
    fn eval(&self) -> Result<(), RuntimeError>;
}

impl StmtInterpret for PrintStmt {
    fn eval(&self) -> Result<(), RuntimeError> {
        let value = self.expr.eval()?;
        println!("{value}");
        Ok(())
    }
}

impl StmtInterpret for ExprStmt {
    fn eval(&self) -> Result<(), RuntimeError> {
        self.expr.eval()?;
        Ok(())
    }
}
