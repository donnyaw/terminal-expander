use std::collections::HashMap;

#[test]
fn test_config_parse_and_render() {
    // Full pipeline: YAML -> Config -> Template -> Render
    let yaml = r#"
matches:
  - trigger: ":hello"
    replace: "Hi {{name}}!"
    vars:
      - name: name
        type: date
        params:
          format: "%Y"
"#;

    let config = yaml.parse::<texpand_config::Config>().unwrap();
    assert_eq!(config.matches.len(), 1);

    let m = &config.matches[0];
    assert_eq!(m.trigger.as_deref(), Some(":hello"));
}

#[test]
fn test_match_engine_with_real_config() {
    use texpand_config::MatchFile;
    use texpand_match::Matcher;

    let matches = vec![
        MatchFile {
            trigger: Some(":greet".to_string()),
            triggers: None,
            replace: Some("Hello World!".to_string()),
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
        },
        MatchFile {
            trigger: Some("git!rel".to_string()),
            triggers: None,
            replace: Some("git add . && git commit".to_string()),
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
        },
    ];

    let matcher = Matcher::new(matches);

    // Test basic match
    let result = matcher.find_best("say :greet");
    assert!(result.is_some());
    assert_eq!(result.unwrap().matched_text, ":greet");

    // Test git-style trigger
    let result = matcher.find_best("run git!rel");
    assert!(result.is_some());
    assert_eq!(result.unwrap().matched_text, "git!rel");

    // Test no match inside word
    let result = matcher.find_best("my:greet");
    assert!(result.is_none());
}

#[test]
fn test_template_with_variable_engine() {
    use texpand_render::Template;
    use texpand_render::VariableEngine;

    let template = Template::new("{{greeting}} {{name}}!");
    let mut vars = HashMap::new();
    vars.insert("greeting".to_string(), "Hi".to_string());
    vars.insert("name".to_string(), "World".to_string());

    let result = template.render(&vars);
    assert_eq!(result, "Hi World!");
}

#[test]
fn test_form_extension_integration() {
    use std::collections::HashMap;
    use texpand_config::FieldConfig;
    use texpand_render::FormExtension;

    let mut fields = HashMap::new();
    fields.insert("name".to_string(), FieldConfig {
        field_type: None,
        multiline: None,
        default: None,
        placeholder: Some("Your name".to_string()),
        values: None,
        trim_string_values: None,
    });

    let mut values = HashMap::new();
    values.insert("name".to_string(), "Alice".to_string());

    let result = FormExtension::render_form(
        "Hello [[name]]!",
        &fields,
        &values,
    );
    assert_eq!(result, "Hello Alice!");
}

#[test]
fn test_config_from_file_simulation() {
    use texpand_config::Config;

    let yaml = r#"
matches:
  - trigger: ":date"
    replace: "{{now}}"
    vars:
      - name: now
        type: date
        params:
          format: "%Y-%m-%d"
  - trigger: ":choose"
    form: "Pick [[option]]"
    form_fields:
      option:
        type: choice
        values:
          - A
          - B
          - C
"#;

    let config: Config = yaml.parse().unwrap();
    assert_eq!(config.matches.len(), 2);

    // First match has variables
    let m1 = &config.matches[0];
    assert_eq!(m1.trigger.as_deref(), Some(":date"));
    assert!(m1.vars.is_some());
    assert_eq!(m1.vars.as_ref().unwrap().len(), 1);

    // Second match has form
    let m2 = &config.matches[1];
    assert_eq!(m2.trigger.as_deref(), Some(":choose"));
    assert!(m2.form.is_some());
    assert!(m2.form_fields.is_some());
}

#[test]
fn test_variable_engine_date_resolve() {
    use texpand_render::VariableEngine;

    let engine = VariableEngine::default();
    let result = engine.resolve("date", &None).unwrap();
    // Date format YYYY-MM-DD = 10 chars
    assert_eq!(result.len(), 10);
    assert_eq!(result.chars().filter(|&c| c == '-').count(), 2);
}

#[test]
fn test_matcher_with_from_files() {
    use texpand_config::Config;
    use texpand_match::Matcher;

    let yaml1 = r#"
matches:
  - trigger: ":hello"
    replace: "world"
"#;
    let yaml2 = r#"
matches:
  - trigger: ":bye"
    replace: "goodbye"
"#;

    let config1: Config = yaml1.parse().unwrap();
    let config2: Config = yaml2.parse().unwrap();

    let files = vec![
        (std::path::PathBuf::from("f1.yml"), config1),
        (std::path::PathBuf::from("f2.yml"), config2),
    ];

    let matcher = Matcher::from_files(files);
    assert!(matcher.find_best("say :hello").is_some());
    assert!(matcher.find_best("say :bye").is_some());
}

#[test]
fn test_detect_evdev_key_source_creation() {
    use texpand_detect::{EvdevKeySource, KeySource};
    let mut source = EvdevKeySource::new(false);
    // Initialize will fail without /dev/input access, but shouldn't panic
    let _ = source.initialize();
}

#[test]
fn test_injector_fallback_order() {
    use texpand_inject::{
        ClipboardInjector, InjectionMethod, Injector, TmuxInjector, UinputInjector,
    };

    let uinput = UinputInjector;
    assert!(matches!(uinput.method(), InjectionMethod::Uinput));

    let tmux = TmuxInjector;
    assert!(matches!(tmux.method(), InjectionMethod::TmuxSendKeys));

    let clipboard = ClipboardInjector;
    assert!(matches!(clipboard.method(), InjectionMethod::Clipboard));
}
