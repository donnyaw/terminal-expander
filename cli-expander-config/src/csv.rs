use crate::Config;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TriggerRecord {
    pub trigger: String,
    pub description: String,
    pub category: String,
    #[serde(rename = "type")]
    pub trigger_type: String,
    pub tags: String,
    pub source_file: String,
}

impl TriggerRecord {
    pub fn from_configs(
        configs: &[(std::path::PathBuf, Config)],
    ) -> Vec<Self> {
        let mut records = Vec::new();
        let mut seen_triggers: HashMap<String, u32> = HashMap::new();

        for (path, config) in configs {
            let source = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();

            for m in &config.matches {
                for trigger in m.triggers() {
                    // Handle duplicate triggers (disambiguation)
                    let key = trigger.to_string();
                    let suffix = seen_triggers.get(&key).copied().unwrap_or(0);
                    seen_triggers.insert(key.clone(), suffix + 1);

                    let display_trigger = if suffix > 0 {
                        format!("{} [{}]", trigger, suffix)
                    } else {
                        trigger.to_string()
                    };

                    let description = m.search_label.as_deref()
                        .or_else(|| {
                            m.replace.as_deref()
                                .or_else(|| m.form.as_deref())
                        })
                        .map(|s| {
                            let s = s.trim();
                            if s.len() > 80 {
                                format!("{}...", &s[..77])
                            } else {
                                s.to_string()
                            }
                        })
                        .unwrap_or_default();

                    let tags = extract_tags(trigger, &description);

                    records.push(TriggerRecord {
                        trigger: display_trigger,
                        description,
                        category: source.clone(),
                        trigger_type: if m.has_form() { "form".to_string() } else { "text".to_string() },
                        tags,
                        source_file: source.clone(),
                    });
                }
            }
        }

        records
    }

    pub fn to_csv_string(&self) -> String {
        let desc = self.description.replace('"', "\"\"");
        let tags = self.tags.replace('"', "\"\"");
        format!(
            "{},\"{}\",{},{},\"{}\",{}",
            self.trigger, desc, self.category, self.trigger_type, tags, self.source_file
        )
    }

    pub fn to_json_value(&self) -> serde_json::Value {
        serde_json::json!({
            "trigger": self.trigger,
            "description": self.description,
            "category": self.category,
            "type": self.trigger_type,
            "tags": self.tags,
            "source_file": self.source_file,
        })
    }
}

pub fn records_to_csv(records: &[TriggerRecord]) -> String {
    let mut out = String::from("trigger,description,category,type,tags,source_file\n");
    for r in records {
        out.push_str(&r.to_csv_string());
        out.push('\n');
    }
    out
}

pub fn records_to_json(records: &[TriggerRecord]) -> String {
    let values: Vec<serde_json::Value> = records.iter().map(|r| r.to_json_value()).collect();
    serde_json::to_string_pretty(&values).unwrap_or_default()
}

pub fn write_csv_file(records: &[TriggerRecord], path: &Path) -> anyhow::Result<()> {
    let content = records_to_csv(records);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, content)?;
    Ok(())
}

pub fn read_csv_file(path: &Path) -> anyhow::Result<Vec<TriggerRecord>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_path(path)?;
    let mut records = Vec::new();
    for result in reader.deserialize() {
        match result {
            Ok(record) => records.push(record),
            Err(e) => log::warn!("Skipping CSV row: {}", e),
        }
    }
    Ok(records)
}

pub fn merge_records(
    auto: Vec<TriggerRecord>,
    existing: Vec<TriggerRecord>,
) -> Vec<TriggerRecord> {
    let mut merged: std::collections::HashMap<String, TriggerRecord> = HashMap::new();

    // Add auto-generated records
    for r in auto {
        merged.insert(r.trigger.clone(), r);
    }

    // Preserve manual records (source_file starts with "manual:")
    for r in existing {
        if r.source_file.starts_with("manual:") {
            merged.insert(r.trigger.clone(), r);
        }
    }

    let mut result: Vec<TriggerRecord> = merged.into_values().collect();
    result.sort_by(|a, b| a.trigger.cmp(&b.trigger));
    result
}

fn extract_tags(trigger: &str, description: &str) -> String {
    let mut tags: Vec<String> = Vec::new();

    // Extract prefix from trigger (e.g., "find" from ":findx")
    if let Some(prefix) = trigger.strip_prefix(':') {
        let prefix = prefix.trim_end_matches(|c: char| c.is_ascii_punctuation());
        if !prefix.is_empty() {
            tags.push(prefix.to_string());
        }
    }

    // Extract keywords from description
    for word in description.split(|c: char| c.is_whitespace() || c == ',' || c == '.') {
        let word = word.trim().to_lowercase();
        if word.len() >= 3 && !tags.contains(&word) {
            tags.push(word);
        }
    }

    tags.truncate(8); // limit to 8 tags
    tags.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Config;

    #[test]
    fn test_from_configs_empty() {
        let records = TriggerRecord::from_configs(&[]);
        assert!(records.is_empty());
    }

    #[test]
    fn test_to_csv_string_quoting() {
        let r = TriggerRecord {
            trigger: ":test".to_string(),
            description: "A test with \"quotes\"".to_string(),
            category: "test".to_string(),
            trigger_type: "text".to_string(),
            tags: "test, demo".to_string(),
            source_file: "test.yml".to_string(),
        };
        let csv = r.to_csv_string();
        assert!(csv.contains("\"A test with \"\"quotes\"\"\""));
    }

    #[test]
    fn test_merge_manual_preserved() {
        let auto = vec![
            TriggerRecord {
                trigger: ":hello".to_string(),
                description: "Hello!".to_string(),
                category: "base".to_string(),
                trigger_type: "text".to_string(),
                tags: "hello".to_string(),
                source_file: "base.yml".to_string(),
            },
        ];
        let existing = vec![
            TriggerRecord {
                trigger: ":manual-trigger".to_string(),
                description: "Manual one".to_string(),
                category: "custom".to_string(),
                trigger_type: "text".to_string(),
                tags: "manual".to_string(),
                source_file: "manual:2026-06-15".to_string(),
            },
        ];
        let merged = merge_records(auto, existing);
        assert!(merged.iter().any(|r| r.trigger == ":manual-trigger"));
        assert!(merged.iter().any(|r| r.trigger == ":hello"));
    }

    #[test]
    fn test_extract_tags() {
        let tags = extract_tags(":findx", "Build find command with options");
        assert!(tags.contains("find"));
    }
}
