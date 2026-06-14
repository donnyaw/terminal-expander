use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::path::PathBuf;
use texpand_ui::FormRenderer;

#[derive(Parser)]
#[command(
    name = "texpand",
    version,
    about = "Terminal-native text expander with Espanso-compatible forms",
    args_conflicts_with_subcommands = true
)]
struct Cli {
    /// Trigger text to expand (e.g. :hello). Shorthand for `expand`.
    #[arg(index = 1)]
    input: Option<String>,

    /// Path to match files directory
    #[arg(short, long, global = true, default_value = "~/.config/texpand/matches")]
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
        #[arg(short, long, default_value = "~/.config/texpand/matches")]
        config_dir: String,
    },

    /// List all available triggers
    List {
        /// Path to match files directory
        #[arg(short, long, default_value = "~/.config/texpand/matches")]
        config_dir: String,
    },

    /// Show a form interactively
    Form {
        /// The form layout text (with [[field]] placeholders)
        layout: String,

        /// Title for the form window
        #[arg(short, long, default_value = "texpand")]
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
        let _ = Cli::parse_from(["texpand", "--help"]);
        std::process::exit(0);
    }) {
        Commands::Expand { input, config_dir } => {
            let input = input.unwrap_or_else(|| {
                eprintln!("error: no input provided for expansion");
                std::process::exit(1);
            });
            let dir = expand_path(&config_dir);
            let configs = texpand_config::Config::load_dir(&dir)?;
            let matcher = texpand_match::Matcher::from_files(configs);

            match matcher.find_best(&input) {
                Some(m) => {
                    if let Some(ref form_layout) = m.form {
                        // This match has a form — render it via Cursive UI
                        let fields = build_form_fields(m.form_fields.as_ref());
                        let renderer = texpand_ui::CursiveFormRenderer;
                        match renderer.show("texpand", &fields)? {
                            Some(result) => {
                                let output = texpand_render::FormExtension::render_form(
                                    form_layout, &m.form_fields.unwrap_or_default(), &result.values
                                );
                                println!("{}", output);
                            }
                            None => std::process::exit(1),
                        }
                    } else if let Some(ref replace) = m.replace {
                        // Static replacement — resolve variables
                        let engine = texpand_render::VariableEngine::default();
                        let mut vars = HashMap::new();

                        if let Some(ref var_defs) = m.vars {
                            if let Ok(resolved) = engine.resolve_all(var_defs) {
                                vars = resolved;
                            }
                        }

                        let template = texpand_render::Template::new(replace);
                        let output = template.render(&vars);
                        println!("{}", output);
                    }
                }
                None => {
                    eprintln!("No match found for trigger in: {}", input);
                    std::process::exit(1);
                }
            }
        }

        Commands::List { config_dir } => {
            let dir = expand_path(&config_dir);
            let configs = texpand_config::Config::load_dir(&dir)?;
            let matcher = texpand_match::Matcher::from_files(configs);

            println!("Available triggers:");
            println!("{:<20} {:<40} Type", "Trigger", "Replace/Form");
            println!("{:-<20} {:-<40} {:-<10}", "-", "-", "-");

            for m in matcher.matches() {
                for trigger in m.triggers() {
                    let replace = m.replace.as_deref().unwrap_or("");
                    let type_str = if m.has_form() { "form" } else { "text" };
                    println!("{:<20} {:<40} {}", trigger, truncate(replace, 37), type_str);
                }
            }
        }

        Commands::Form { layout, title } => {
            let fields = vec![
                texpand_ui::FormField {
                    name: "input".to_string(),
                    label: "Input:".to_string(),
                    field_type: texpand_ui::FieldType::Text,
                    default: None,
                    placeholder: Some("Type here...".to_string()),
                    values: None,
                    multiline: false,
                }
            ];
            let renderer = texpand_ui::CursiveFormRenderer;
            match renderer.show(&title, &fields)? {
                Some(result) => {
                    let output = texpand_render::FormExtension::render_form(
                        &layout, &HashMap::new(), &result.values
                    );
                    println!("{}", output);
                }
                None => std::process::exit(1),
            }
        }

        Commands::Config => {
            println!("texpand configuration:");
            println!("  Config directory: ~/.config/texpand/matches/");
            println!("  Config file: ~/.config/texpand/config.yml");
            println!();
            println!("Shell plugins available at:");
            println!("  shell/texpand.zsh — Zsh plugin");
            println!("  shell/texpand.bash — Bash plugin");
            println!("  shell/texpand.fish — Fish plugin");
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

fn build_form_fields(
    fields: Option<&std::collections::HashMap<String, texpand_config::FieldConfig>>,
) -> Vec<texpand_ui::FormField> {
    let mut result = Vec::new();
    if let Some(fields) = fields {
        for (name, config) in fields {
            let field_type = match config.field_type.as_deref() {
                Some("choice") => texpand_ui::FieldType::Choice,
                Some("list") => texpand_ui::FieldType::List,
                _ => texpand_ui::FieldType::Text,
            };

            let values = config.values.as_ref().and_then(|v| {
                v.as_sequence().map(|seq| {
                    seq.iter().filter_map(|val| val.as_str().map(String::from)).collect()
                })
            });

            result.push(texpand_ui::FormField {
                name: name.clone(),
                label: format!("{}:", name),
                field_type,
                default: config.default.clone(),
                placeholder: config.placeholder.clone(),
                values,
                multiline: config.multiline.unwrap_or(false),
            });
        }
    }
    result
}
