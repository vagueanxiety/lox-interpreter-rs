use super::expr::BinaryExpr;
use super::expr::GroupingExpr;
use super::expr::LiteralExpr;
use super::expr::UnaryExpr;
use super::literal::Literal;
use super::token::Token;
use super::token::TokenType;

pub trait AstDisplay {
    fn print(&self) -> String;
}

impl AstDisplay for LiteralExpr {
    fn print(&self) -> String {
        format!("{}", self.value)
    }
}

impl AstDisplay for BinaryExpr {
    fn print(&self) -> String {
        let lhs = self.left.print();
        let rhs = self.right.print();
        format!("({} {} {})", self.operator.lexeme, lhs, rhs)
    }
}

impl AstDisplay for UnaryExpr {
    fn print(&self) -> String {
        let rhs = self.right.print();
        format!("({} {})", self.operator.lexeme, rhs)
    }
}

impl AstDisplay for GroupingExpr {
    fn print(&self) -> String {
        let e = self.expr.print();
        format!("(group {})", e)
    }
}

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

    let result = e.print();
    println!("{result}");
}
