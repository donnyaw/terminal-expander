use std::collections::HashMap;
use texpand_config::FieldConfig;

pub type ExtensionResult = HashMap<String, String>;

pub trait Extension: Send {
    fn name(&self) -> &'static str;
    fn calculate(&self, params: &serde_norway::Value) -> anyhow::Result<ExtensionResult>;
}

pub struct FormExtension;

impl FormExtension {
    pub fn render_form(
        layout: &str,
        _fields: &HashMap<String, FieldConfig>,
        values: &HashMap<String, String>,
    ) -> String {
        let mut result = layout.to_string();
        for (key, value) in values {
            let placeholder = format!("[[{}]]", key);
            result = result.replace(&placeholder, value);
        }
        result
    }
}
