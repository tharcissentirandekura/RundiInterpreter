// This file contains the implementation of the environment, including functions and types for managing variable scopes.

pub struct Environment{
    variables: std::collections::HashMap<String, i32>,
    parent: Option<Box<Environment>>,
}

impl Environment{
    pub fn new() -> Self {
        Environment {
            variables: std::collections::HashMap::new(),
            parent: None,
        }
    }

    pub fn new_with_parent(parent: Environment) -> Self {
        Environment {
            variables: std::collections::HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn set(&mut self, name: String, value: i32) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&i32> {
        if let Some(value) = self.variables.get(name) {
            Some(value)
        } else if let Some(ref parent) = self.parent {
            parent.get(name)
        } else {
            None
        }
    }

    pub fn remove(&mut self, name: &str) {
        self.variables.remove(name);
    }
}

