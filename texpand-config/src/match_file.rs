use crate::config::MatchFile;

/// Represents a parsed match file from Espanso-compatible YAML.
impl MatchFile {
    /// Get all triggers for this match.
    pub fn triggers(&self) -> Vec<&str> {
        let mut t = Vec::new();
        if let Some(ref trigger) = self.trigger {
            t.push(trigger.as_str());
        }
        if let Some(ref triggers) = self.triggers {
            for trigger in triggers {
                t.push(trigger.as_str());
            }
        }
        t
    }

    /// Whether this match uses a form instead of a static replacement.
    pub fn has_form(&self) -> bool {
        self.form.is_some()
    }
}
