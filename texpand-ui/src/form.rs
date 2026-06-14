use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FormField {
    pub name: String,
    pub label: String,
    pub field_type: FieldType,
    pub default: Option<String>,
    pub placeholder: Option<String>,
    pub values: Option<Vec<String>>,
    pub multiline: bool,
}

#[derive(Debug, Clone)]
pub enum FieldType {
    Text,
    Choice,
    List,
}

#[derive(Debug, Clone)]
pub struct FormResult {
    pub values: HashMap<String, String>,
}

pub trait FormRenderer: Send {
    fn show(&self, title: &str, fields: &[FormField]) -> anyhow::Result<Option<FormResult>>;
}
