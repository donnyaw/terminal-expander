use clap::{Parser, Subcommand};
use cli_expander_config::TriggerRecord;
use cli_expander_config::{records_to_csv, records_to_json};
use cli_expander_config::{read_csv_file, write_csv_file, merge_records};
use cli_expander_ui::FormRenderer;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "cli-expander",
    version,
    about = "Terminal-native text expander with multi-field form support",
    args_conflicts_with_subcommands = true
)]
struct Cli {
    /// Trigger text to expand (e.g. :hello). Shorthand for `expand`.
    #[arg(index = 1)]
    input: Option<String>,

    /// Path to match files directory
    #[arg(
        short,
        long,
        global = true,
        default_value = "~/.config/cli-expander/matches"
    )]
    config_dir: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Expand a trigger string and output the result
    Expand {
        /// The input text to search for triggers
        input: Option<String>,

        /// Path to match files directory
        #[arg(short, long, default_value = "~/.config/cli-expander/matches")]
        config_dir: String,
    },

    /// List all available triggers
    List {
        /// Path to match files directory
        #[arg(short, long, default_value = "~/.config/cli-expander/matches")]
        config_dir: String,

        /// Output in CSV format (pipe to fzf)
        #[arg(long)]
        csv: bool,

        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },

    /// Generate triggers.csv from match files
    GenerateCsv {
        /// Path to match files directory
        #[arg(short, long, default_value = "~/.config/cli-expander/matches")]
        config_dir: String,

        /// Output path for CSV (default: ~/.config/cli-expander/triggers.csv)
        #[arg(short, long)]
        output: Option<String>,

        /// Overwrite existing CSV instead of merging
        #[arg(long)]
        force: bool,
    },

    /// Search triggers by keyword
    Search {
        /// Search query
        query: String,

        /// Path to match files directory
        #[arg(short, long, default_value = "~/.config/cli-expander/matches")]
        config_dir: String,

        /// Output in CSV format
        #[arg(long)]
        csv: bool,
    },

    /// Show details for a specific trigger
    Details {
        /// Trigger name (e.g. :findx)
        trigger: String,

        /// Path to match files directory
        #[arg(short, long, default_value = "~/.config/cli-expander/matches")]
        config_dir: String,
    },

    /// Show a form interactively
    Form {
        /// The form layout text (with [[field]] placeholders)
        layout: String,

        /// Title for the form window
        #[arg(short, long, default_value = "cli-expander")]
        title: String,
    },

    /// Show configuration info
    Config,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Determine the effective config_dir
    let config_dir = cli.config_dir.clone();

    // If a bare input was given, treat it as expand
    let cmd = if let Some(input) = cli.input {
        Some(Commands::Expand {
            input: Some(input),
            config_dir,
        })
    } else {
        cli.command
    };

    match cmd.unwrap_or_else(|| {
        // No subcommand and no input — show help
        let _ = Cli::parse_from(["cli-expander", "--help"]);
        std::process::exit(0);
    }) {
        Commands::Expand { input, config_dir } => {
            let input = input.unwrap_or_else(|| {
                eprintln!("error: no input provided for expansion");
                std::process::exit(1);
            });
            let dir = expand_path(&config_dir);
            let configs = cli_expander_config::Config::load_dir(&dir)?;
            let matcher = cli_expander_match::Matcher::from_files(configs);

            // Try exact end-of-buffer match first, then fall back to anywhere
            let matched = matcher
                .find_best(&input)
                .or_else(|| matcher.find_in(&input));
            match matched {
                Some(m) => {
                    if let Some(ref form_layout) = m.form {
                        // This match has a form — render it via Cursive UI
                        let fields = build_form_fields(m.form_fields.as_ref());

                        let renderer = cli_expander_ui::CursiveFormRenderer;
                        let form_result = renderer.show("cli-expander", &fields);
                        match form_result {
                            Ok(Some(result)) => {
                                let output = cli_expander_render::FormExtension::render_form(
                                    form_layout,
                                    &m.form_fields.as_ref().cloned().unwrap_or_default(),
                                    &result.values,
                                );
                                println!("{}", normalize_command_output(&output));
                            }
                            Ok(None) => {
                                eprintln!("[debug] Form cancelled by user");
                                std::process::exit(1);
                            }
                            Err(e) => {
                                let result = default_form_result(&fields);
                                let output = cli_expander_render::FormExtension::render_form(
                                    form_layout,
                                    &m.form_fields.as_ref().cloned().unwrap_or_default(),
                                    &result.values,
                                );
                                eprintln!(
                                    "warning: form unavailable ({}); using default form values",
                                    e
                                );
                                println!("{}", normalize_command_output(&output));
                            }
                        }
                    } else if let Some(ref replace) = m.replace {
                        // Static replacement — resolve variables
                        let engine = cli_expander_render::VariableEngine::default();
                        let mut vars = HashMap::new();

                        if let Some(ref var_defs) = m.vars {
                            // Check for form variables first
                            for var in var_defs {
                                if var.var_type == "form" {
                                    let fields = build_fields_from_form_var(var);

                                    if !fields.is_empty() {
                                        let renderer = cli_expander_ui::CursiveFormRenderer;
                                        let result = match renderer.show("cli-expander", &fields) {
                                            Ok(Some(result)) => result,
                                            Ok(None) => std::process::exit(1),
                                            Err(e) => {
                                                eprintln!(
                                                    "warning: form unavailable ({}); using default form values",
                                                    e
                                                );
                                                default_form_result(&fields)
                                            }
                                        };

                                        for (key, val) in &result.values {
                                            // Inject both with prefix (form.path) and without (path)
                                            vars.insert(
                                                format!("{}.{}", var.name, key),
                                                val.clone(),
                                            );
                                            vars.insert(key.clone(), val.clone());
                                        }
                                    }
                                    break;
                                }
                            }

                            // Resolve remaining variables (date, clipboard, shell)
                            if let Ok(resolved) = engine.resolve_all(var_defs) {
                                for (key, val) in resolved {
                                    vars.entry(key).or_insert(val);
                                }
                            }
                        }

                        let template = cli_expander_render::Template::new(replace);
                        let output = template.render(&vars);
                        println!("{}", normalize_command_output(&output));
                    }
                }
                None => {
                    eprintln!("No match found for trigger in: {}", input);
                    std::process::exit(1);
                }
            }
        }

        Commands::List {
            config_dir,
            csv,
            json,
        } => {
            let dir = expand_path(&config_dir);
            let configs = cli_expander_config::Config::load_dir(&dir)?;
            let records = TriggerRecord::from_configs(&configs);

            if csv {
                println!("{}", records_to_csv(&records));
            } else if json {
                println!("{}", records_to_json(&records));
            } else {
                println!("Available triggers:");
                println!("{:<25} {:<55} {:<10} {}", "Trigger", "Description", "Type", "Category");
                println!("{:-<25} {:-<55} {:-<10} {:-<15}", "", "", "", "");

                for r in &records {
                    let desc = if r.description.len() > 52 {
                        format!("{}...", &r.description[..49])
                    } else {
                        r.description.clone()
                    };
                    println!(
                        "{:<25} {:<55} {:<10} {}",
                        r.trigger, desc, r.trigger_type, r.category
                    );
                }
            }
        }

        Commands::GenerateCsv {
            config_dir,
            output,
            force,
        } => {
            let dir = expand_path(&config_dir);
            let configs = cli_expander_config::Config::load_dir(&dir)?;
            let auto_records = TriggerRecord::from_configs(&configs);

            let out_path = output.map(|p| PathBuf::from(p)).unwrap_or_else(|| {
                let home = std::env::var("HOME").unwrap_or_default();
                PathBuf::from(home).join(".config/cli-expander/triggers.csv")
            });

            if force {
                write_csv_file(&auto_records, &out_path)?;
                eprintln!("Wrote {} triggers to {}", auto_records.len(), out_path.display());
            } else {
                let existing = read_csv_file(&out_path).unwrap_or_default();
                let merged = merge_records(auto_records, existing);
                write_csv_file(&merged, &out_path)?;
                eprintln!(
                    "Merged {} triggers into {} (auto + manual)",
                    merged.len(),
                    out_path.display()
                );
            }
        }

        Commands::Search {
            query,
            config_dir,
            csv,
        } => {
            let dir = expand_path(&config_dir);
            let configs = cli_expander_config::Config::load_dir(&dir)?;
            let records = TriggerRecord::from_configs(&configs);
            let q = query.to_lowercase();

            let matched: Vec<&TriggerRecord> = records
                .iter()
                .filter(|r| {
                    r.trigger.to_lowercase().contains(&q)
                        || r.description.to_lowercase().contains(&q)
                        || r.category.to_lowercase().contains(&q)
                        || r.tags.to_lowercase().contains(&q)
                })
                .collect();

            if csv {
                let csv_records: Vec<TriggerRecord> =
                    matched.iter().map(|r| (*r).clone()).collect();
                println!("{}", records_to_csv(&csv_records));
            } else {
                if matched.is_empty() {
                    eprintln!("No triggers matching '{}'", query);
                } else {
                    println!("Triggers matching '{}':", query);
                    for r in &matched {
                        println!("  {:<25} {}", r.trigger, r.description);
                    }
                }
            }
        }

        Commands::Details {
            trigger,
            config_dir,
        } => {
            let dir = expand_path(&config_dir);
            let configs = cli_expander_config::Config::load_dir(&dir)?;
            let records = TriggerRecord::from_configs(&configs);

            if let Some(r) = records.iter().find(|r| r.trigger == trigger) {
                println!("Trigger:    {}", r.trigger);
                println!("Description: {}", r.description);
                println!("Category:   {}", r.category);
                println!("Type:       {}", r.trigger_type);
                println!("Tags:       {}", r.tags);
                println!("Source:     {}", r.source_file);
            } else {
                eprintln!("Trigger '{}' not found", trigger);
                std::process::exit(1);
            }
        }

        Commands::Form { layout, title } => {
            let fields = vec![cli_expander_ui::FormField {
                name: "input".to_string(),
                label: "Input:".to_string(),
                section: None,
                field_type: cli_expander_ui::FieldType::Text,
                default: None,
                placeholder: Some("Type here...".to_string()),
                values: None,
                multiline: false,
                depends_on: None,
                depends_map: None,
            }];
            let renderer = cli_expander_ui::CursiveFormRenderer;
            match renderer.show(&title, &fields)? {
                Some(result) => {
                    let output = cli_expander_render::FormExtension::render_form(
                        &layout,
                        &HashMap::new(),
                        &result.values,
                    );
                    println!("{}", output);
                }
                None => std::process::exit(1),
            }
        }

        Commands::Config => {
            println!("cli-expander configuration:");
            println!("  Config directory: ~/.config/cli-expander/matches/");
            println!("  Config file: ~/.config/cli-expander/config.yml");
            println!();
            println!("Shell plugins available at:");
            println!("  shell/cli-expander.zsh — Zsh plugin");
            println!("  shell/cli-expander.bash — Bash plugin");
            println!("  shell/cli-expander.fish — Fish plugin");
        }
    }

    Ok(())
}

