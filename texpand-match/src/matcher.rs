use std::collections::HashMap;
use texpand_config::{FieldConfig, MatchFile, VariableDef};

#[derive(Debug, Clone)]
pub struct Match {
    pub matched_text: String,
    pub replace: Option<String>,
    pub form: Option<String>,
    pub form_fields: Option<HashMap<String, FieldConfig>>,
    pub vars: Option<Vec<VariableDef>>,
    pub source: MatchFile,
}

#[derive(Debug, Clone)]
pub struct MatchResult {
    pub matches: Vec<Match>,
}

pub struct Matcher {
    matches: Vec<MatchFile>,
}

impl Matcher {
    pub fn new(matches: Vec<MatchFile>) -> Self {
        Self { matches }
    }

    pub fn from_files(files: Vec<(std::path::PathBuf, texpand_config::Config)>) -> Self {
        let all_matches: Vec<MatchFile> = files
            .into_iter()
            .flat_map(|(_, config)| config.matches)
            .collect();
        Self::new(all_matches)
    }

    /// Find all matches for a given input buffer.
    pub fn find_all(&self, input: &str) -> MatchResult {
        let mut results = Vec::new();

        for m in &self.matches {
            for trigger in m.triggers() {
                if let Some(matched) = self.check_trigger(input, trigger) {
                    results.push(Match {
                        matched_text: matched,
                        replace: m.replace.clone(),
                        form: m.form.clone(),
                        form_fields: m.form_fields.clone(),
                        vars: m.vars.clone(),
                        source: m.clone(),
                    });
                }
            }
        }

        // Sort by longest match first (more specific triggers win)
        results.sort_by(|a, b| a.matched_text.len().cmp(&b.matched_text.len()).reverse());

        MatchResult { matches: results }
    }

    /// Get a reference to all loaded matches
    pub fn matches(&self) -> &[MatchFile] {
        &self.matches
    }

    /// Find the best (longest) match for a given input buffer.
    pub fn find_best(&self, input: &str) -> Option<Match> {
        let result = self.find_all(input);
        result.matches.into_iter().next()
    }

    fn check_trigger(&self, input: &str, trigger: &str) -> Option<String> {
        if input.ends_with(trigger) {
            let prefix_end = input.len() - trigger.len();
            let prefix = &input[..prefix_end];

            // Check word boundary: trigger must be at start of input or preceded by whitespace
            if prefix.is_empty() || prefix.ends_with(char::is_whitespace) {
                return Some(trigger.to_string());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_match(trigger: &str, replace: &str) -> MatchFile {
        MatchFile {
            trigger: Some(trigger.to_string()),
            triggers: None,
            replace: Some(replace.to_string()),
            form: None,
            form_fields: None,
            vars: None,
            force_mode: None,
            propagate_case: None,
            word: None,
            image_path: None,
            markdown: None,
            html: None,
            search_label: None,
        }
    }

    #[test]
    fn test_simple_match() {
        let matcher = Matcher::new(vec![make_match(":greet", "Hello there!")]);
        let result = matcher.find_best("say :greet");
        assert!(result.is_some());
        assert_eq!(result.unwrap().matched_text, ":greet");
    }

    #[test]
    fn test_no_match_within_word() {
        let matcher = Matcher::new(vec![make_match(":greet", "Hello!")]);
        let result = matcher.find_best("my:greet");
        assert!(result.is_none());
    }

    #[test]
    fn test_match_at_start() {
        let matcher = Matcher::new(vec![make_match(":hi", "world")]);
        let result = matcher.find_best(":hi");
        assert!(result.is_some());
    }

    #[test]
    fn test_longest_match_wins() {
        let matcher = Matcher::new(vec![
            make_match(":g", "short"),
            make_match(":greet", "long"),
        ]);
        let result = matcher.find_best("type :greet");
        assert!(result.is_some());
        assert_eq!(result.unwrap().matched_text, ":greet");
    }

    #[test]
    fn test_multiple_triggers() {
        let mut m = make_match(":hello", "world");
        m.triggers = Some(vec!["hi".to_string(), "hey".to_string()]);
        let matcher = Matcher::new(vec![m]);

        assert!(matcher.find_best("say hi").is_some());
        assert!(matcher.find_best("say hey").is_some());
    }

    #[test]
    fn test_no_match_empty_input() {
        let matcher = Matcher::new(vec![make_match(":greet", "Hello!")]);
        assert!(matcher.find_best("").is_none());
    }

    #[test]
    fn test_match_disambiguation() {
        let matcher = Matcher::new(vec![
            make_match(":quote", "Quote one"),
            make_match(":quote", "Quote two"),
            make_match(":quote", "Quote three"),
        ]);
        let result = matcher.find_all(":quote");
        assert_eq!(result.matches.len(), 3);
    }

    #[test]
    fn test_trigger_with_prefix() {
        let matcher = Matcher::new(vec![make_match("git!rel", "release")]);
        assert!(matcher.find_best("run git!rel").is_some());
        assert!(matcher.find_best("nogit!rel").is_none());
    }

    #[test]
    fn test_no_match_inside_word() {
        let matcher = Matcher::new(vec![make_match("ls", "list")]);
        let result = matcher.find_best("callsls");
        assert!(result.is_none());
    }

    #[test]
    fn test_tab_complete_context() {
        let matcher = Matcher::new(vec![make_match(":now", "date")]);
        assert!(matcher.find_best("echo :now").is_some());
    }
}
