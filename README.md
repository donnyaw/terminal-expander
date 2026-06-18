# cli-expander

An **interactive command builder for terminal users** with multi-field forms, fzf-powered trigger discovery, and a portable CSV trigger database. Written in Rust.

Type `:hello[Space]` and it expands to `Hello World!` inline in your shell prompt. Need a complex `find` command? Type `:findx[Space]`, fill in the terminal form, and cli-expander builds the command for you. Forgot a trigger name? In Bash, press `Ctrl+F` and fuzzy-search across your own large library of CLI snippets and command builders.

---

## Demo

```text
:hello[Space]  ->  Hello World!
:date[Space]   ->  2026-06-17
:findx[Space]  ->  opens an interactive command builder form
```

---

## Features

### Trigger Expansion

Type a trigger and press Space to expand it inline in your shell prompt. Static replacements and generated commands both land in the editable command line before you run them.

```text
:hello[Space]  ->  Hello World!
:date[Space]   ->  2026-06-17
```

Cursor positioning is supported with `$|$` markers in shell integrations that handle the marker.

### Interactive Command Forms

Complex CLI commands can be defined as guided terminal forms. Fill in fields step-by-step, submit the form, and the generated command is inserted into your prompt.

Supported field types:

| Type | Description |
|------|-------------|
| Text | Single-line text input |
| Multiline | Multi-line text area |
| Choice | Dropdown with single selection, searchable via `/` |
| List | Scrollable list, searchable via `/` |
| Checkbox | Boolean toggle (`[X]` / `[ ]`) |
| Password | Masked text input |
| Cascade | Dependent dropdown where child options change with the parent |

Example form layout:

```text
┌─ cli-expander > :findx ───────────────────────┐
│ Tab next | / search dropdown | Ctrl+Enter submit│
│────────────────────────────────────────────────│
│ Search Path:                                   │
│   [........................................]   │
│ File Type:                                     │
│   [v -type f                              ]    │
│ Extension:                                     │
│   [sh.....................................]    │
│                                                │
│ [Submit]                              [Cancel] │
└────────────────────────────────────────────────┘
```

### FZF Trigger Search

Bash supports `Ctrl+F` and `Alt+F` trigger discovery through `fzf`. The search uses `ce list --csv`, shows trigger descriptions, and previews details with `ce details`.

```text
:findx     Interactive form with 4 fields
:ticket    Create a ticket with fields
:priority  Set priority level (choice)
```

Zsh and Fish currently support expansion keybindings, but their checked-in plugins do not yet implement `Ctrl+F` fzf search.

### CSV Trigger Database

Triggers can be indexed into `~/.config/cli-expander/triggers.csv`. The CSV is generated from YAML match files and merge mode preserves manual rows.

```csv
trigger,description,category,type,source_file
:hello,"Hello World!",base,text,/home/.../base.yml
:findx,"Interactive form with 4 fields",forms-advanced,form,/home/.../forms-advanced.yml
```

### Variables

Templates support values resolved at expansion time.

| Variable Type | Example | Description |
|---------------|---------|-------------|
| Date | `{{today}}` | Current date/time with a strftime format |
| Clipboard | `{{clip}}` | System clipboard content |
| Shell | `{{branch}}` | stdout from a shell command |
| Form | `{{form.field}}` | Values submitted from an interactive form |

Shell variables execute through `sh -c`, so only use trusted match files.

---

## Installation

### From Source

```bash
cargo build --release
mkdir -p ~/.local/bin
cp target/release/ce ~/.local/bin/ce
```

The workspace also builds a `cli-expander` binary. `ce` is the short alias used by the shell plugins and examples.

### Verify

```bash
ce --help
ce config
```

### Optional Dependencies

| Dependency | Used For |
|------------|----------|
| `fzf` | Bash `Ctrl+F` trigger search |
| Clipboard provider | `clipboard` variables |
| Interactive terminal with `TERM` set | Form rendering |

---

## Quick Start

Create your first match file:

```bash
mkdir -p ~/.config/cli-expander/matches
cat > ~/.config/cli-expander/matches/base.yml << 'EOF'
matches:
  - trigger: ":hello"
    replace: "Hello World!"
  - trigger: ":date"
    replace: "{{now}}"
    vars:
      - name: now
        type: date
        params:
          format: "%Y-%m-%d"
EOF
```

Generate the CSV index:

```bash
ce generate-csv --force
```

Source a shell plugin, then type `:hello` and press Space.

---

## Shell Integration

Source the plugin for your shell. Use an absolute path when adding this to your shell startup file.

