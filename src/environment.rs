use std::collections::HashMap;
use std::any::Any;

// TODO:

#[derive(Debug)]
pub struct Environment {
    store: Vec<Box<dyn Any>>,
    keys: HashMap<String, usize>
}

impl Default for Environment {
    fn default() -> Self {
        Environment {
            store: Vec::new(),
            keys: HashMap::new(),
        }
    }
}