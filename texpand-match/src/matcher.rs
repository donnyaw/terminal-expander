use texpand_config::MatchFile;

#[derive(Debug, Clone)]
pub struct MatchResult {
    pub matched: String,
    pub replace: Option<String>,
    pub form: Option<String>,
    pub form_fields: Option<std::collections::HashMap<String, texpand_config::FieldConfig>>,
    pub vars: Option<Vec<texpand_config::VariableDef>>,
}

pub struct Matcher {
    matches: Vec<MatchFile>,
}

impl Matcher {
    pub fn new(matches: Vec<MatchFile>) -> Self {
        Self { matches }
    }

    pub fn find(&self, input: &str) -> Option<MatchResult> {
        for m in &self.matches {
            for trigger in m.triggers() {
                if input.ends_with(trigger) {
                    let matched = trigger.to_string();
                    // Check if followed by a word separator
                    let rest = &input[..input.len() - matched.len()];
                    if rest.is_empty() || rest.ends_with(char::is_whitespace) {
                        return Some(MatchResult {
                            matched,
                            replace: m.replace.clone(),
                            form: m.form.clone(),
                            form_fields: m.form_fields.clone(),
                            vars: m.vars.clone(),
                        });
                    }
                }
            }
        }
        None
    }
}
