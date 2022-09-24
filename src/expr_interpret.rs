use super::environment::Environments;
use super::expr::*;
use super::literal::Literal;
use super::token::TokenType;
use std::error::Error;
use std::fmt;
use std::io;

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

impl From<io::Error> for RuntimeError {
    fn from(error: io::Error) -> Self {
        RuntimeError {
            msg: format!("RuntimeError caused by an IO error: {error}"),
        }
    }
}

pub type Result<T> = std::result::Result<T, RuntimeError>;

pub trait ExprInterpret {
    fn eval(&self, env: &mut Environments) -> Result<Literal>;
}

impl ExprInterpret for LiteralExpr {
    fn eval(&self, _env: &mut Environments) -> Result<Literal> {
        Ok(self.value.clone())
    }
}

impl ExprInterpret for GroupingExpr {
    fn eval(&self, env: &mut Environments) -> Result<Literal> {
        self.expr.eval(env)
    }
}

impl ExprInterpret for UnaryExpr {
    fn eval(&self, env: &mut Environments) -> Result<Literal> {
        let rhs = self.right.eval(env)?;
        match self.operator.token_type {
            TokenType::BANG => Ok(Literal::BoolLiteral(rhs.is_truthy())),
            TokenType::MINUS => match rhs.negative() {
                Ok(x) => Ok(x),
                _ => Err(RuntimeError {
                    msg: format!(
                        "{} cannot be applied to {}, it must be a number",
                        self.operator.lexeme, rhs
                    ),
                }),
            },
            ref tt => Err(RuntimeError {
                msg: format!("{:?} is unimplemented", tt),
            }),
        }
    }
}

impl ExprInterpret for BinaryExpr {
    fn eval(&self, env: &mut Environments) -> Result<Literal> {
        let lhs = self.left.eval(env)?;
        let rhs = self.right.eval(env)?;
        match self.operator.token_type {
            TokenType::MINUS => match lhs.minus(&rhs) {
                Ok(x) => Ok(x),
                _ => Err(RuntimeError {
                    msg: format!(
                        "{} cannot be applied to {} and {}, both must be number",
                        self.operator.lexeme, lhs, rhs
                    ),
                }),
            },
            TokenType::PLUS => match lhs.plus(&rhs) {
                Ok(x) => Ok(x),
                _ => Err(RuntimeError {
                    msg: format!(
                        "{} cannot be applied to {}, must be two numbers or two strings",
                        self.operator.lexeme, rhs
                    ),
                }),
            },
            TokenType::STAR => match lhs.multiply(&rhs) {
                Ok(x) => Ok(x),
                _ => Err(RuntimeError {
                    msg: format!(
                        "{} cannot be applied to {} and {}, both must be number",
                        self.operator.lexeme, lhs, rhs
                    ),
                }),
            },
            TokenType::SLASH => match rhs.divide(&lhs) {
                Ok(x) => Ok(x),
                _ => Err(RuntimeError {
                    msg: format!(
                        "{} cannot be applied to {} and {}, both must be number",
                        self.operator.lexeme, lhs, rhs
                    ),
                }),
            },
            TokenType::EQUAL_EQUAL => Ok(lhs.equal(&rhs)),
            TokenType::BANG_EQUAL => Ok(lhs.not_equal(&rhs)),
            TokenType::GREATER => match lhs.greater(&rhs) {
                Ok(x) => Ok(x),
                _ => Err(RuntimeError {
                    msg: format!(
                        "{} cannot be applied to {} and {}, both must be number",
                        self.operator.lexeme, lhs, rhs
                    ),
                }),
            },
            TokenType::GREATER_EQUAL => match lhs.greater_equal(&rhs) {
                Ok(x) => Ok(x),
                _ => Err(RuntimeError {
                    msg: format!(
                        "{} cannot be applied to {} and {}, both must be number",
                        self.operator.lexeme, lhs, rhs
                    ),
                }),
            },
            TokenType::LESS => match lhs.less(&rhs) {
                Ok(x) => Ok(x),
                _ => Err(RuntimeError {
                    msg: format!(
                        "{} cannot be applied to {} and {}, both must be number",
                        self.operator.lexeme, lhs, rhs
                    ),
                }),
            },
            TokenType::LESS_EQUAL => match lhs.less_equal(&rhs) {
                Ok(x) => Ok(x),
                _ => Err(RuntimeError {
                    msg: format!(
                        "{} cannot be applied to {} and {}, both must be number",
                        self.operator.lexeme, lhs, rhs
                    ),
                }),
            },
            ref tt => Err(RuntimeError {
                msg: format!("{:?} is unimplemented", tt),
            }),
        }
    }
}

impl ExprInterpret for VarExpr {
    fn eval(&self, env: &mut Environments) -> Result<Literal> {
        Ok(env.get(&self.name.lexeme)?.clone())
    }
}

impl ExprInterpret for AssignExpr {
    fn eval(&self, env: &mut Environments) -> Result<Literal> {
        let value = self.value.eval(env)?;
        env.assign(&self.name.lexeme, value.clone())?;
        Ok(value)
    }
}

impl ExprInterpret for LogicalExpr {
    fn eval(&self, env: &mut Environments) -> Result<Literal> {
        let lhs = self.left.eval(env)?;
        if self.operator.token_type == TokenType::OR {
            if lhs.is_truthy() {
                return Ok(lhs);
            }
        } else {
            if !lhs.is_truthy() {
                return Ok(lhs);
            }
        }
        Ok(self.right.eval(env)?)
    }
}
