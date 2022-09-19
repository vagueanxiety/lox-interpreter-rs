mod environment;
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

use environment::Environments;
use expr_interpret::RuntimeError;
use parser::Parser;
use scanner::Scanner;
use statement::Stmt;
use std::error::Error;
use stmt_interpret::StmtInterpret;

pub struct Interpreter {
    env: Environments,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            env: Environments::new(),
        }
    }

    pub fn run(&mut self, source: String) -> Result<(), Box<dyn Error>> {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan()?;
        let parser = Parser::new(tokens);
        let statements = parser.parse()?;
        self.interpret(statements)?;
        Ok(())
    }

    fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
        for s in statements {
            println!("{s}");
            s.execute(&mut self.env)?;
        }
        Ok(())
    }
}
