mod expr;
mod scanner;
mod token;
use expr::test_ast_printer;
use scanner::Scanner;
use std::io::{self, Write};

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    for t in scanner.scan_tokens() {
        println!("{}", t);
    }
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
