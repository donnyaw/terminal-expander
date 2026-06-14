# Terminal Text Expander — Project Knowledgebase

## Project Summary

- Project name: `texpand` (working title)
- Repository root: `$PROJECT_ROOT`
- Product type: CLI tool — terminal-native text expander
- Language: Rust
- Primary users: Developers, SysAdmins, terminal power users
- Business goal: A privacy-first, terminal-native text expander with Espanso-compatible forms, running entirely in the terminal (no GUI)

## Stack

| Layer | Choice | Rationale |
|-------|--------|-----------|
| Language | Rust | Performance, safety, Espanso is also Rust |
| TUI framework | Cursive (v0.21+) | Built-in form widgets (EditView, TextArea, SelectView.popup, Dialog, Button). No third-party form deps needed. |
| Backend | Ratatui | Alternative if Cursive doesn't fit — more popular (21k stars) but needs ratatui-textarea for input |
| Key detection | evdev crate | System-wide, works on Wayland/X11/TTY |
| Text injection | evdev uinput | System-wide, proven by srkt project |
| Config format | YAML via serde_norway | Espanso-compatible — serde_yaml breaks on multiline `form: \|` |
| Template engine | Custom (Espanso-compatible) | `{{field_name}}` syntax, `"$\|$"` cursor marker |
| Clipboard | arboard crate | Cross-platform clipboard access |
| Shell integration | zle (zsh), bind -x (bash), function (fish) | Zero-permission shell-level expansion |

## Key Dependencies

| Crate | Purpose | Notes |
|-------|---------|-------|
| cursive (0.21) | TUI form rendering | Built-in: EditView, TextArea, SelectView, Dialog |
| evdev (0.13) | Keyboard event reading | /dev/input/event* — needs `input` group |
| serde + serde_norway | YAML config parsing | serde_norway for Espanso compat |
| clap (4.x) | CLI argument parsing | Standard Rust CLI framework |
| anyhow | Error handling | Simple error propagation |
| arboard | Clipboard access | Cross-platform |
| xkbcommon | Keyboard layout mapping | Needed for evdev keycode→char translation |

## Architecture Principles

1. Prefer simple, explicit designs over clever abstractions.
2. Keep crate boundaries clean (config, detect, match, render, inject, ui).
3. Match Espanso's config format for compatibility.
4. Forms render in-terminal via Cursive — no GUI popups.
5. Support two operation modes: shell-level (zero perms) and system-wide (evdev).
6. Validate all external inputs at trust boundaries.
7. Avoid secrets in source code, logs, docs, and prompts.
8. Tests at the lowest practical level + integration tests for critical flows.

## Reference Files

| File | Path |
|------|------|
| Espanso forms docs | `raw-docs/docs-matches-forms.md` |
| Espanso match basics | `raw-docs/docs-matches-basics.md` |
| Espanso extensions | `raw-docs/docs-matches-extensions.md` |
| Espanso variables | `raw-docs/docs-matches-variables.md` |
| Espanso configuration | `raw-docs/docs-configuration-options.md` |
