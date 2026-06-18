# cli-expander

A **terminal-native text expander** with multi-field form support, fzf-integrated trigger discovery, and a portable CSV trigger database. Written in Rust.

Type `:hello[Space]` and it expands to `Hello World!` inline in your bash prompt. Need a complex `find` command? Type `:findx[Enter]` and fill in a step-by-step wizard. Forgot a trigger name? Press `Ctrl+F` and fuzzy-search across all 250+ triggers with descriptions.

---

## Features

### Trigger Expansion (`:hello[Space]`)
Type a trigger and press Space — the text expands inline in your shell prompt. No Enter needed. Supports cursor positioning with `$|$` markers.
```
:hello[Space]  →  Hello World!
:date[Space]   →  2026-06-17
```

### Fuzzy Trigger Search (`Ctrl+F`)
Press `Ctrl+F` in your bash prompt — fzf opens with all 250+ triggers and their descriptions. Type to filter, select with arrow keys, press Enter to expand inline. No need to memorize trigger names.
```
┌────────────────────────────────────────────────┐
│  :findx    Interactive form with 4 fields      │
│  :fd-name  Find files/dirs by name pattern     │
│  :ticket   Create a ticket with fields          │
│  :priority Set priority level (choice)          │
│                                                │
│  > search query                                 │
└────────────────────────────────────────────────┘
```

### Interactive Forms (`:trigger[Enter]`)
Complex CLI commands become simple Q&A forms. Fill in fields step-by-step, then the command is built for you.

**Supported field types:**
| Type | Description |
|------|-------------|
| Text | Single-line text input |
| Multiline | Multi-line text area |
| Choice | Dropdown with single selection, searchable via `/` |
| List | Scrollable list, searchable via `/` |
| Checkbox | Boolean toggle (`[X]` / `[ ]`) |
| Password | Masked text input |
| Cascade | Dependent dropdown — child options change when parent changes |

**Hierarchical form layout:**
```
┌─ cli-expander ▸ :findx ───────────────────────┐
│ Tab next | / search dropdown | Ctrl+Enter submit│
│────────────────────────────────────────────────│
│                                                 │
│ ── Scope ──                                      │
│                                                 │
│   Search Path:                                  │
│     [./home.............................]       │
│                                                 │
│   File Type:                                    │
│     [▼ -type f                           ]      │
│                                                 │
│ ── Criteria ──                                   │
│                                                 │
│   Extension:                                    │
│     [sh......................................]  │
│                                                 │
│ [Submit]                              [Cancel]  │
└─────────────────────────────────────────────────┘
```

### Searchable Dropdowns
Press `/` when focused on a Choice or List field. A search dialog opens with live filtering and match count.
```
┌─────────── Search: Package ───────────┐
│   Query: Package                      │
│   ┌──────────────────────────────┐    │
│   │ [postgresql               ] │    │
│   └──────────────────────────────┘    │
│   2 matches                          │
│   ┌──────────────────────────────┐    │
│   │ postgresql                    │    │
│   │ postgresql-contrib            │    │
│   └──────────────────────────────┘    │
│   Enter select | Esc close            │
└──────────────────────────────────────┘
```

### Cascade / Dependent Dropdowns
Child dropdown options update dynamically when the parent selection changes. Defined via `depends_on` in the YAML config.

### Portable CSV Trigger Database
All triggers are indexed in `~/.config/cli-expander/triggers.csv`. Auto-generated from YAML match files, with manual row preservation.

```csv
trigger,description,category,type,source_file
:hello,"Hello World!",base,text,/home/.../base.yml
:findx,"Interactive form with 4 fields",forms-advanced,form,/home/.../forms-advanced.yml
```

**Commands:**
| Command | Description |
|---------|-------------|
| `ce list --csv` | Output all triggers as CSV (pipe to fzf) |
| `ce list --json` | Output all triggers as JSON |
| `ce details <trigger>` | Show trigger info (description, category, type, source) |
| `ce search <query>` | Text search without fzf |
| `ce generate-csv [--force]` | Regenerate CSV from YAML (merge mode preserves manual rows) |

### Variable System
| Variable | Syntax | Description |
|----------|--------|-------------|
| Date | `{{date}}` | Current date/time with strftime format |
| Clipboard | `{{clipboard}}` | System clipboard content |
| Shell | `{{shell}}` | Shell command stdout |
| Form | `{{form.field}}` | Interactive form values |

### Shell Plugins
| Shell | File | Keybindings |
|-------|------|-------------|
| Bash | `shell/cli-expander.bash` | Space, Ctrl+T, Ctrl+F, Alt+F |
| Zsh | `shell/cli-expander.zsh` | Space, Ctrl+T, Ctrl+F |
| Fish | `shell/cli-expander.fish` | Space, Ctrl+T, Ctrl+F |

### Auto-Regeneration (systemd)
When YAML match files change, `triggers.csv` is automatically regenerated via a systemd path watcher. No manual `ce generate-csv` needed.

### Paste Protection
Enter key is swallowed globally in forms to prevent accidental submission when pasting text via tmux. Use the Submit/Cancel buttons or `Ctrl+Enter` to submit.

---

## Quick Start

```bash
# Build
cargo build --release
cp target/release/ce ~/.local/bin/

# Create first match file
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

# Source the plugin
source shell/cli-expander.bash

# Try it
:hello[Space]     → Hello World!
:date[Space]      → 2026-06-17

# Browse triggers
Ctrl+F            → fzf search all triggers

# Generate CSV database
ce generate-csv --force
```

---

## Architecture

```
cli-expander/           ← Rust workspace (7 crates)
├── cli-expander-cli/   ← Main binary (CLI, commands, field builders)
├── cli-expander-config/← YAML config parser (FieldConfig, MatchFile)
├── cli-expander-match/ ← Trigger detection engine
├── cli-expander-render/← Template engine + Variable resolvers
├── cli-expander-ui/    ← Cursive TUI form renderer
├── cli-expander-detect/← evdev keyboard detection (Linux)
├── cli-expander-inject/← Text injection (uinput, tmux, clipboard)
└── shell/              ← Bash/Zsh/Fish plugins
    └── cli-expander.bash
```

---

## Developer Notes

- **`dev/` folder** — Contains AI-generated build plans and session records. Ignored by git (`.gitignore`) and syncthing (`.stignore`) to keep the main repository clean.
- **Config packs** — Extended match packs live in `/home/rezriz/github/common-config/cli-expander/`. Symlink to `~/.config/cli-expander/matches/` to use them.
- **Crates** — The workspace has 7 crates: `cli-expander-cli`, `cli-expander-config`, `cli-expander-match`, `cli-expander-render`, `cli-expander-ui`, `cli-expander-detect`, `cli-expander-inject`.

---

## Testing

```bash
cargo test                    # 67+ tests
cargo clippy -- -D warnings   # Zero warnings
```

---

## License

MIT
