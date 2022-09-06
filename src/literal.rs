use std::fmt;
#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Empty,
    StringLiteral(String),
    BoolLiteral(bool),
    NumberLiteral(f64),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Literal::Empty => write!(f, "nil"),
            Literal::StringLiteral(ref s) => write!(f, "{}", s),
            Literal::NumberLiteral(n) => write!(f, "{}", n),
            Literal::BoolLiteral(b) => write!(f, "{}", b),
        }
    }
}

pub struct OperatorError;
type Result<T> = std::result::Result<T, OperatorError>;

impl Literal {
    pub fn is_truthy(&self) -> Literal {
        match *self {
            Literal::Empty => Literal::BoolLiteral(false),
            Literal::BoolLiteral(b) => Literal::BoolLiteral(b),
            _ => Literal::BoolLiteral(true),
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
