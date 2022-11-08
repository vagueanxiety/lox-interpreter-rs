mod environment;
mod expr;
mod expr_display;
mod expr_interpret;
mod function;
mod literal;
mod parser;
mod scanner;
mod statement;
mod stmt_display;
mod stmt_interpret;
mod token;

use environment::EnvironmentTree;
use parser::Parser;
use scanner::Scanner;
use std::{error::Error, io::Write};

pub struct Interpreter {
    env: EnvironmentTree,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            env: EnvironmentTree::new(),
        }
    }

    // TODO: how can the users of interpreter distinguish between different errors
    // put different types of errors into an enum?
    pub fn run<T: Write, U: Write>(
        &mut self,
        source: String,
        mut output: T,
        mut error_output: U,
        print_ast: bool,
    ) -> Result<(), Box<dyn Error>> {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan()?;
        let parser = Parser::new(tokens);
        let statements = parser.parse(&mut error_output)?;
        for s in statements {
            if print_ast {
                write!(output, "AST-START\n{s}\nAST-END\n")?;
            }
            s.execute(&mut self.env, &mut output)?;
        }
        Ok(())
    }
}
