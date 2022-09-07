use super::expr::BinaryExpr;
use super::expr::GroupingExpr;
use super::expr::LiteralExpr;
use super::expr::UnaryExpr;
use super::literal::Literal;
use super::token::TokenType;
use std::error::Error;
use std::fmt;

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

type Result<T> = std::result::Result<T, RuntimeError>;

pub trait AstInterpret {
    fn eval(&self) -> Result<Literal>;
}

impl AstInterpret for LiteralExpr {
    fn eval(&self) -> Result<Literal> {
        Ok(self.value.clone())
    }
}

impl AstInterpret for GroupingExpr {
    fn eval(&self) -> Result<Literal> {
        self.expr.eval()
    }
}

impl AstInterpret for UnaryExpr {
    fn eval(&self) -> Result<Literal> {
        let rhs = self.right.eval()?;
        match self.operator.token_type {
            TokenType::BANG => Ok(rhs.is_truthy().revert()),
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

impl AstInterpret for BinaryExpr {
    fn eval(&self) -> Result<Literal> {
        let lhs = self.left.eval()?;
        let rhs = self.right.eval()?;
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
