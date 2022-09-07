mod ast_display;
mod ast_interpret;
mod expr;
mod literal;
mod parser;
mod scanner;
mod token;

use parser::Parser;
use scanner::Scanner;
use std::error::Error;

pub fn run(source: String) -> Result<(), Box<dyn Error>> {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    //println!("{:?}", tokens);
    let parser = Parser::new(tokens);
    let expr = parser.parse()?;
    let ast = expr.print();
    println!("{ast}");
    let result = expr.eval()?;
    println!("{result}");
    Ok(())
}
