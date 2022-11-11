use crate::expr_interpret::RuntimeError;

use crate::environment::Environment;
use crate::environment::EnvironmentTree;
use crate::expr_interpret::Result;
use crate::literal::Literal;
use crate::statement::FunctionStmt;
use crate::stmt_interpret::ExecError;
use indextree::NodeId;
use std::fmt::Display;
use std::io::Write;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct LoxFunction {
    declaration: Rc<FunctionStmt>,
    closure: NodeId,
}

impl PartialEq for LoxFunction {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.declaration, &other.declaration) && self.closure == other.closure
    }
}

impl Display for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.declaration)
    }
}

impl LoxFunction {
    pub fn new(declaration: Rc<FunctionStmt>, closure: NodeId) -> Self {
        LoxFunction {
            declaration,
            closure,
        }
    }

    pub fn call<T: Write>(
        &self,
        args: Vec<Rc<Literal>>,
        env: &mut EnvironmentTree,
        output: &mut T,
    ) -> Result<Rc<Literal>> {
        let prev = env.checkout(self.closure);

        env.push(Environment::new());
        for (i, p) in self.declaration.params.iter().enumerate() {
            env.define(p.lexeme.clone(), args[i].clone());
        }

        let mut return_value = Rc::new(Literal::Empty);
        for s in self.declaration.body.iter() {
            match s.execute(env, output) {
                Ok(_) => {}
                Err(ExecError::Return(value)) => {
                    return_value = value;
                    break;
                }
                Err(ExecError::RuntimeError(error)) => {
                    return Err(error);
                }
            }
        }
        env.pop();
        env.checkout(prev);

        Ok(return_value)
    }

    pub fn arity(&self) -> usize {
        self.declaration.params.len()
    }
}

#[derive(Clone, PartialEq)]
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
        (self.fun)(args).or_else(|error| {
            Err(RuntimeError {
                msg: format!("[@{}] {}", self.name, error.msg),
            })
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
            msg: format!("Time went backwards!"),
        })
    }
}

static LOX_ASCII: &str = r"
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
