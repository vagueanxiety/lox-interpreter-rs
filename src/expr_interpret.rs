use super::environment::EnvironmentTree;
use super::expr::*;
use super::literal::Literal;
use super::token::Token;
use super::token::TokenType;
use std::borrow::Borrow;
use std::error::Error;
use std::fmt;
use std::io::Write;
use std::rc::Rc;

#[derive(Debug)]
pub struct RuntimeError {
    pub msg: String,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RuntimeError: {}", self.msg)
    }
}

impl Error for RuntimeError {}

impl RuntimeError {
    pub fn new(t: &Token, msg: &str) -> RuntimeError {
        let full_msg = format!("[line {}] {}", t.line, msg);
        RuntimeError { msg: full_msg }
    }
}

pub type Result<T> = std::result::Result<T, RuntimeError>;

// TODO: probably should use the crate enum_dispatch
impl Expr {
    pub fn eval<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<Rc<Literal>> {
        match self {
            Expr::LiteralExpr(expr) => expr.eval(env, output),
            Expr::BinaryExpr(expr) => expr.eval(env, output),
            Expr::UnaryExpr(expr) => expr.eval(env, output),
            Expr::GroupingExpr(expr) => expr.eval(env, output),
            Expr::VarExpr(expr) => expr.eval(env, output),
            Expr::AssignExpr(expr) => expr.eval(env, output),
            Expr::LogicalExpr(expr) => expr.eval(env, output),
            Expr::CallExpr(expr) => expr.eval(env, output),
        }
    }
}

impl LiteralExpr {
    pub fn eval<T: Write>(
        &self,
        _env: &mut EnvironmentTree,
        _output: &mut T,
    ) -> Result<Rc<Literal>> {
        Ok(Rc::new(self.value.clone()))
    }
}

impl GroupingExpr {
    pub fn eval<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<Rc<Literal>> {
        self.expr.eval(env, output)
    }
}

impl UnaryExpr {
    pub fn eval<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<Rc<Literal>> {
        let rhs = self.right.eval(env, output)?;
        match self.operator.token_type {
            TokenType::BANG => Ok(Rc::new(Literal::BoolLiteral(rhs.is_truthy()))),
            TokenType::MINUS => match rhs.negative() {
                Ok(x) => Ok(Rc::new(x)),
                _ => Err(RuntimeError::new(
                    &self.operator,
                    &format!(
                        "{} cannot be applied to {}, it must be a number",
                        self.operator.lexeme, rhs
                    ),
                )),
            },
            ref tt => Err(RuntimeError::new(
                &self.operator,
                &format!("{:?} is unimplemented", tt),
            )),
        }
    }
}

