use std::collections::HashMap;

use crate::interpreter::interpreter::Value;

pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: String, value: Value) {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
        } else if let Some(enclosing) = &mut self.enclosing {
            enclosing.assign(name, value);
        } else {
            panic!("Undefined variable '{}'.", name);
        }
    }

    pub fn get(&self, name: String) -> Value {
        if let Some(value) = self.values.get(&name) {
            value.clone()
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.get(name)
        } else {
            panic!("Undefined variable '{}'.", name);
        }
    }
}