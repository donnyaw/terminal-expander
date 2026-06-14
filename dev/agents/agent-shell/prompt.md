# Shell Integration Agent

**Specialty**: zle hooks, bash bindings, fish abbr
**Model**: `opencode-go/deepseek-v4-flash`

## Objective

Implement shell-level text expansion hooks for zsh, bash, and fish. This provides zero-permission text expansion that works entirely within the shell prompt.

## Before Coding

1. Run `agent-librarian prompt-read.md` to inject relevant memory
2. Read `agents/planner/architecture.md` for CLI command interface

## Requirements

### zsh Widget (PHASE-09)
- Override `self-insert` widget via `zle -N`
- Accumulate typed characters in a buffer
- When buffer matches a trigger + word separator, call `terminal-expander expand`
- Replace `BUFFER` and position `CURSOR` with expanded text
- Compatible with zsh-autosuggestions and zsh-syntax-highlighting (chain properly)
- Install via `source` script in `.zshrc`

### bash Bindings (PHASE-10)
- `bind -x` for key sequences bound to expansion check
- `READLINE_LINE` and `READLINE_POINT` manipulation
- Space/Enter as expansion triggers
- Install via `source` script in `.bashrc`

### fish Function (PHASE-11)
- Native `abbr` command for simple expansions
- Custom fish function for form-based expansions
- Install via `conf.d/` snippets

## Tasks

- [ ] PHASE-09: zle widget
- [ ] PHASE-10: bash readline bindings
- [ ] PHASE-11: fish shell integration

## Quality Criteria

- Shell-native expansion works without any external daemon
- Does not break other shell plugins
- Trigger detection in shell is instantaneous
- Forms work in shell context (spawn Cursive TUI, return to shell on submit)
- Install scripts are idempotent
