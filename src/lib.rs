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
use parser::Parser;
use scanner::Scanner;
use std::{error::Error, io::Write};
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

    pub fn run<T: Write, U: Write>(
        &mut self,
        source: String,
        mut output: T,
        mut error_output: U,
    ) -> Result<(), Box<dyn Error>> {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan()?;
        let parser = Parser::new(tokens);
        let statements = parser.parse(&mut error_output)?;
        for s in statements {
            s.execute(&mut self.env, &mut output)?;
        }
        Ok(())
    }
}
