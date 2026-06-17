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
    pub section: Option<String>,
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
    fn show(
        &self,
        title: &str,
        fields: &[FormField],
    ) -> anyhow::Result<Option<FormResult>>;

    /// Show form with a trigger name displayed in the header
    fn show_with_trigger(
        &self,
        title: &str,
        trigger: &str,
        fields: &[FormField],
    ) -> anyhow::Result<Option<FormResult>> {
        let combined = format!("{}{}{}", title, "\x1f", trigger);
        self.show(&combined, fields)
    }
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

fn filter_options(options: &[String], query: &str) -> Vec<String> {
    let needle = query.trim().to_lowercase();
    if needle.is_empty() {
        return options.to_vec();
    }

    options
        .iter()
        .filter(|value| value.to_lowercase().contains(&needle))
        .cloned()
        .collect()
}

fn clean_label(label: &str) -> String {
    label
        .trim()
        .trim_start_matches(['-', '*', '|', '║', '│', ' '])
        .trim()
        .trim_end_matches([':', '|', '║', '│', ' '])
        .trim()
        .to_string()
}

fn display_label(label: &str, searchable: bool) -> String {
    let base = clean_label(label);
    if searchable {
        if base.contains("/ search") || base.contains("/search") {
            format!("  - {}:", base)
        } else {
            format!("  - {} (/ search):", base)
        }
    } else {
        format!("  - {}:", base)
    }
}

fn indented_view<V: cursive::View + 'static>(view: V) -> cursive::views::LinearLayout {
    cursive::views::LinearLayout::horizontal()
        .child(cursive::views::TextView::new("   "))
        .child(view)
}

fn add_field_block<V: cursive::View + 'static>(
    layout: &mut cursive::views::LinearLayout,
    label: String,
    view: V,
) {
    layout.add_child(cursive::views::TextView::new(""));
    layout.add_child(cursive::views::TextView::new(label));
    layout.add_child(indented_view(view));
}

fn populate_select(
    select: &mut cursive::views::SelectView<String>,
    values: &[String],
    is_list: bool,
) {
    select.clear();
    for value in values {
        if is_list {
            select.add_item(format!("- {}", value), value.clone());
        } else {
            select.add_item_str(value.clone());
        }
    }

    if !values.is_empty() {
        select.set_selection(0);
    }
}

fn select_by_value(s: &mut cursive::Cursive, field_name: &str, value: &str) {
    let _ = s.call_on_name(
        field_name,
        |view: &mut cursive::views::SelectView<String>| {
            let items: Vec<String> = view.iter().map(|(_, item)| item.clone()).collect();
            if let Some(index) = items.iter().position(|item| item.as_str() == value) {
                view.set_selection(index);
            }
        },
    );
}

fn refresh_search_results(
    s: &mut cursive::Cursive,
    results_name: &str,
    count_name: &str,
    options: &[String],
    query: &str,
) {
    let filtered = filter_options(options, query);
    let count_text = if filtered.len() == 1 {
        "1 match".to_string()
    } else {
        format!("{} matches", filtered.len())
    };

    let _ = s.call_on_name(count_name, |view: &mut cursive::views::TextView| {
        view.set_content(count_text);
    });

    let _ = s.call_on_name(
        results_name,
        |view: &mut cursive::views::SelectView<String>| {
            view.clear();
            if filtered.is_empty() {
                view.add_item_str("No matches".to_string());
                return;
            }
            for value in filtered {
                view.add_item_str(value);
            }
            if !view.is_empty() {
                view.set_selection(0);
            }
        },
    );
}

fn open_dropdown_search(
    s: &mut cursive::Cursive,
    field_name: String,
    field_label: String,
    option_store: Arc<Mutex<HashMap<String, Vec<String>>>>,
) {
    use cursive::event::Key;
    use cursive::traits::{Nameable, Resizable};
    use cursive::views::{Dialog, EditView, LinearLayout, OnEventView, SelectView, TextView};

    let options = option_store
        .lock()
        .ok()
        .and_then(|store| store.get(&field_name).cloned())
        .unwrap_or_default();

    let results_name = format!("{}_search_results", field_name);
    let count_name = format!("{}_search_count", field_name);
    let query_name = format!("{}_search_query", field_name);

    let results_name_for_edit = results_name.clone();
    let count_name_for_edit = count_name.clone();
    let options_for_edit = options.clone();
    let search_input = EditView::new()
        .on_edit(move |s, query, _| {
            refresh_search_results(
                s,
                &results_name_for_edit,
                &count_name_for_edit,
                &options_for_edit,
                query,
            );
        })
        .with_name(query_name)
        .fixed_width(32);

    let field_name_for_submit = field_name.clone();
    let results = SelectView::<String>::new()
        .on_submit(move |s, value| {
            if value == "No matches" {
                return;
            }
            select_by_value(s, &field_name_for_submit, value);
            s.pop_layer();
        })
        .with_name(results_name)
        .min_height(8);

    let layout = LinearLayout::vertical()
        .child(TextView::new(format!(
            "Query: {}",
            clean_label(&field_label)
        )))
        .child(search_input)
        .child(TextView::new("0 matches").with_name(count_name))
        .child(results)
        .child(TextView::new("Enter select | Esc close"));

    let results_name_for_initial = format!("{}_search_results", field_name);
    let count_name_for_initial = format!("{}_search_count", field_name);
    let query_name_for_focus = format!("{}_search_query", field_name);
    let dialog = OnEventView::new(
        Dialog::around(layout)
            .title(format!("Search: {}", clean_label(&field_label)))
            .button("Close", |s| {
                s.pop_layer();
            }),
    )
    .on_event(Key::Esc, |s| {
        s.pop_layer();
    });

    s.add_layer(dialog);
    let _ = s.focus_name(&query_name_for_focus);
    refresh_search_results(
        s,
        &results_name_for_initial,
        &count_name_for_initial,
        &options,
        "",
    );
}

