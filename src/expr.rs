use super::literal::Literal;
use super::token::Token;
use super::token::TokenType;

#[derive(Debug)]
pub enum Expr {
    Literal {
        value: Literal,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
}

pub trait Visitor {
    type Result;
    fn visit_expr(&self, expr: &Expr) -> Self::Result;
}

// TODO: hmm, not exactly the same visitor pattern in the book, we use pattern matching here, might be a little slower?
pub struct AstPrinter;
impl Visitor for AstPrinter {
    type Result = String;
    fn visit_expr(&self, expr: &Expr) -> Self::Result {
        match *expr {
            Expr::Literal { ref value } => format!("{}", value),
            Expr::Binary {
                ref left,
                ref operator,
                ref right,
            } => {
                let lhs = self.visit_expr(left);
                let rhs = self.visit_expr(right);
                format!("({} {} {})", operator.lexeme, lhs, rhs)
            }
            Expr::Unary {
                ref operator,
                ref right,
            } => {
                let rhs = self.visit_expr(right);
                format!("({} {})", operator.lexeme, rhs)
            }
            Expr::Grouping { ref expr } => {
                let e = self.visit_expr(expr);
                format!("(group {})", e)
            }
        }
    }
}

pub fn test_ast_printer() {
    let minus_token = Token::new(TokenType::MINUS, String::from("-"), Literal::Empty, 0);
    let lhs = Box::new(Expr::Unary {
        operator: minus_token,
        right: Box::new(Expr::Literal {
            value: Literal::NumberLiteral(1.23),
        }),
    });
    let rhs = Box::new(Expr::Grouping {
        expr: Box::new(Expr::Literal {
            value: Literal::NumberLiteral(45.67),
        }),
    });
    let star_token = Token::new(TokenType::STAR, String::from("*"), Literal::Empty, 0);
    let e = Expr::Binary {
        left: lhs,
        operator: star_token,
        right: rhs,
    };

    let p = AstPrinter {};
    let result = p.visit_expr(&e);
    println!("{result}");
}
