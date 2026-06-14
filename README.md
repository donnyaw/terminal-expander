# terminal-expander

A terminal-native text expander with Espanso-compatible form support. Written in Rust.

## Features

- **Trigger-based expansion** — Type `:hello` and have it expand to `Hello World!`
- **Multi-field forms** — Interactive forms with text fields, multiline, choice dropdowns, and list selectors — all rendered in the terminal via Cursive
- **Espanso-compatible config** — Use your existing Espanso `match/*.yml` files
- **Variable system** — Date, clipboard, shell command, and form variable injection
- **Shell plugins** — Expansion hooks for zsh, bash, and fish
- **System-wide detection** — evdev-based keyboard monitoring (Linux)
- **Multiple injection methods** — uinput, ydotool, tmux send-keys, clipboard

## Project Status

Currently in early development. Core engine, config parser, match engine, template renderer, form renderer, shell plugins, and platform layer are implemented. See `tasks.csv` for detailed progress.

## Architecture

```
texpand-cli/         ← Main binary (CLI)
texpand-config/      ← YAML config parser (Espanso-compatible)
texpand-match/       ← Trigger detection and matching engine
texpand-render/      ← Template rendering and variable system
texpand-ui/          ← Cursive-based TUI form renderer
texpand-detect/      ← evdev keyboard event detection
texpand-inject/      ← Text injection (uinput, tmux, clipboard)
```

## Quick Start

```bash
# Build
cargo build

# Run tests
cargo test

# Create a match file
mkdir -p ~/.config/texpand/matches
cat > ~/.config/texpand/matches/base.yml << 'EOF'
matches:
  - trigger: ":hello"
    replace: "Hello World!"
EOF

# Run the expander (requires shell plugin or system-wide mode)
texpand expand ":hello"
```

## Shell Integration

### Zsh
```bash
source /path/to/shell/texpand.zsh
```

### Bash
```bash
source /path/to/shell/texpand.bash
```

### Fish
```fish
source /path/to/shell/texpand.fish
```

## Configuration

terminal-expander uses Espanso-compatible YAML match files:

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
```

## License

MIT
