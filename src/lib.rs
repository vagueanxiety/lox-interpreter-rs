mod ast_display;
mod ast_interpret;
mod expr;
mod literal;
mod parser;
mod scanner;
mod token;

use ast_display::test_ast_printer;
use parser::Parser;
use scanner::Scanner;
use std::fmt;
use std::io::{self, Write};

use ast_interpret::RuntimeError;
use parser::ParsingError;
use scanner::ScanningError;

#[derive(Debug)]
pub struct LoxError {
    msg: String,
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<ParsingError> for LoxError {
    fn from(err: ParsingError) -> Self {
        LoxError { msg: err.msg }
    }
}

impl From<RuntimeError> for LoxError {
    fn from(err: RuntimeError) -> Self {
        LoxError { msg: err.msg }
    }
}

impl From<ScanningError> for LoxError {
    fn from(err: ScanningError) -> Self {
        LoxError { msg: err.msg }
    }
}

fn run(source: String) -> Result<(), LoxError> {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    //println!("{:?}", tokens);
    let parser = Parser::new(tokens);
    let expr = parser.parse()?;
    //let ast = expr.print();
    //println!("{ast}");
    let result = expr.eval()?;
    println!("{result}");
    Ok(())
}

pub fn run_prompt() {
    println!(
        r"
   ,--,                                
,---.'|       ,----..                  
|   | :      /   /   \  ,--,     ,--,  
:   : |     /   .     : |'. \   / .`|  
|   ' :    .   /   ;.  \; \ `\ /' / ;  
;   ; '   .   ;   /  ` ;`. \  /  / .'  
'   | |__ ;   |  ; \ ; | \  \/  / ./   
|   | :.'||   :  | ; | '  \  \.'  /    
'   :    ;.   |  ' ' ' :   \  ;  ;     
|   |  ./ '   ;  \; /  |  / \  \  \    
;   : ;    \   \  ',  /  ;  /\  \  \   
|   ,/      ;   :    / ./__;  \  ;  \  
'---'        \   \ .'  |   : / \  \  ; 
              `---`    ;   |/   \  ' | 
                       `---'     `--`  
                                       
"
    );

    println!("Testing AST printer...");
    test_ast_printer();

    println!("Entering REPL...");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        match run(line) {
            Err(e) => {
                eprintln!("{e}")
            }
            _ => {}
        };
    }
}
