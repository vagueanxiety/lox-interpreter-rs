mod environment;

mod class;
mod function;
mod instance;
mod literal;
mod native_function;
mod token;

mod parser;
mod resolver;
mod scanner;

mod expr;
mod expr_display;
mod expr_interpret;
mod expr_resolve;

mod statement;
mod stmt_display;
mod stmt_interpret;
mod stmt_resolve;

use environment::EnvironmentTree;
use literal::Literal;
use native_function::clock;
use native_function::lox;
use native_function::NativeFunction;
use parser::Parser;
use resolver::Resolver;
use scanner::Scanner;
use std::rc::Rc;
use std::{error::Error, io::Write};
use stmt_interpret::ExecError;

pub use native_function::LOX_ASCII;

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

    fn _run<T: Write, U: Write>(
        &mut self,
        source: String,
        output: &mut T,
        error_output: &mut U,
        debug: bool,
    ) -> Result<(), Box<dyn Error>> {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan()?;

        let parser = Parser::new(tokens);
        let mut statements = parser.parse(error_output)?;

        let resolver = Resolver::new();
        resolver.resolve(&mut statements)?;

        for s in statements {
            if debug {
                write!(output, "AST-START\n{s}\nAST-END\n")?;
            }
            match s.execute(&mut self.env, output) {
                Ok(_) => {}
                Err(ExecError::Return(_)) => {}
                Err(ExecError::RuntimeError(error)) => {
                    return Err(Box::new(error));
                }
            }
        }
        Ok(())
    }

    // TODO: how can the users of interpreter distinguish between different errors
    // put different types of errors into an enum?
    pub fn run<T: Write, U: Write>(
        &mut self,
        source: String,
        output: &mut T,
        error_output: &mut U,
        debug: bool,
    ) -> Result<(), Box<dyn Error>> {
        match self._run(source, output, error_output, debug) {
            Ok(x) => Ok(x),
            Err(err) => {
                write!(error_output, "{}\n", err)?;
                Err(err)
            }
        }
    }
}
