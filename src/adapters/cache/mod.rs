use std::collections::HashMap;

pub struct Cache<T> {
    data: HashMap<String, T>,
}

impl<'a, T> Cache<T> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn get(&'a self, key: &str) -> Option<&T> {
        match self.data.get(key) {
            Some(data) => Some(data),
            None => None,
        }
    }

    pub fn insert(&mut self, key: &'a str, value: T) {
        self.data.insert(String::from(key), value);
    }

    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
}