```bash
# Bash
source /path/to/cli-expander/shell/cli-expander.bash

# Zsh
source /path/to/cli-expander/shell/cli-expander.zsh

# Fish
source /path/to/cli-expander/shell/cli-expander.fish
```

Current checked-in shell capabilities:

| Feature | Bash | Zsh | Fish |
|---------|------|-----|------|
| Expand on Space | Yes | Auto-insert hook | Yes |
| Manual expand `Ctrl+T` | Yes | Yes | Yes |
| FZF search `Ctrl+F` | Yes | Not yet | Not yet |
| FZF search `Alt+F` | Yes | Not yet | Not yet |
| `$|$` cursor marker handling | Yes | Not yet documented | Not yet documented |

The primary workflow is to expand into the editable prompt first, inspect the result, then press Enter when you are ready to execute it.

---

## Match File Example

```yaml
matches:
  - trigger: ":hello"
    replace: "Hello World!"

  - trigger: ":findx"
    replace: "find {{path}} {{type}} -name '*.{{ext}}' -exec {{exec}} {} +"
    vars:
      - name: form
        type: form
        params:
          layout: |
            Search Path: [[path]]
            File Type: [[type]]
            Extension: [[ext]]
            Action: [[exec]]
          fields:
            path:
              default: "."
            type:
              type: choice
              values:
                - ""
                - "-type f"
                - "-type d"
            ext:
              default: "sh"
            exec:
              default: "ls -lh"
```

More examples are available in `examples/base.yml` and `examples/forms-advanced.yml`.

---

## CLI Reference

| Command | Description |
|---------|-------------|
| `ce <trigger>` | Shorthand for expanding a trigger |
| `ce expand <input>` | Expand a trigger or buffer string |
| `ce list` | List available triggers |
| `ce list --csv` | Output triggers as CSV |
| `ce list --json` | Output triggers as JSON |
| `ce generate-csv` | Regenerate `triggers.csv` in merge mode |
| `ce generate-csv --force` | Regenerate `triggers.csv` and overwrite existing rows |
| `ce generate-csv --output PATH` | Write the CSV index to a custom path |
| `ce search <query>` | Search trigger names, descriptions, and categories |
| `ce search <query> --csv` | Output search results as CSV |
| `ce details <trigger>` | Show trigger metadata |
| `ce form <layout>` | Open a simple interactive form for a layout string |
| `ce config` | Show configuration paths and shell plugin names |

The default match directory is `~/.config/cli-expander/matches`.

---

## Configuration

cli-expander recursively loads `.yml` and `.yaml` files from the match directory.

```text
~/.config/cli-expander/
├── matches/
│   ├── base.yml
│   └── tools/
│       └── find.yml
└── triggers.csv
```

Common match keys:

| Key | Purpose |
|-----|---------|
| `trigger` | Single trigger string |
| `triggers` | Multiple trigger strings for one replacement |
| `replace` | Static or templated replacement text |
| `form` | Form layout using `[[field]]` placeholders |
| `form_fields` | Field configuration for `form` layouts |
| `vars` | Variable definitions for templates |
| `search_label` | Description used by trigger listings |

Some Espanso-compatible keys are parsed for compatibility even if the current CLI path does not use every behavior yet.

---

## Troubleshooting

| Problem | Fix |
|---------|-----|
| `ce: command not found` | Add `~/.local/bin` to `PATH` or copy `target/release/ce` there |
| `fzf not found` | Install `fzf` or use `ce search <query>` instead |
| No trigger expands | Check `ce list` and confirm your YAML files are under `~/.config/cli-expander/matches` |
| Form does not open | Ensure you are in an interactive terminal and `TERM` is not `dumb` |
| Clipboard variable fails | Confirm a clipboard provider is available in your desktop/session |
| YAML is skipped | Run `ce list` and inspect YAML indentation, field names, and quoting |

---

## Architecture

```text
cli-expander/            Rust workspace
├── cli-expander-cli/    Main binary, CLI commands, field builders
├── cli-expander-config/ YAML config parser and CSV records
├── cli-expander-match/  Trigger detection and matching
├── cli-expander-render/ Template engine and variable resolvers
├── cli-expander-ui/     Cursive TUI form renderer
├── cli-expander-detect/ Input detection layer
├── cli-expander-inject/ Text injection layer
└── shell/               Bash, Zsh, and Fish plugins
```

---

## Development

```bash
cargo test
cargo clippy -- -D warnings
```

Current test status: `67` tests pass across the workspace.

Planning and agent notes live under `dev/`. Those files are historical/project-planning material and may describe planned work that is not part of the current user-facing behavior.

---

## License

MIT
