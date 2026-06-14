# Architecture Guide

## Crate Map

```
                    ┌─────────────┐
                    │ texpand-cli │
                    └──────┬──────┘
          ┌────────────────┼────────────────┐
          v                v                v
  ┌────────────┐   ┌────────────┐   ┌──────────────┐
  │texpand-    │   │texpand-    │   │texpand-render│
  │config      │   │match       │   │              │
  └────────────┘   └────────────┘   └──────────────┘
          ┌────────────────┼────────────────┐
          v                v                v
  ┌────────────┐   ┌────────────┐   ┌──────────────┐
  │texpand-    │   │texpand-    │   │texpand-ui    │
  │detect      │   │inject      │   │              │
  └────────────┘   └────────────┘   └──────────────┘
```

## Crate Responsibilities

### texpand-config
Parses Espanso-compatible YAML match files. Defines `Config`, `MatchFile`, `FieldConfig`, and `VariableDef` structs. Uses `serde_norway` for YAML parsing to support Espanso's multi-line `form: |` syntax.

### texpand-match
Trigger detection engine. Maintains a `RollingBuffer` of recent keystrokes and a `Matcher` that finds matching triggers with word boundary detection and longest-match disambiguation.

### texpand-render
Template rendering and variable system. Resolves `{{variable}}` placeholders via `VariableEngine` with support for:
- `date` — Formatted date/time with offset
- `clipboard` — Current clipboard contents
- `shell` — Command output
- `form` — Form field values (via `FormExtension`)

### texpand-ui
Cursive-based terminal form renderer. Supports:
- Single-line text fields (`EditView`)
- Multi-line text areas (`TextArea`)
- Choice dropdowns (`SelectView::popup()`)
- Tab navigation, Ctrl+Enter submit, Esc cancel

### texpand-detect
evdev-based keyboard event detection. Scans `/dev/input/by-path/` for keyboard devices and provides `InputEvent` stream.

### texpand-inject
Text injection with fallback chain: uinput → ydotool → tmux send-keys → clipboard.

### texpand-cli
Main binary with `clap`-based CLI.

## Key Traits

```rust
// texpand-config
pub trait ConfigProvider { fn load(&self) -> Result<Config>; }

// texpand-match
pub trait MatchDetector { fn find(&self, input: &str) -> Option<MatchResult>; }

// texpand-render
pub trait VariableResolver { fn resolve(&self, params) -> Result<String>; }

// texpand-ui
pub trait FormRenderer { fn show(&self, title, fields) -> Result<Option<FormResult>>; }

// texpand-detect
pub trait KeySource { fn initialize(&mut self); fn read_event(&mut self); }

// texpand-inject
pub trait Injector { fn inject(&self, text: &str) -> Result<()>; }
```

## Adding a New Variable Type

1. Create a struct implementing `VariableResolver` in `texpand-render/src/variables.rs`
2. Implement the `resolve()` method
3. Register it in `VariableEngine::default()`
4. Add tests

## Adding a New Form Field Type

1. Add variant to `FieldType` enum in `texpand-ui/src/form.rs`
2. Add rendering logic in `render_cursive_form()`
3. Add value extraction logic in the Submit button callback
4. Add tests
