use std::collections::HashMap;

pub struct Headers {
    values: HashMap<String, String>,
}

impl Headers {
    pub fn default() -> Self {
        Headers::new()
    }

    pub fn new() -> Self {
        Headers {
            values: HashMap::new(),
        }
    }

    pub fn keys(&self) -> Vec<String> {
        self.values
            .keys()
            .map(|k| k.to_string())
            .collect::<Vec<_>>()
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        match self.values.get(&key.to_lowercase()) {
            Some(v) => Some(&v),
            None => None,
        }
    }

    pub fn put(&mut self, key: &str, value: &str) {
        self.values
            .insert(key.to_lowercase().to_string(), value.to_string());
    }
}
