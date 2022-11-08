use super::function::LoxFunction;
use super::function::NativeFunction;
use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Literal {
    Empty,
    StringLiteral(String),
    BoolLiteral(bool),
    NumberLiteral(f64),
    FunctionLiteral(LoxFunction),
    NativeFunctionLiteral(NativeFunction),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Literal::Empty => write!(f, "nil"),
            Literal::StringLiteral(ref s) => write!(f, "\"{}\"", s),
            Literal::NumberLiteral(n) => write!(f, "{}", n),
            Literal::BoolLiteral(b) => write!(f, "{}", b),
            Literal::FunctionLiteral(ref fun) => write!(f, "{}", fun),
            Literal::NativeFunctionLiteral(ref fun) => write!(f, "{}", fun),
        }
    }
}

pub struct OperatorError;
type Result<T> = std::result::Result<T, OperatorError>;

impl Literal {
    pub fn is_truthy(&self) -> bool {
        match *self {
            Literal::Empty => false,
            Literal::BoolLiteral(b) => b,
            _ => true,
        }
    }

    pub fn negative(&self) -> Result<Literal> {
        match *self {
            Literal::NumberLiteral(n) => Ok(Literal::NumberLiteral(-n)),
            _ => Err(OperatorError),
        }
    }

    pub fn minus(&self, other: &Literal) -> Result<Literal> {
        match *self {
            Literal::NumberLiteral(a) => match *other {
                Literal::NumberLiteral(b) => Ok(Literal::NumberLiteral(a - b)),
                _ => Err(OperatorError),
            },
            _ => Err(OperatorError),
        }
    }

    pub fn plus(&self, other: &Literal) -> Result<Literal> {
        match *self {
            Literal::NumberLiteral(a) => match *other {
                Literal::NumberLiteral(b) => Ok(Literal::NumberLiteral(a + b)),
                _ => Err(OperatorError),
            },
            Literal::StringLiteral(ref a) => match *other {
                Literal::StringLiteral(ref b) => Ok(Literal::StringLiteral(format!("{}{}", a, b))),
                _ => Err(OperatorError),
            },
            _ => Err(OperatorError),
        }
    }

    pub fn divide(&self, other: &Literal) -> Result<Literal> {
        match *self {
            Literal::NumberLiteral(a) => match *other {
                Literal::NumberLiteral(b) => Ok(Literal::NumberLiteral(b / a)),
                _ => Err(OperatorError),
            },
            _ => Err(OperatorError),
        }
    }

    pub fn multiply(&self, other: &Literal) -> Result<Literal> {
        match *self {
            Literal::NumberLiteral(a) => match *other {
                Literal::NumberLiteral(b) => Ok(Literal::NumberLiteral(a * b)),
                _ => Err(OperatorError),
            },
            _ => Err(OperatorError),
        }
    }

    pub fn equal(&self, other: &Literal) -> Literal {
        Literal::BoolLiteral(self == other)
    }

    pub fn not_equal(&self, other: &Literal) -> Literal {
        Literal::BoolLiteral(self != other)
    }

    // TODO: ugh, code duplication
    pub fn greater(&self, other: &Literal) -> Result<Literal> {
        match *self {
            Literal::NumberLiteral(a) => match *other {
                Literal::NumberLiteral(b) => Ok(Literal::BoolLiteral(a > b)),
                _ => Err(OperatorError),
            },
            _ => Err(OperatorError),
        }
    }

    pub fn greater_equal(&self, other: &Literal) -> Result<Literal> {
        match *self {
            Literal::NumberLiteral(a) => match *other {
                Literal::NumberLiteral(b) => Ok(Literal::BoolLiteral(a >= b)),
                _ => Err(OperatorError),
            },
            _ => Err(OperatorError),
        }
    }
    pub fn less(&self, other: &Literal) -> Result<Literal> {
        match *self {
            Literal::NumberLiteral(a) => match *other {
                Literal::NumberLiteral(b) => Ok(Literal::BoolLiteral(a < b)),
                _ => Err(OperatorError),
            },
            _ => Err(OperatorError),
        }
    }
    pub fn less_equal(&self, other: &Literal) -> Result<Literal> {
        match *self {
            Literal::NumberLiteral(a) => match *other {
                Literal::NumberLiteral(b) => Ok(Literal::BoolLiteral(a <= b)),
                _ => Err(OperatorError),
            },
            _ => Err(OperatorError),
        }
    }
}
