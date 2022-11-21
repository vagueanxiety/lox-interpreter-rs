use crate::expr_interpret::Result;
use crate::function::LoxFunction;
use crate::instance::LoxInstance;
use crate::{environment::EnvironmentTree, literal::Literal};
use std::cell::RefCell;
use std::collections::HashMap;
use std::{fmt::Display, io::Write, rc::Rc};

#[derive(PartialEq)]
pub struct LoxClass {
    pub name: String,
    pub methods: HashMap<String, LoxFunction>,
    pub superclass: Option<Rc<LoxClass>>,
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl LoxClass {
    pub fn new(
        name: String,
        methods: HashMap<String, LoxFunction>,
        superclass: Option<Rc<LoxClass>>,
    ) -> Self {
        LoxClass {
            name,
            methods,
            superclass,
        }
    }

    pub fn call<T: Write>(
        self: &Rc<Self>,
        args: Vec<Rc<Literal>>,
        env: &mut EnvironmentTree,
        output: &mut T,
    ) -> Result<Rc<Literal>> {
        let instance = Rc::new(Literal::InstanceLiteral(RefCell::new(LoxInstance::new(
            self.clone(),
        ))));

        if let Some(i) = self.methods.get("init") {
            let initializer = i.bind(env, instance.clone());
            initializer.call(args, env, output)?;
        }

        Ok(instance)
    }

    pub fn arity(&self) -> usize {
        self.methods.get("init").map_or(0, |m| m.arity())
    }

    pub fn find_method(&self, name: &str) -> Option<&LoxFunction> {
        self.methods.get(name).or_else(|| {
            self.superclass
                .as_ref()
                .map_or(None, |sc| sc.find_method(name))
        })
    }
}
