use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub matches: Vec<MatchFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchFile {
    pub trigger: Option<String>,
    pub triggers: Option<Vec<String>>,
    pub replace: Option<String>,
    pub form: Option<String>,
    pub form_fields: Option<HashMap<String, FieldConfig>>,
    pub vars: Option<Vec<VariableDef>>,
    pub force_mode: Option<String>,
    pub propagate_case: Option<bool>,
    pub word: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldConfig {
    #[serde(rename = "type")]
    pub field_type: Option<String>,
    pub multiline: Option<bool>,
    pub default: Option<String>,
    pub placeholder: Option<String>,
    pub values: Option<serde_norway::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableDef {
    pub name: String,
    #[serde(rename = "type")]
    pub var_type: String,
    pub params: Option<serde_norway::Value>,
}

impl Config {
    pub fn load(path: &PathBuf) -> Result<Self, super::ConfigError> {
        let content =
            std::fs::read_to_string(path).map_err(|e| super::ConfigError::Io(e.to_string()))?;
        let config: Config = serde_norway::from_str(&content)
            .map_err(|e| super::ConfigError::Parse(e.to_string()))?;
        Ok(config)
    }

    pub fn load_dir(dir: &PathBuf) -> Result<Vec<(PathBuf, Self)>, super::ConfigError> {
        let mut results = Vec::new();
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|e| e == "yml" || e == "yaml")
                {
                    match Self::load(&path) {
                        Ok(config) => results.push((path, config)),
                        Err(e) => log::warn!("Skipping {}: {}", path.display(), e),
                    }
                }
            }
        }
        Ok(results)
    }
}
