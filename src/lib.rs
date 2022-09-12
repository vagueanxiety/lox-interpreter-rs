mod expr;
mod expr_display;
mod expr_interpret;
mod literal;
mod parser;
mod scanner;
mod statement;
mod stmt_display;
mod stmt_interpret;
mod token;

use parser::Parser;
use scanner::Scanner;
use std::error::Error;

pub fn run(source: String) -> Result<(), Box<dyn Error>> {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    //println!("{:?}", tokens);
    let parser = Parser::new(tokens);
    let statements = parser.parse()?;
    for s in statements {
        println!("{s}");
        s.execute()?;
    }
    Ok(())
}
