use super::expr_interpret::Result;
use super::expr_interpret::RuntimeError;
use super::literal::Literal;
use std::collections::HashMap;

type Environment = HashMap<String, Literal>;

pub struct Environments {
    chain: Vec<Environment>,
}

impl Environments {
    pub fn new() -> Environments {
        Environments {
            chain: vec![Environment::new()],
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        // Need to maintain some invariants
        let length = self.chain.len();
        if length == 0 {
            panic!("No environment in the chain")
        }
        self.chain[length - 1].insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<&Literal> {
        if let Some(literal) = self.find(name) {
            Ok(literal)
        } else {
            Err(RuntimeError {
                msg: format!("Undefined variable '{}'", name),
            })
        }
    }

    pub fn assign(&mut self, name: &str, value: Literal) -> Result<()> {
        if let Some(literal) = self.find_mut(name) {
            *literal = value;
            Ok(())
        } else {
            Err(RuntimeError {
                msg: format!("Undefined variable '{}'", name),
            })
        }
    }

    pub fn push(&mut self) {
        self.chain.push(Environment::new());
    }

    pub fn pop(&mut self) {
        self.chain.pop();
    }

    fn find(&self, name: &str) -> Option<&Literal> {
        for env in self.chain.iter().rev() {
            if let Some(l) = env.get(name) {
                return Some(l);
            }
        }

        return None;
    }

    fn find_mut(&mut self, name: &str) -> Option<&mut Literal> {
        for env in self.chain.iter_mut().rev() {
            if let Some(l) = env.get_mut(name) {
                return Some(l);
            }
        }
        return None;
    }
}
