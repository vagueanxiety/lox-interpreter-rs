mod error;
mod expr;
mod literal;
mod parser;
mod scanner;
mod token;

use expr::test_ast_printer;
use expr::AstPrinter;
use expr::Visitor;

use parser::Parser;
use scanner::Scanner;
use std::io::{self, Write};

fn run(source: String) {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    //println!("{:?}", tokens);
    let parser = Parser::new(tokens);
    let expr = parser.parse();
    //println!("{:?}", expr);
    let p = AstPrinter {};
    let result = p.visit_expr(&expr);
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
