use std::collections::HashMap;

use crate::parser::List;


pub struct Functions(HashMap<String, List>);

impl Functions {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn define(&mut self, name: String, body: List) {
        self.0.insert(name, body);
    }

    pub fn get(&self, name: &str) -> Option<&List> {
        self.0.get(name)
    }
}