fn render_cursive_form(title: &str, fields: &[FormField]) -> anyhow::Result<Option<FormResult>> {
    use cursive::align::HAlign;
    use cursive::event::Key;
    use cursive::traits::{Nameable, Resizable};
    use cursive::views::{
        Button, Checkbox, Dialog, EditView, LinearLayout, OnEventView, ScrollView, SelectView,
        TextArea, TextView,
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
    let option_store: Arc<Mutex<HashMap<String, Vec<String>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    // Show trigger name in header if passed via show_with_trigger
    let header_trigger = title.find('\x1f').map(|pos| &title[pos + 1..]);
    let display_title = header_trigger.unwrap_or(title);

    let mut layout = LinearLayout::vertical();
    layout.add_child(TextView::new(format!("  ▸ {}", display_title)));
    layout.add_child(TextView::new(
        "Tab next  |  / search dropdown  |  Ctrl+Enter submit",
    ));
    layout.add_child(TextView::new(
        "-----------------------------------------------",
    ));

    let mut current_section: Option<String> = None;

    for field in fields {
        let label = field.label.clone();
        let name = field.name.clone();

        if field.section != current_section {
            if let Some(section) = &field.section {
                layout.add_child(TextView::new(""));
                layout.add_child(TextView::new(section.clone()));
            }
            current_section = field.section.clone();
        }

        if field.field_type == FieldType::Choice || field.field_type == FieldType::List {
            let is_list = field.field_type == FieldType::List;
            let mut select = SelectView::new();
            let available_values = if let Some(ref dep_name) = field.depends_on {
                initial_values
                    .get(dep_name)
                    .and_then(|parent_val| field.depends_map.as_ref()?.get(parent_val).cloned())
            } else {
                field.values.clone()
            };

            if let Some(ref values) = available_values {
                if let Ok(mut store) = option_store.lock() {
                    store.insert(name.clone(), values.clone());
                }
            } else {
                if let Ok(mut store) = option_store.lock() {
                    store.remove(&name);
                }
            }

            if let Some(ref values) = available_values {
                populate_select(&mut select, values, is_list);
            }
            let idx = field
                .default
                .as_ref()
                .and_then(|default| available_values.as_ref()?.iter().position(|x| x == default))
                .unwrap_or(0);
            if !select.is_empty() {
                select.set_selection(idx.min(select.len().saturating_sub(1)));
            }
            if let Some(value) = available_values.as_ref().and_then(|values| values.get(idx)) {
                initial_values.insert(name.clone(), value.clone());
            }

            let field_name = name.clone();
            let field_label = label.clone();
            let option_store_for_search = option_store.clone();
            let searchable =
                OnEventView::new(select.with_name(name.clone()).min_width(40).min_height(3))
                    .on_event('/', move |s| {
                        open_dropdown_search(
                            s,
                            field_name.clone(),
                            field_label.clone(),
                            option_store_for_search.clone(),
                        );
                    });
            add_field_block(&mut layout, display_label(&label, true), searchable);

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

            add_field_block(
                &mut layout,
                display_label(&label, false),
                checkbox.with_name(name.clone()),
            );
        } else if field.multiline {
            let textarea = TextArea::new().content(field.default.as_deref().unwrap_or(""));

            add_field_block(
                &mut layout,
                display_label(&label, false),
                textarea.with_name(name.clone()).min_width(50).min_height(5),
            );
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

            add_field_block(
                &mut layout,
                display_label(&label, false),
                edit.with_name(name.clone()).min_width(50),
            );
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
        let option_store = option_store.clone();
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
                if let Ok(mut store) = option_store.lock() {
                    store.insert(
                        child_name.clone(),
                        dep_map.get(selected).cloned().unwrap_or_default(),
                    );
                }
            });
        });
    }

    siv.set_fps(30);

    // Swallow Enter globally so paste trailing newlines don't trigger Submit
    // Users must click the Submit or Cancel button (Tab + Enter) to close the form
    siv.add_global_callback(Key::Enter, |_| {});

    siv.run();

    let result_ref = result.lock().unwrap().take();
    Ok(result_ref)
}

#[cfg(test)]
mod tests {
    use super::filter_options;

    #[test]
    fn test_filter_options_matches_substrings_case_insensitive() {
        let options = vec![
            "Alpha".to_string(),
            "postgresql".to_string(),
            "Fox".to_string(),
        ];
        assert_eq!(
            filter_options(&options, "post"),
            vec!["postgresql".to_string()]
        );
        assert_eq!(filter_options(&options, "FO"), vec!["Fox".to_string()]);
    }

    #[test]
    fn test_filter_options_returns_all_for_empty_query() {
        let options = vec!["A".to_string(), "B".to_string()];
        assert_eq!(filter_options(&options, ""), options);
    }
}
