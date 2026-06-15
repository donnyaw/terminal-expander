use std::collections::HashMap;
use std::io::IsTerminal;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    Text,
    Choice,
    List,
}

#[derive(Debug, Clone)]
pub struct FormField {
    pub name: String,
    pub label: String,
    pub field_type: FieldType,
    pub default: Option<String>,
    pub placeholder: Option<String>,
    pub values: Option<Vec<String>>,
    pub multiline: bool,
    pub depends_on: Option<String>,
    /// For cascading selects: maps parent_value -> child_values
    pub depends_map: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, Clone)]
pub struct FormResult {
    pub values: HashMap<String, String>,
}

pub trait FormRenderer: Send {
    fn show(&self, title: &str, fields: &[FormField]) -> anyhow::Result<Option<FormResult>>;
}

pub struct CursiveFormRenderer;

impl FormRenderer for CursiveFormRenderer {
    fn show(&self, title: &str, fields: &[FormField]) -> anyhow::Result<Option<FormResult>> {
        render_cursive_form(title, fields)
    }
}

fn is_interactive_terminal() -> bool {
    std::io::stdout().is_terminal()
        && std::env::var("TERM").ok().is_some_and(|t| t != "dumb")
}

fn render_cursive_form(title: &str, fields: &[FormField]) -> anyhow::Result<Option<FormResult>> {
    use cursive::align::HAlign;
    use cursive::event::Key;
    use cursive::traits::{Nameable, Resizable};
    use cursive::views::{Button, Dialog, EditView, LinearLayout, ScrollView, SelectView, TextArea, TextView};
    use cursive::Cursive;

    if !is_interactive_terminal() {
        anyhow::bail!("Form requires an interactive terminal. Ensure TERM is set (e.g. TERM=xterm-256color) and you are running in a real terminal, not a pipe.");
    }

    let result = Arc::new(Mutex::new(None::<FormResult>));

    let mut siv = match std::panic::catch_unwind(|| {
        cursive::default()
    }) {
        Ok(s) => s,
        Err(e) => {
            let msg = if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else {
                "unknown error (terminal may not support TUI)".to_string()
            };
            anyhow::bail!("Failed to initialize terminal UI: {}. Try setting TERM=xterm-256color or running in a different terminal.", msg);
        }
    };

    // Collect cascading relationships: (parent_name, child_name, depends_map)
    let mut cascades: Vec<(String, String, HashMap<String, Vec<String>>)> = Vec::new();

    let mut layout = LinearLayout::vertical();

    for field in fields {
        // Visually distinguish dependent fields with > prefix and indent
        let is_dependent = field.depends_on.is_some();
        let label = if is_dependent {
            format!("  > {}", field.label.trim())
        } else {
            field.label.clone()
        };
        let name = field.name.clone();

        if field.field_type == FieldType::Choice || field.field_type == FieldType::List {
            let mut select = SelectView::new();
            if let Some(ref values) = field.values {
                for v in values {
                    select.add_item_str(v.clone());
                }
            }
            if let Some(ref default) = field.default {
                let idx = field.values.as_ref()
                    .and_then(|v| v.iter().position(|x| x == default))
                    .unwrap_or(0);
                select.set_selection(idx);
            }

            layout.add_child(TextView::new(label));
            layout.add_child(select.with_name(name.clone()).min_width(40).min_height(3));

            // Record cascading relationship
            if let Some(ref dep_name) = field.depends_on {
                if let Some(ref dep_map) = field.depends_map {
                    cascades.push((dep_name.clone(), name.clone(), dep_map.clone()));
                }
            }
        } else if field.multiline {
            let textarea = TextArea::new()
                .content(field.default.as_deref().unwrap_or(""));

            layout.add_child(TextView::new(label));
            layout.add_child(textarea.with_name(name.clone()).min_width(50).min_height(5));
        } else {
            let edit = EditView::new()
                .content(field.placeholder.as_deref().unwrap_or(""));

            layout.add_child(TextView::new(label));
            layout.add_child(edit.with_name(name.clone()).min_width(50));
        }
    }

    let buttons = LinearLayout::horizontal()
        .child(Button::new("Submit", {
            let result = result.clone();
            let fields: Vec<FormField> = fields.to_vec();
            move |s: &mut Cursive| {
                let mut values = HashMap::new();
                for field in &fields {
                    let name = &field.name;
                    if field.field_type == FieldType::Choice || field.field_type == FieldType::List {
                        if let Some(val) = s.call_on_name(name, |v: &mut SelectView<String>| {
                            v.selection().map(|s| (*s).clone())
                        }).flatten() {
                            values.insert(name.clone(), val);
                        }
                    } else if field.multiline {
                        if let Some(content) = s.call_on_name(name, |v: &mut TextArea| {
                            v.get_content().to_string()
                        }) {
                            values.insert(name.clone(), content);
                        }
                    } else {
                        if let Some(content) = s.call_on_name(name, |v: &mut EditView| {
                            (*v.get_content()).clone()
                        }) {
                            values.insert(name.clone(), content);
                        }
                    }
                }
                *result.lock().unwrap() = Some(FormResult { values });
                s.quit();
            }
        }))
        .child(Button::new("Cancel", {
            let result = result.clone();
            move |s: &mut Cursive| {
                *result.lock().unwrap() = None;
                s.quit();
            }
        }));

    layout.add_child(buttons);

    let dialog = Dialog::around(ScrollView::new(layout))
        .title(title)
        .h_align(HAlign::Center);

    siv.add_layer(dialog);

    // Wire up cascading selects after views are added to Cursive
    for (parent_name, child_name, dep_map) in &cascades {
        let child_name = child_name.clone();
        let dep_map = dep_map.clone();
        siv.call_on_name(parent_name, |parent: &mut SelectView<String>| {
            parent.set_on_select(move |s: &mut Cursive, selected: &String| {
                s.call_on_name(&child_name, |child: &mut SelectView<String>| {
                    child.clear();
                    if let Some(items) = dep_map.get(selected) {
                        for item in items {
                            child.add_item_str(item.clone());
                        }
                    }
                    child.set_selection(0);
                });
            });
        });
    }

    siv.set_fps(30);

    siv.add_global_callback(Key::Esc, |s| {
        s.quit();
    });

    siv.run();

    let result_ref = result.lock().unwrap().take();
    Ok(result_ref)
}