impl BinaryExpr {
    pub fn eval<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<Rc<Literal>> {
        let lhs = self.left.eval(env, output)?;
        let rhs = self.right.eval(env, output)?;
        match self.operator.token_type {
            TokenType::MINUS => match lhs.minus(&rhs) {
                Ok(x) => Ok(Rc::new(x)),
                _ => Err(RuntimeError::new(
                    &self.operator,
                    &format!(
                        "{} cannot be applied to {} and {}, both must be number",
                        self.operator.lexeme, lhs, rhs
                    ),
                )),
            },
            TokenType::PLUS => match lhs.plus(&rhs) {
                Ok(x) => Ok(Rc::new(x)),
                _ => Err(RuntimeError::new(
                    &self.operator,
                    &format!(
                        "{} cannot be applied to {}, must be two numbers or two strings",
                        self.operator.lexeme, rhs
                    ),
                )),
            },
            TokenType::STAR => match lhs.multiply(&rhs) {
                Ok(x) => Ok(Rc::new(x)),
                _ => Err(RuntimeError::new(
                    &self.operator,
                    &format!(
                        "{} cannot be applied to {} and {}, both must be number",
                        self.operator.lexeme, lhs, rhs
                    ),
                )),
            },
            TokenType::SLASH => match rhs.divide(&lhs) {
                Ok(x) => Ok(Rc::new(x)),
                _ => Err(RuntimeError::new(
                    &self.operator,
                    &format!(
                        "{} cannot be applied to {} and {}, both must be number",
                        self.operator.lexeme, lhs, rhs
                    ),
                )),
            },
            TokenType::EQUAL_EQUAL => Ok(Rc::new(lhs.equal(&rhs))),
            TokenType::BANG_EQUAL => Ok(Rc::new(lhs.not_equal(&rhs))),
            TokenType::GREATER => match lhs.greater(&rhs) {
                Ok(x) => Ok(Rc::new(x)),
                _ => Err(RuntimeError::new(
                    &self.operator,
                    &format!(
                        "{} cannot be applied to {} and {}, both must be number",
                        self.operator.lexeme, lhs, rhs
                    ),
                )),
            },
            TokenType::GREATER_EQUAL => match lhs.greater_equal(&rhs) {
                Ok(x) => Ok(Rc::new(x)),
                _ => Err(RuntimeError::new(
                    &self.operator,
                    &format!(
                        "{} cannot be applied to {} and {}, both must be number",
                        self.operator.lexeme, lhs, rhs
                    ),
                )),
            },
            TokenType::LESS => match lhs.less(&rhs) {
                Ok(x) => Ok(Rc::new(x)),
                _ => Err(RuntimeError::new(
                    &self.operator,
                    &format!(
                        "{} cannot be applied to {} and {}, both must be number",
                        self.operator.lexeme, lhs, rhs
                    ),
                )),
            },
            TokenType::LESS_EQUAL => match lhs.less_equal(&rhs) {
                Ok(x) => Ok(Rc::new(x)),
                _ => Err(RuntimeError::new(
                    &self.operator,
                    &format!(
                        "{} cannot be applied to {} and {}, both must be number",
                        self.operator.lexeme, lhs, rhs
                    ),
                )),
            },
            ref tt => Err(RuntimeError::new(
                &self.operator,
                &format!("{:?} is unimplemented", tt),
            )),
        }
    }
}

impl VarExpr {
    pub fn eval<T: Write>(
        &self,
        env: &mut EnvironmentTree,
        _output: &mut T,
    ) -> Result<Rc<Literal>> {
        Ok(env.get(&self.name)?.clone())
    }
}

impl AssignExpr {
    pub fn eval<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<Rc<Literal>> {
        let value = self.value.eval(env, output)?;
        env.assign(&self.name, value.clone())?;
        Ok(value)
    }
}

impl LogicalExpr {
    pub fn eval<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<Rc<Literal>> {
        let lhs = self.left.eval(env, output)?;
        if self.operator.token_type == TokenType::OR {
            if lhs.is_truthy() {
                return Ok(lhs);
            }
        } else {
            if !lhs.is_truthy() {
                return Ok(lhs);
            }
        }
        Ok(self.right.eval(env, output)?)
    }
}

impl CallExpr {
    pub fn eval<T: Write>(&self, env: &mut EnvironmentTree, output: &mut T) -> Result<Rc<Literal>> {
        let callee = self.callee.eval(env, output)?;
        let mut args = vec![];
        for arg in &self.args {
            args.push(arg.eval(env, output)?);
        }

        match callee.borrow() {
            Literal::FunctionLiteral(fun) => {
                if args.len() != fun.arity() {
                    return Err(RuntimeError::new(
                        &self.paren,
                        &format!(
                            "Expected {} arguments but got {}. ",
                            fun.arity(),
                            args.len()
                        ),
                    ));
                }
                Ok(fun.call(args, env, output)?)
            }
            Literal::NativeFunctionLiteral(fun) => {
                if args.len() != fun.arity() {
                    return Err(RuntimeError::new(
                        &self.paren,
                        &format!(
                            "Expected {} arguments but got {}. ",
                            fun.arity(),
                            args.len()
                        ),
                    ));
                }
                Ok(fun.call(args)?)
            }
            _ => Err(RuntimeError::new(
                &self.paren,
                "Can only call functions and classes.",
            )),
        }
    }
}
