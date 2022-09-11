use super::expr::BinaryExpr;
use super::expr::GroupingExpr;
use super::expr::LiteralExpr;
use super::expr::UnaryExpr;
use super::literal::Literal;
use super::token::Token;
use super::token::TokenType;
use std::fmt::Display;

impl Display for LiteralExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Display for BinaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.operator.lexeme, self.left, self.right)
    }
}

impl Display for UnaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.operator.lexeme, self.right)
    }
}

impl Display for GroupingExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(group {})", self.expr)
    }
}

#[allow(dead_code)]
pub fn test_ast_printer() {
    let minus_token = Token::new(TokenType::MINUS, String::from("-"), Literal::Empty, 0);
    let lhs = Box::new(UnaryExpr {
        operator: minus_token,
        right: Box::new(LiteralExpr {
            value: Literal::NumberLiteral(1.23),
        }),
    });
    let rhs = Box::new(GroupingExpr {
        expr: Box::new(LiteralExpr {
            value: Literal::NumberLiteral(45.67),
        }),
    });
    let star_token = Token::new(TokenType::STAR, String::from("*"), Literal::Empty, 0);
    let e = BinaryExpr {
        left: lhs,
        operator: star_token,
        right: rhs,
    };

    println!("{e}");
}
