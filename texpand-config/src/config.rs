use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub matches: Vec<MatchFile>,

    #[serde(default)]
    pub params: Option<serde_norway::Value>,
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
    pub image_path: Option<String>,
    pub markdown: Option<String>,
    pub html: Option<String>,
    pub search_label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldConfig {
    #[serde(rename = "type")]
    pub field_type: Option<String>,
    pub multiline: Option<bool>,
    pub default: Option<String>,
    pub placeholder: Option<String>,
    pub values: Option<serde_norway::Value>,
    #[serde(rename = "trim_string_values")]
    pub trim_string_values: Option<bool>,
    #[serde(rename = "depends_on")]
    pub depends_on: Option<String>,
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
        Self::load_dir_recursive(dir, &mut results);
        results.sort_by_key(|(path, _)| path.clone());
        Ok(results)
    }

    fn load_dir_recursive(dir: &PathBuf, results: &mut Vec<(PathBuf, Self)>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            let mut entries: Vec<_> = entries.flatten().collect();
            entries.sort_by_key(|e| e.file_name());
            for entry in entries {
                let path = entry.path();
                if path.is_dir() {
                    Self::load_dir_recursive(&path, results);
                } else if path.extension().is_some_and(|e| e == "yml" || e == "yaml") {
                    match Self::load(&path) {
                        Ok(config) => results.push((path, config)),
                        Err(e) => log::warn!("Skipping {}: {}", path.display(), e),
                    }
                }
            }
        }
    }
}

impl std::str::FromStr for Config {
    type Err = super::ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_norway::from_str(s).map_err(|e| super::ConfigError::Parse(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::Config;

    #[test]
    fn test_simple_text_replacement() {
        let yaml = r#"
matches:
    - trigger: ":example"
      replace: "Hi there!"
"#;
        let config = yaml.parse::<Config>().unwrap();
        assert_eq!(config.matches.len(), 1);
        let m = &config.matches[0];
        assert_eq!(m.trigger.as_deref(), Some(":example"));
        assert_eq!(m.replace.as_deref(), Some("Hi there!"));
    }

    #[test]
    fn test_multiple_triggers() {
        let yaml = r#"
matches:
  - triggers: [":hello", ":hi"]
    replace: "world"
"#;
        let config = yaml.parse::<Config>().unwrap();
        let m = &config.matches[0];
        assert_eq!(m.triggers.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_multi_line_replace() {
        let yaml = r#"
matches:
  - trigger: ":mlt"
    replace: |
      This is line one.
      This is line two.
"#;
        let config = yaml.parse::<Config>().unwrap();
        let m = &config.matches[0];
        assert!(m.replace.as_deref().unwrap().contains("line one"));
    }

    #[test]
    fn test_form_with_fields() {
        let yaml = r#"
matches:
  - trigger: ":greet"
    form: |
      Hey [[name]],
      Happy Birthday!
    form_fields:
      name:
        placeholder: "Enter your name"
"#;
        let config = yaml.parse::<Config>().unwrap();
        let m = &config.matches[0];
        assert!(m.form.is_some());
        assert!(m.form_fields.is_some());
    }

    #[test]
    fn test_choice_field() {
        let yaml = r#"
matches:
  - trigger: ":choose"
    form: "Pick [[choice]]"
    form_fields:
      choice:
        type: choice
        values:
          - Option A
          - Option B
"#;
        let config = yaml.parse::<Config>().unwrap();
        let m = &config.matches[0];
        let fields = m.form_fields.as_ref().unwrap();
        assert_eq!(fields["choice"].field_type.as_deref(), Some("choice"));
    }

    #[test]
    fn test_variables() {
        let yaml = r#"
matches:
  - trigger: ":now"
    replace: "It's {{mytime}}"
    vars:
      - name: mytime
        type: date
        params:
          format: "%H:%M"
"#;
        let config = yaml.parse::<Config>().unwrap();
        let m = &config.matches[0];
        let vars = m.vars.as_ref().unwrap();
        assert_eq!(vars[0].name, "mytime");
        assert_eq!(vars[0].var_type, "date");
    }

    #[test]
    fn test_verbose_form() {
        let yaml = r#"
matches:
  - trigger: ":rev"
    replace: "{{reversed}}"
    vars:
    - name: form1
      type: form
      params:
        layout: "Reverse [[name]]"
    - name: reversed
      type: shell
      params:
        cmd: "echo '{{form1.name}}' | rev"
"#;
        let config = yaml.parse::<Config>().unwrap();
        let m = &config.matches[0];
        let vars = m.vars.as_ref().unwrap();
        assert_eq!(vars[1].var_type, "shell");
    }

    #[test]
    fn test_empty_config() {
        let yaml = "matches: []";
        let config = yaml.parse::<Config>().unwrap();
        assert!(config.matches.is_empty());
    }

    #[test]
    fn test_invalid_yaml() {
        let yaml = "matches: [broken";
        assert!(yaml.parse::<Config>().is_err());
    }

    #[test]
    fn test_load_dir_recursively_loads_yaml_files() {
        let root = std::env::temp_dir().join(format!(
            "texpand-config-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let nested = root.join("linux").join("ubuntu").join("apt");
        std::fs::create_dir_all(&nested).unwrap();

        std::fs::write(
            root.join("01-root.yml"),
            r#"matches:
  - trigger: ":test-root"
    replace: "root level works"
"#,
        )
        .unwrap();
        std::fs::write(
            root.join("linux").join("02-find.yaml"),
            r#"matches:
  - trigger: ":test-find"
    replace: "linux/find level works"
"#,
        )
        .unwrap();
        std::fs::write(
            root.join("linux").join("ubuntu").join("03-aliases.yml"),
            r#"matches:
  - trigger: ":test-aliases"
    replace: "linux/ubuntu/aliases level works"
"#,
        )
        .unwrap();
        std::fs::write(
            nested.join("04-apt.yml"),
            r#"matches:
  - trigger: ":test-apt"
    replace: "linux/ubuntu/apt level works"
"#,
        )
        .unwrap();
        std::fs::write(root.join("ignore.txt"), "matches: []").unwrap();

        let configs = Config::load_dir(&root).unwrap();
        let triggers: Vec<_> = configs
            .iter()
            .map(|(_, config)| config.matches[0].trigger.as_deref().unwrap())
            .collect();

        assert_eq!(
            triggers,
            vec![":test-root", ":test-find", ":test-aliases", ":test-apt"]
        );

        std::fs::remove_dir_all(root).unwrap();
    }
}
