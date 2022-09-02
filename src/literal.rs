use std::fmt;
#[derive(Clone, Debug)]
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