fn expand_path(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/") {
        let home = std::env::var("HOME").unwrap_or_default();
        PathBuf::from(home).join(rest)
    } else {
        PathBuf::from(path)
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max.saturating_sub(3)])
    }
}

fn default_form_result(fields: &[cli_expander_ui::FormField]) -> cli_expander_ui::FormResult {
    let mut values = HashMap::new();

    for field in fields {
        let value = if field.field_type == cli_expander_ui::FieldType::Checkbox {
            field.default.clone().unwrap_or_else(|| "false".to_string())
        } else if let Some(default) = &field.default {
            default.clone()
        } else if let Some(dep_name) = &field.depends_on {
            values
                .get(dep_name)
                .and_then(|parent| field.depends_map.as_ref()?.get(parent))
                .and_then(|items| items.first())
                .cloned()
                .unwrap_or_default()
        } else {
            field
                .values
                .as_ref()
                .and_then(|items| items.first())
                .cloned()
                .or_else(|| field.placeholder.clone())
                .unwrap_or_default()
        };

        values.insert(field.name.clone(), value);
    }

    cli_expander_ui::FormResult { values }
}

/// Build form fields from a form variable definition (verbose syntax)
/// Parse a mapping of parent_value -> child_values from a serde_norway Value
fn parse_depends_map(
    val: &serde_norway::Value,
) -> Option<std::collections::HashMap<String, Vec<String>>> {
    let mapping = val.as_mapping()?;
    let mut result = std::collections::HashMap::new();
    for (key_val, vals_val) in mapping.iter() {
        let key = key_val.as_str()?.to_string();
        let seq = vals_val.as_sequence()?;
        let list: Vec<String> = seq
            .iter()
            .filter_map(|item| item.as_str().map(String::from))
            .collect();
        if !list.is_empty() {
            result.insert(key, list);
        }
    }
    Some(result)
}

