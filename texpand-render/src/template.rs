use std::collections::HashMap;

pub struct Template {
    source: String,
}

impl Template {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
        }
    }

    pub fn render(&self, vars: &HashMap<String, String>) -> String {
        let mut result = self.source.clone();
        for (key, value) in vars {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }
        result
    }

    pub fn cursor_position(&self) -> Option<usize> {
        self.source.find("$|$")
    }
}
