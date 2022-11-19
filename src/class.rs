use crate::expr_interpret::Result;
use crate::function::LoxFunction;
use crate::instance::LoxInstance;
use crate::{environment::EnvironmentTree, literal::Literal};
use std::cell::RefCell;
use std::collections::HashMap;
use std::{fmt::Display, io::Write, rc::Rc};

// TODO: clone or not clone?
#[derive(PartialEq)]
pub struct LoxClass {
    pub name: String,
    pub methods: HashMap<String, LoxFunction>,
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl LoxClass {
    pub fn new(name: String, methods: HashMap<String, LoxFunction>) -> Self {
        LoxClass { name, methods }
    }

    pub fn call<T: Write>(
        self: &Rc<Self>,
        _args: Vec<Rc<Literal>>,
        _env: &mut EnvironmentTree,
        _output: &mut T,
    ) -> Result<Rc<Literal>> {
        // should be fine to clone lox class
        let instance = LoxInstance::new(self.clone());
        Ok(Rc::new(Literal::InstanceLiteral(RefCell::new(instance))))
    }

    pub fn arity(&self) -> usize {
        0
    }
}