fn build_fields_from_form_var(
    var: &cli_expander_config::VariableDef,
) -> Vec<cli_expander_ui::FormField> {
    use serde_norway::Value;
    use std::collections::HashMap;

    let mut result = Vec::new();
    let params = match var.params.as_ref() {
        Some(p) => p,
        None => return result,
    };

    let layout = match params.get("layout").and_then(|v| v.as_str()) {
        Some(l) => l,
        None => return result,
    };

    // Extract field names from layout [[field]] patterns with labels and section headings.
    let mut field_meta: Vec<(String, String, Option<String>)> = Vec::new();
    let mut current_section: Option<String> = None;
    for line in layout.lines() {
        let trimmed = line.trim();
        if !trimmed.contains("[[") {
            let section = trimmed
                .trim_matches(|c: char| {
                    matches!(
                        c,
                        '-' | '=' | '─' | '═' | '╔' | '╗' | '╚' | '╝' | '║' | '│' | ' '
                    )
                })
                .trim();
            if !section.is_empty() {
                current_section = Some(section.to_string());
            }
            continue;
        }

        let mut search_from = 0;
        while let Some(rel_start) = trimmed[search_from..].find("[[") {
            let start = search_from + rel_start;
            if let Some(rel_end) = trimmed[start..].find("]]") {
                let end = start + rel_end;
                let fname = trimmed[start + 2..end].trim().to_string();
                let flabel = if search_from == 0 {
                    trimmed[..start].trim().to_string()
                } else {
                    fname.clone()
                };
                if !fname.is_empty() {
                    let label = if flabel.ends_with(':') {
                        flabel
                    } else if flabel.is_empty() {
                        format!("{}:", fname)
                    } else {
                        format!("{}:", flabel)
                    };
                    field_meta.push((fname, label, current_section.clone()));
                }
                search_from = end + 2;
            } else {
                break;
            }
        }
    }

    // Parse field configs from params.fields mapping
    let mut field_configs: HashMap<String, HashMap<String, Value>> = HashMap::new();
    if let Some(fields_val) = params.get("fields") {
        if let Some(mapping) = fields_val.as_mapping() {
            for (key_val, cfg_val) in mapping.iter() {
                if let (Some(fname), Some(cfg_map)) = (key_val.as_str(), cfg_val.as_mapping()) {
                    let mut cfg = HashMap::new();
                    for (ck, cv) in cfg_map.iter() {
                        if let Some(ck_str) = ck.as_str() {
                            cfg.insert(ck_str.to_string(), cv.clone());
                        }
                    }
                    field_configs.insert(fname.to_string(), cfg);
                }
            }
        }
    }

    for (fname, label, section) in field_meta {
        let mut field_type = cli_expander_ui::FieldType::Text;
        let mut multiline = false;
        let mut default = None;
        let mut placeholder = None;
        let mut values = None;

        if let Some(cfg) = field_configs.get(&fname) {
            if let Some(t) = cfg.get("type").and_then(|v| v.as_str()) {
                match t {
                    "choice" => field_type = cli_expander_ui::FieldType::Choice,
                    "list" => field_type = cli_expander_ui::FieldType::List,
                    "checkbox" | "bool" | "boolean" => {
                        field_type = cli_expander_ui::FieldType::Checkbox
                    }
                    "password" | "secret" => field_type = cli_expander_ui::FieldType::Password,
                    _ => {}
                }
            }
            if let Some(v) = cfg.get("multiline").and_then(|v| v.as_bool()) {
                multiline = v;
            }
            if let Some(v) = cfg.get("default").and_then(|v| v.as_str()) {
                default = Some(v.to_string());
            }
            if let Some(v) = cfg.get("placeholder").and_then(|v| v.as_str()) {
                placeholder = Some(v.to_string());
            }
            if let Some(v) = cfg.get("values") {
                if let Some(seq) = v.as_sequence() {
                    let vlist: Vec<String> = seq
                        .iter()
                        .filter_map(|item| item.as_str().map(String::from))
                        .collect();
                    if !vlist.is_empty() {
                        values = Some(vlist);
                    }
                }
            }
        }

        // Check for depends_on and depends_map
        let mut depends_on = None;
        let mut depends_map = None;
        if let Some(cfg) = field_configs.get(&fname) {
            if let Some(v) = cfg.get("depends_on").and_then(|v| v.as_str()) {
                depends_on = Some(v.to_string());
                if let Some(v) = cfg.get("values") {
                    depends_map = parse_depends_map(v);
                    values = None; // values come from depends_map instead
                }
            }
        }

        result.push(cli_expander_ui::FormField {
            name: fname,
            label,
            section,
            field_type,
            default,
            placeholder,
            values,
            multiline,
            depends_on,
            depends_map,
        });
    }
    result
}

