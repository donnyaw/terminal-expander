use std::collections::HashMap;
use std::io::IsTerminal;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    Text,
    Choice,
    List,
    Checkbox,
    Password,
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
    #[cfg(unix)]
    let has_terminal = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/tty")
        .is_ok_and(|tty| tty.is_terminal());

    #[cfg(not(unix))]
    let has_terminal = std::io::stdout().is_terminal();

    has_terminal && std::env::var("TERM").ok().is_some_and(|t| t != "dumb")
}

fn render_cursive_form(title: &str, fields: &[FormField]) -> anyhow::Result<Option<FormResult>> {
    use cursive::align::HAlign;
    use cursive::event::Key;
    use cursive::traits::{Nameable, Resizable};
    use cursive::views::{
        Button, Checkbox, Dialog, EditView, LinearLayout, ScrollView, SelectView, TextArea,
        TextView,
    };
    use cursive::Cursive;

    if !is_interactive_terminal() {
        anyhow::bail!("Form requires an interactive terminal.");
    }

    let result = Arc::new(Mutex::new(None::<FormResult>));

    let mut siv = match std::panic::catch_unwind(cursive::default) {
        Ok(s) => s,
        Err(e) => {
            let msg = e
                .downcast_ref::<&str>()
                .map(|s| s.to_string())
                .or_else(|| e.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "unknown error".to_string());
            anyhow::bail!("Failed to initialize terminal UI: {}", msg);
        }
    };

    // Collect fields that need cascading: (parent_name, child_name, depends_map)
    let mut cascades: Vec<(String, String, HashMap<String, Vec<String>>)> = Vec::new();
    let mut initial_values: HashMap<String, String> = HashMap::new();

    let mut layout = LinearLayout::vertical();

    for field in fields {
        let label = field.label.clone();
        let name = field.name.clone();

        if field.field_type == FieldType::Choice || field.field_type == FieldType::List {
            let mut select = SelectView::new();
            let available_values = if let Some(ref dep_name) = field.depends_on {
                initial_values
                    .get(dep_name)
                    .and_then(|parent_val| field.depends_map.as_ref()?.get(parent_val).cloned())
            } else {
                field.values.clone()
            };

            if let Some(ref values) = available_values {
                for v in values {
                    if field.field_type == FieldType::List {
                        select.add_item(format!("- {}", v), v.clone());
                    } else {
                        select.add_item_str(v.clone());
                    }
                }
            }
            let idx = field
                .default
                .as_ref()
                .and_then(|default| available_values.as_ref()?.iter().position(|x| x == default))
                .unwrap_or(0);
            select.set_selection(idx);
            if let Some(value) = available_values.as_ref().and_then(|values| values.get(idx)) {
                initial_values.insert(name.clone(), value.clone());
            }

            layout.add_child(TextView::new(label));
            layout.add_child(select.with_name(name.clone()).min_width(40).min_height(3));

            if let Some(ref dep_name) = field.depends_on {
                if let Some(ref dep_map) = field.depends_map {
                    cascades.push((dep_name.clone(), name.clone(), dep_map.clone()));
                }
            }
        } else if field.field_type == FieldType::Checkbox {
            let checked = field.default.as_deref().is_some_and(|value| {
                matches!(
                    value.to_ascii_lowercase().as_str(),
                    "true" | "yes" | "1" | "on"
                )
            });
            let checkbox = if checked {
                Checkbox::new().checked()
            } else {
                Checkbox::new()
            };

            layout.add_child(TextView::new(label));
            layout.add_child(checkbox.with_name(name.clone()));
        } else if field.multiline {
            let textarea = TextArea::new().content(field.default.as_deref().unwrap_or(""));

            layout.add_child(TextView::new(label));
            layout.add_child(textarea.with_name(name.clone()).min_width(50).min_height(5));
        } else {
            let edit = EditView::new().content(
                field
                    .default
                    .as_deref()
                    .or(field.placeholder.as_deref())
                    .unwrap_or(""),
            );
            let edit = if field.field_type == FieldType::Password {
                edit.secret()
            } else {
                edit
            };

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
                    if field.field_type == FieldType::Choice || field.field_type == FieldType::List
                    {
                        if let Some(val) = s
                            .call_on_name(name, |v: &mut SelectView<String>| {
                                v.selection().map(|s| (*s).clone())
                            })
                            .flatten()
                        {
                            values.insert(name.clone(), val);
                        }
                    } else if field.field_type == FieldType::Checkbox {
                        if let Some(checked) =
                            s.call_on_name(name, |v: &mut Checkbox| v.is_checked())
                        {
                            values.insert(name.clone(), checked.to_string());
                        }
                    } else if field.multiline {
                        if let Some(content) =
                            s.call_on_name(name, |v: &mut TextArea| v.get_content().to_string())
                        {
                            values.insert(name.clone(), content);
                        }
                    } else if let Some(content) =
                        s.call_on_name(name, |v: &mut EditView| (*v.get_content()).clone())
                    {
                        values.insert(name.clone(), content);
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

    // Wire up cascading selects
    for (parent_name, child_name, dep_map) in &cascades {
        let child_name = child_name.clone();
        let dep_map = dep_map.clone();
        let _ = siv.call_on_name(parent_name, |parent: &mut SelectView<String>| {
            parent.set_on_select(move |s: &mut Cursive, selected: &String| {
                let _ = s.call_on_name(&child_name, |child: &mut SelectView<String>| {
                    child.clear();
                    if let Some(items) = dep_map.get(selected) {
                        for item in items {
                            child.add_item(format!("  {}", item), item.clone());
                        }
                    }
                    if !child.is_empty() {
                        child.set_selection(0);
                    }
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
