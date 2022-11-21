use crate::expr_interpret::Result;
use crate::expr_interpret::RuntimeError;
use crate::literal::Literal;
use std::fmt::Display;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(PartialEq, Eq)]
pub struct NativeFunction {
    name: &'static str,
    arity: usize,
    fun: fn(Vec<Rc<Literal>>) -> Result<Rc<Literal>>,
}

impl Display for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl NativeFunction {
    pub fn new(
        name: &'static str,
        arity: usize,
        fun: fn(Vec<Rc<Literal>>) -> Result<Rc<Literal>>,
    ) -> Self {
        NativeFunction { name, arity, fun }
    }

    pub fn call(&self, args: Vec<Rc<Literal>>) -> Result<Rc<Literal>> {
        (self.fun)(args).map_err(|error| RuntimeError {
            msg: format!("[@{}] {}", self.name, error.msg),
        })
    }

    pub fn arity(&self) -> usize {
        self.arity
    }
}

// example native function
pub fn clock(_args: Vec<Rc<Literal>>) -> Result<Rc<Literal>> {
    let start = SystemTime::now();
    if let Ok(since_the_epoch) = start.duration_since(UNIX_EPOCH) {
        let secs = since_the_epoch.as_secs_f64();
        Ok(Rc::new(Literal::NumberLiteral(secs)))
    } else {
        Err(RuntimeError {
            msg: "Time went backwards!".to_string(),
        })
    }
}

pub static LOX_ASCII: &str = r"
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
";

pub fn lox(_args: Vec<Rc<Literal>>) -> Result<Rc<Literal>> {
    Ok(Rc::new(Literal::StringLiteral(LOX_ASCII.to_string())))
}