fn build_form_fields(
    fields: Option<&std::collections::HashMap<String, cli_expander_config::FieldConfig>>,
) -> Vec<cli_expander_ui::FormField> {
    let mut result = Vec::new();
    if let Some(fields) = fields {
        for (name, config) in fields {
            let field_type = match config.field_type.as_deref() {
                Some("choice") => cli_expander_ui::FieldType::Choice,
                Some("list") => cli_expander_ui::FieldType::List,
                Some("checkbox" | "bool" | "boolean") => cli_expander_ui::FieldType::Checkbox,
                Some("password" | "secret") => cli_expander_ui::FieldType::Password,
                _ => cli_expander_ui::FieldType::Text,
            };

            let values = config.values.as_ref().and_then(|v| {
                v.as_sequence().map(|seq| {
                    seq.iter()
                        .filter_map(|val| val.as_str().map(String::from))
                        .collect()
                })
            });

            let depends_on = config.depends_on.clone();
            let depends_map = if config.depends_on.is_some() {
                config.values.as_ref().and_then(parse_depends_map)
            } else {
                None
            };
            let resolved_values = if depends_map.is_some() { None } else { values };

            result.push(cli_expander_ui::FormField {
                name: name.clone(),
                label: format!("{}:", name),
                section: None,
                field_type,
                default: config.default.clone(),
                placeholder: config.placeholder.clone(),
                values: resolved_values,
                multiline: config.multiline.unwrap_or(false),
                depends_on,
                depends_map,
            });
        }
    }
    result
}

