use super::token::Token;
use super::token::TokenType;

pub fn report(line: usize, location: &str, msg: &str) {
    println!("[line {}] Error{}: {}", line, location, msg);
}

pub fn report_token_err(t: &Token, msg: &str) {
    if t.token_type == TokenType::EOF {
        report(t.line, &String::from(" at end"), msg);
    } else {
        report(t.line, &format!(" at '{}'", t.lexeme), msg);
    }
}
