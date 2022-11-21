use crate::environment::EnvironmentTree;
use crate::expr_interpret::{Result, RuntimeError};
use crate::token::Token;
use crate::{class::LoxClass, literal::Literal};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::{fmt::Display, rc::Rc};

// Note that currently nothing stops you from putting an instance into
// itself, even though it will create a reference cycle of Rc<Literal>.
// We *might* be able to prevent it by doing some static analysis during
// variable resolution (but how do we handle a loop with
// more than two Rc<Literal>?)

#[derive(PartialEq)]
pub struct LoxInstance {
    class: Rc<LoxClass>,
    fields: HashMap<String, Rc<Literal>>,
    bound_methods: HashMap<String, Rc<Literal>>,
    // hash of (name of superclass, name of method) -> Rc<Literal::FunctionLiteral...>
    bound_super_methods: HashMap<u64, Rc<Literal>>,
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
            bound_super_methods: HashMap::new(),
        }
    }

    pub fn get(
        &mut self,
        name: &Token,
        env: &mut EnvironmentTree,
        instance: &Rc<Literal>,
    ) -> Result<Rc<Literal>> {
        if let Some(f) = self.fields.get(&name.lexeme) {
            Ok(f.clone())
        } else if let Some(bm) = self.bound_methods.get(&name.lexeme) {
            // method is already bound so reuse it
            Ok(bm.clone())
        } else if let Some(m) = self.class.find_method(&name.lexeme) {
            let bound_method = Rc::new(Literal::FunctionLiteral(m.bind(env, instance.clone())));
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

    pub fn get_super_method(
        &mut self,
        superclass: &LoxClass,
        method: &Token,
        env: &mut EnvironmentTree,
        instance: &Rc<Literal>,
    ) -> Result<Rc<Literal>> {
        let method_name = &method.lexeme;
        let class_name = &superclass.name;

        let mut hasher = DefaultHasher::new();
        (class_name, method_name).hash(&mut hasher);
        let hash = hasher.finish();

        if let Some(bound_method) = self.bound_super_methods.get(&hash) {
            return Ok(bound_method.clone());
        } else if let Some(method) = superclass.find_method(method_name) {
            let bound_method =
                Rc::new(Literal::FunctionLiteral(method.bind(env, instance.clone())));
            self.bound_super_methods.insert(hash, bound_method.clone());
            return Ok(bound_method);
        }

        Err(RuntimeError::new(
            method,
            &format!("Undefined property '{}'.", &method.lexeme),
        ))
    }
}
