use super::expr_interpret::Result;
use super::expr_interpret::RuntimeError;
use super::literal::Literal;
use std::collections::HashMap;

pub struct Environment<'a> {
    enclosing: Option<&'a Environment<'a>>,
    map: HashMap<String, Literal>,
}

impl<'a> Environment<'a> {
    pub fn new() -> Environment<'a> {
        Environment {
            enclosing: None,
            map: HashMap::new(),
        }
    }

    // TODO: unused until scoping is done
    #[allow(dead_code)]
    pub fn from(enclosing: &'a Environment) -> Environment<'a> {
        Environment {
            enclosing: Some(enclosing),
            map: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.map.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<&Literal> {
        match self.map.get(name) {
            Some(l) => Ok(l),
            None => Err(RuntimeError {
                msg: format!("Undefined variable '{}'", name),
            }),
        }
    }

    pub fn assign(&mut self, name: String, value: Literal) -> Result<()> {
        if self.map.contains_key(&name) {
            self.map.insert(name, value);
            return Ok(());
        } else {
            return Err(RuntimeError {
                msg: format!("Undefined variable '{}'", name),
            });
        }
    }
}
