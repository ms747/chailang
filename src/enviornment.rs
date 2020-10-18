use std::collections::HashMap;

use crate::object::ChaiObject;

#[derive(Debug, Clone, PartialEq)]
pub struct Enviornment {
    state: HashMap<String, ChaiObject>,
    outer: Option<Box<Enviornment>>,
}

impl Enviornment {
    pub fn new() -> Self {
        Enviornment {
            state: HashMap::new(),
            outer: None,
        }
    }

    pub fn enclosed(outer: Enviornment) -> Self {
        Enviornment {
            state: HashMap::new(),
            outer: Some(outer.into()),
        }
    }

    pub fn set(&mut self, name: &str, object: ChaiObject) {
        self.state.insert(name.into(), object);
    }

    pub fn get(&mut self, name: &str) -> Option<ChaiObject> {
        let value = self.state.get(name.into()).cloned();
        if !value.is_some() && !self.outer.is_none() {
            if let Some(outer) = self.outer.clone() {
                return outer.clone().get(name);
            }
        }
        value
    }
}
