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
use function::clock;
use function::lox;
use function::NativeFunction;
use literal::Literal;
use parser::Parser;
use scanner::Scanner;
use std::rc::Rc;
use std::{error::Error, io::Write};
use stmt_interpret::ExecError;

pub struct Interpreter {
    env: EnvironmentTree,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut i = Interpreter {
            env: EnvironmentTree::new(),
        };
        i.init();
        i
    }

    fn init(&mut self) {
        let clock_fun = NativeFunction::new("native-fn-clock", 0, clock);
        self.env.define(
            "clock".to_string(),
            Rc::new(Literal::NativeFunctionLiteral(clock_fun)),
        );

        let lox_fun = NativeFunction::new("native-fn-lox", 0, lox);
        self.env.define(
            "lox".to_string(),
            Rc::new(Literal::NativeFunctionLiteral(lox_fun)),
        );
    }

    // TODO: how can the users of interpreter distinguish between different errors
    // put different types of errors into an enum?
    pub fn run<T: Write, U: Write>(
        &mut self,
        source: String,
        mut output: T,
        mut error_output: U,
        debug: bool,
    ) -> Result<(), Box<dyn Error>> {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan()?;
        let parser = Parser::new(tokens);
        let statements = parser.parse(&mut error_output)?;
        for s in statements {
            if debug {
                write!(output, "AST-START\n{s}\nAST-END\n")?;
            }
            match s.execute(&mut self.env, &mut output) {
                Ok(_) => {}
                Err(ExecError::Return(_)) => {}
                Err(ExecError::RuntimeError(error)) => {
                    return Err(Box::new(error));
                }
            }
        }
        Ok(())
    }
}
