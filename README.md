# cli-expander

A terminal-native text expander with multi-field form support. Written in Rust.

## Features

- **Trigger-based expansion** — Type `:hello` and have it expand to `Hello World!`
- **Prompt-first workflow** — Type `:find[Space]`, fill a TUI form, then edit or run the generated command from the shell prompt
- **Multi-field forms** — Interactive forms with text, password, checkbox, multiline, choice dropdowns, list selectors, cascade dropdowns, and mixed text-after-dropdown layouts — all rendered in the terminal via Cursive
- **Searchable dropdowns** — Press `/` inside choice/list fields to filter large option sets with simple substring search
- **YAML config** — Define matches in `~/.config/cli-expander/matches/*.yml` files
- **Variable system** — Date, clipboard, shell command, and form variable injection
- **Shell plugins** — Expansion hooks for zsh, bash, and fish
- **System-wide detection** — evdev-based keyboard monitoring (Linux)
- **Multiple injection methods** — uinput, ydotool, tmux send-keys, clipboard

## Architecture

```
cli-expander-cli/         ← Main binary (CLI)
cli-expander-config/      ← YAML config parser
cli-expander-match/       ← Trigger detection and matching engine
cli-expander-render/      ← Template rendering and variable system
cli-expander-ui/          ← Cursive-based TUI form renderer
cli-expander-detect/      ← evdev keyboard event detection
cli-expander-inject/      ← Text injection (uinput, tmux, clipboard)
```

## Quick Start

```bash
# Build
cargo build

# Run tests
cargo test

# Create a match file
mkdir -p ~/.config/cli-expander/matches
cat > ~/.config/cli-expander/matches/base.yml << 'EOF'
matches:
  - trigger: ":hello"
    replace: "Hello World!"
EOF

# Run the expander (requires shell plugin or system-wide mode)
cli-expander expand ":hello"
```

## Shell Integration

The primary workflow is `:trigger[Space]`: the expansion is inserted into the
current shell prompt so you can review or edit it before pressing Enter.

```text
:findname[Space]  # open form, insert generated find command into prompt
```

Avoid `:trigger[Enter]` for command builders. Enter executes the typed trigger
as a shell command, while Space expands it into the editable command line.

### Latest Form Examples

The repo includes a few ready-to-test examples in `examples/forms-advanced.yml`:

- `:afterdropdown[Space]` - dropdown first, then text inputs after it
- `:cascade-mixed[Space]` - cascade dropdown plus normal text fields
- `:bigdropdown[Space]` - large choice list for testing `/` search

Inside a dropdown field, press `/` to search and narrow the choices.

### Zsh
```bash
source /path/to/shell/cli-expander.zsh
```

### Bash
```bash
source /path/to/shell/cli-expander.bash
```

### Fish
```fish
source /path/to/shell/cli-expander.fish
```

## Configuration

cli-expander uses YAML match files:

```yaml
matches:
  - trigger: ":greet"
    replace: "Hi {{name}}!"
    vars:
      - name: name
        type: form
        params:
          layout: "Enter your name: [[name]]"

  - trigger: ":date"
    replace: "{{now}}"
    vars:
      - name: now
        type: date
        params:
          format: "%Y-%m-%d"

  - trigger: ":choose"
    form: "Pick one: [[option]]"
    form_fields:
      option:
        type: choice
        values:
          - Option A
          - Option B

  - trigger: ":bigdropdown"
    form: "Selected package: [[package]]"
    form_fields:
      package:
        type: choice
        values:
          - nginx
          - postgresql
          - redis-server
```

## License

MIT
