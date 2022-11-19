use crate::environment::EnvironmentTree;
use crate::expr_interpret::{Result, RuntimeError};
use crate::token::Token;
use crate::{class::LoxClass, literal::Literal};
use std::collections::HashMap;
use std::{fmt::Display, rc::Rc};

// Note that currently nothing stops you from putting an instance into
// itself, even though it will create a reference cycle of Rc<Literal>.
// As a result, they never get deallocated. We *might* be able to prevent it
// by doing some static analysis during variable resolution (but how do we handle
// a loop with more than two Rc<Literal>?)
#[derive(PartialEq)]
pub struct LoxInstance {
    class: Rc<LoxClass>,
    fields: HashMap<String, Rc<Literal>>,
    bound_methods: HashMap<String, Rc<Literal>>,
}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "instance of class {}", self.class)
    }
}

impl LoxInstance {
    pub fn new(class: Rc<LoxClass>) -> Self {
        LoxInstance {
            class,
            fields: HashMap::new(),
            bound_methods: HashMap::new(),
        }
    }

    pub fn get(
        &mut self,
        name: &Token,
        env: &mut EnvironmentTree,
        instance: Rc<Literal>,
    ) -> Result<Rc<Literal>> {
        if let Some(f) = self.fields.get(&name.lexeme) {
            Ok(f.clone())
        } else if let Some(bm) = self.bound_methods.get(&name.lexeme) {
            // method is already bound and so reuse it
            Ok(bm.clone())
        } else if let Some(m) = self.class.methods.get(&name.lexeme) {
            let bound_method = Rc::new(Literal::FunctionLiteral(m.bind(env, instance)));
            self.bound_methods
                .insert(name.lexeme.clone(), bound_method.clone());
            Ok(bound_method)
        } else {
            Err(RuntimeError::new(
                name,
                &format!("Undefined property '{}'.", &name.lexeme),
            ))
        }
    }

    pub fn set(&mut self, name: String, value: Rc<Literal>) {
        self.fields.insert(name, value);
    }
}
