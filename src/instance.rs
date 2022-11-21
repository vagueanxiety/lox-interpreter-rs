use crate::environment::EnvironmentTree;
use crate::expr_interpret::{Result, RuntimeError};
use crate::token::Token;
use crate::{class::LoxClass, literal::Literal};
use std::collections::HashMap;
use std::{fmt::Display, rc::Rc};

// Note that currently nothing stops you from putting an instance into
// itself, even though it will create a reference cycle of Rc<Literal>.
// We *might* be able to prevent it by doing some static analysis during variable resolution (but how do we handle a loop with
// more than two Rc<Literal>?)
#[derive(PartialEq)]
pub struct LoxInstance {
    class: Rc<LoxClass>,
    fields: HashMap<String, Rc<Literal>>,
    bound_methods: HashMap<String, Rc<Literal>>,
    // name of superclass -> name of method -> Rc<Literal::FunctionLiteral...>
    bound_super_methods: HashMap<String, HashMap<String, Rc<Literal>>>,
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
        instance: Rc<Literal>,
    ) -> Result<Rc<Literal>> {
        if let Some(f) = self.fields.get(&name.lexeme) {
            Ok(f.clone())
        } else if let Some(bm) = self.bound_methods.get(&name.lexeme) {
            // method is already bound so reuse it
            Ok(bm.clone())
        } else if let Some(m) = self.class.find_method(&name.lexeme) {
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

    pub fn get_super_method(
        &mut self,
        superclass: &LoxClass,
        method: &Token,
        env: &mut EnvironmentTree,
        instance: Rc<Literal>,
    ) -> Result<Rc<Literal>> {
        let method_name = &method.lexeme;
        let class_name = &superclass.name;

        if let Some(map) = self.bound_super_methods.get_mut(class_name) {
            if let Some(m) = map.get(method_name) {
                // already bound
                return Ok(m.clone());
            } else if let Some(m) = superclass.find_method(method_name) {
                // superclass entry exists but not the method
                let bound_method = Rc::new(Literal::FunctionLiteral(m.bind(env, instance)));
                map.insert(method_name.to_owned(), bound_method.clone());
                return Ok(bound_method);
            }
        } else if let Some(m) = superclass.find_method(method_name) {
            // both superclass and method don not exist
            let bound_method = Rc::new(Literal::FunctionLiteral(m.bind(env, instance)));
            let mut map = HashMap::new();
            map.insert(method_name.to_owned(), bound_method.clone());
            self.bound_super_methods.insert(class_name.clone(), map);
            return Ok(bound_method);
        }

        Err(RuntimeError::new(
            method,
            &format!("Undefined property '{}'.", &method.lexeme),
        ))
    }
}
