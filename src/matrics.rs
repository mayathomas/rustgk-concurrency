
use std::collections::HashMap;

#[derive(Debug)]
pub struct Metrics{
    data: HashMap<String, i64>,
}

impl Metrics{
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn inc(&mut self, key: &str){
        let counter = self.data.entry(key.to_string()).or_insert(0);
        *counter += 1;
    }

    pub fn dec(&mut self, key: &str){
        let counter = self.data.entry(key.to_string()).or_insert(0);
        *counter -= 1;
    }
}