fn normalize_command_output(output: &str) -> String {
    let trimmed = output.trim();
    if !trimmed.starts_with("find ") || trimmed.contains('\n') {
        return output.to_string();
    }

    let mut normalized = String::new();
    let mut quote: Option<char> = None;
    let mut escaped = false;
    let mut previous_space = false;

    for ch in trimmed.chars() {
        if escaped {
            normalized.push(ch);
            escaped = false;
            previous_space = false;
            continue;
        }

        if ch == '\\' {
            normalized.push(ch);
            escaped = true;
            previous_space = false;
            continue;
        }

        if let Some(q) = quote {
            normalized.push(ch);
            if ch == q {
                quote = None;
            }
            previous_space = false;
            continue;
        }

        if ch == '\'' || ch == '"' {
            quote = Some(ch);
            normalized.push(ch);
            previous_space = false;
        } else if ch.is_whitespace() {
            if !previous_space {
                normalized.push(' ');
                previous_space = true;
            }
        } else {
            normalized.push(ch);
            previous_space = false;
        }
    }

    normalized
}

#[cfg(test)]
mod tests {
    use super::{build_fields_from_form_var, build_form_fields, normalize_command_output};

    #[test]
    fn test_build_form_fields_keeps_text_inputs_in_mixed_forms() {
        let fields: std::collections::HashMap<String, cli_expander_config::FieldConfig> =
            serde_norway::from_str(
                r#"
title:
  default: "My ticket"
  placeholder: "Enter title"
item:
  type: choice
  depends_on: category
  values:
    Fruits:
      - Apple
      - Banana
"#,
            )
            .unwrap();

        let built = build_form_fields(Some(&fields));
        assert_eq!(built.len(), 2);

        let title = built.iter().find(|field| field.name == "title").unwrap();
        assert_eq!(title.field_type, cli_expander_ui::FieldType::Text);
        assert_eq!(title.default.as_deref(), Some("My ticket"));

        let item = built.iter().find(|field| field.name == "item").unwrap();
        assert_eq!(item.field_type, cli_expander_ui::FieldType::Choice);
        assert_eq!(item.depends_on.as_deref(), Some("category"));
        assert!(item.depends_map.is_some());
    }

