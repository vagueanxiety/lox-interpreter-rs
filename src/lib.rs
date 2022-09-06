mod ast_display;
mod ast_interpret;
mod error;
mod expr;
mod literal;
mod parser;
mod scanner;
mod token;

use crate::ast_display::test_ast_printer;
use parser::Parser;
use scanner::Scanner;
use std::io::{self, Write};

fn run(source: String) {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    //println!("{:?}", tokens);
    let parser = Parser::new(tokens);
    let expr = parser.parse();
    let ast = expr.print();
    println!("{ast}");
    let result = expr.eval().expect("Failed to evaluate");
    println!("{result}");
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
        run(line);
    }
}