    #[test]
    fn test_build_fields_from_form_var_supports_text_and_cascade_fields() {
        let yaml = r#"
name: form
type: form
params:
  layout: |
    Metadata
      - Title: [[title]]

    Search Criteria
      - Category: [[category]]
      - Item: [[item]]
  fields:
    title:
      placeholder: "Enter a title"
    category:
      type: choice
      values:
        - Fruits
        - Animals
    item:
      type: choice
      depends_on: category
      values:
        Fruits:
          - Apple
          - Banana
        Animals:
          - Cat
          - Dog
"#;

        let var: cli_expander_config::VariableDef = serde_norway::from_str(yaml).unwrap();
        let fields = build_fields_from_form_var(&var);

        assert_eq!(fields.len(), 3);
        assert_eq!(fields[0].name, "title");
        assert_eq!(fields[0].section.as_deref(), Some("Metadata"));
        assert_eq!(fields[0].field_type, cli_expander_ui::FieldType::Text);
        assert_eq!(fields[1].field_type, cli_expander_ui::FieldType::Choice);
        assert_eq!(fields[1].section.as_deref(), Some("Search Criteria"));
        assert_eq!(fields[2].depends_on.as_deref(), Some("category"));
        assert!(fields[2].depends_map.is_some());
    }

    #[test]
    fn test_normalize_command_output_collapses_unquoted_find_spacing() {
        let output = "find . -name 'README*'  -print";
        assert_eq!(
            normalize_command_output(output),
            "find . -name 'README*' -print"
        );
    }

    #[test]
    fn test_normalize_command_output_preserves_quoted_spacing() {
        let output = "find . -name 'my  file.txt'  -print";
        assert_eq!(
            normalize_command_output(output),
            "find . -name 'my  file.txt' -print"
        );
    }
}
