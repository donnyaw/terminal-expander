# terminal-expander — Build Plan

**Goal**: A terminal-native Rust text expander with Espanso-compatible form support (multi-field forms with text, multiline, choice/list dropdowns — all rendered in-terminal via Cursive).

**Repo**: https://github.com/donnyaw/terminal-expander
**Total cost**: ~$0.53 (18 tasks)
**Timeline**: ~7 days

---

## Phase 0: Project Setup (1 task, $0.01)

### PHASE-00 — Initialize Rust workspace
| Field | Value |
|-------|-------|
| Agent | agent-rust-core |
| Model | opencode-go/deepseek-v4-flash |
| Status | pending |
| Depends | — |

**Tasks:**
- `cargo init --lib` in project root
- Set up workspace `Cargo.toml` with crate stubs:
  - `texpand-config` — YAML parsing
  - `texpand-detect` — evdev key detection
  - `texpand-match` — trigger matching
  - `texpand-render` — template rendering + form extension
  - `texpand-inject` — text injection
  - `texpand-ui` — Cursive form renderer
  - `texpand-cli` — main binary
- Add workspace deps: `clap`, `serde`, `serde_norway`, `anyhow`, `cursive`, `evdev`
- Configure `rust-toolchain.toml`
- Add `.gitignore`
- Verify `cargo build` passes

---

## Phase 1: Architecture (2 tasks, $0.30)

### PHASE-01 — System architecture & crate decomposition
| Field | Value |
|-------|-------|
| Agent | agent-planner |
| Model | **openai/gpt-5.5** |
| Status | pending |
| Depends | — |

**Deliverables:**
- `agents/planner/architecture.md` — crate map, trait interfaces, data flow
- `agents/planner/api-spec.md` — CLI commands, config schema, form YAML spec

**Design decisions to make:**
- Crate boundaries and responsibilities
- Key traits: `ConfigProvider`, `MatchDetector`, `FormRenderer`, `TextInjector`, `KeySource`
- Form extension type system (parsing `form_fields` → `FieldConfig`)
- Template variable injection pipeline
- CLI command structure (subcommands: `run`, `configure`, `list`, etc.)

### PHASE-01-qc — QC: Architecture plan
| Field | Value |
|-------|-------|
| Agent | agent-qc |
| Model | **openai/gpt-5.5** |
| Status | pending |
| Depends | PHASE-01 |

**Criteria:**
- Score >= 75 required before implementation starts
- Crate boundaries are clean with minimal cross-crate coupling
- Form extension API matches Espanso's semantics (text, choice, list fields)
- Template variable system supports: `{{form.field}}`, `{{clipboard}}`, `{{date:format}}`, `{{shell:cmd}}`
- Espanso config files (`match/*.yml`) parse without changes

---

## Phase 2: Core Engine (4 tasks, $0.10)

### PHASE-02 — YAML config parser (Espanso-compatible)
| Field | Value |
|-------|-------|
| Agent | agent-rust-core |
| Model | opencode-go/deepseek-v4-flash |
| Status | pending |
| Depends | PHASE-01-qc |

**Requirements:**
- Parse Espanso-compatible YAML match files using `serde_norway`
- Support: `trigger`, `triggers`, `replace`, `form:`, `form_fields:`, `vars:`
- Support multi-line YAML (`|` and `>`)
- File watching via `notify` crate for hot-reload
- Search labels for form filtering
- App-specific include/exclude rules

**QC (PHASE-02-qc):**
- Must parse existing Espanso match files correctly (from `raw-docs/` test corpus)
- Hot-reload detects file changes within 1 second

---

### PHASE-03 — Match engine (trigger detection + rolling buffer)
| Field | Value |
|-------|-------|
| Agent | agent-rust-core |
| Model | opencode-go/deepseek-v4-flash |
| Status | pending |
| Depends | PHASE-02-qc |

**Requirements:**
- Rolling key buffer for trigger detection
- Prefix matching (`:trigger`), word boundary detection
- Match disambiguation (multiple matches for same trigger)
- Search labels for form filtering

**QC (PHASE-03-qc):**
- Trigger detection works with Espanso-style `:prefix` triggers
- Sub-second detection latency
- Match disambiguation presents all options correctly

---

### PHASE-06 — Template renderer + variable system
| Field | Value |
|-------|-------|
| Agent | agent-rust-core |
| Model | opencode-go/deepseek-v4-flash |
| Status | pending |
| Depends | PHASE-03-qc, PHASE-05-qc |

**Requirements:**
- `{{field_name}}` substitution in replace text
- `$|$` cursor position marker
- Variable types: `clipboard`, `date`, `form`, `shell`, `match`
- Variable chaining (clipboard → form default → output)
- Form extension: `type: form` with `layout` and `fields` params
- Nested matches (`{{output}}` → `:explore-one`)

**QC (PHASE-06-qc):**
- Variable injection chain works: clipboard → shell → form → output
- Form extension returns correct `HashMap<String, String>`
- Date variables support format strings and offsets
- Shell variables execute and capture stdout

---

## Phase 3: TUI Form Renderer (2 tasks, $0.04)

### PHASE-04 — Cursive form: text fields + multiline
| Field | Value |
|-------|-------|
| Agent | agent-form-tui |
| Model | opencode-go/deepseek-v4-flash |
| Status | pending |
| Depends | PHASE-01-qc |

**Requirements:**
- Cursive crate as TUI framework
- `FormConfig` struct with `layout: String` and `fields: HashMap<String, FieldConfig>`
- `FieldType`: Text (single-line), TextArea (multiline)
- `FieldConfig`: `default`, `placeholder`, `multiline`
- Cursive widgets: `EditView` (text), `TextArea` (multiline)
- `Dialog` wrapper with Submit and Cancel buttons
- Tab key navigates between fields
- Ctrl+Enter submits, Esc cancels
- Returns `HashMap<String, String>` on submit, `None` on cancel

**QC (PHASE-04-qc):**
- Multi-field form displays with labels, defaults, placeholders
- Field focus cycles correctly with Tab
- Default values pre-populate correctly
- Form layout preserves Espanso's `[[field_name]]` syntax

---

### PHASE-05 — Cursive form: choice/list dropdowns
| Field | Value |
|-------|-------|
| Agent | agent-form-tui |
| Model | opencode-go/deepseek-v4-flash |
| Status | pending |
| Depends | PHASE-04-qc |

**Requirements:**
- `SelectView::new().popup()` for choice dropdown
- `ListView` for multi-select list
- Dynamic value population from shell command output
- `FieldType`: Choice (single-select dropdown), List (multi-select)

**QC (PHASE-05-qc):**
- Choice and list fields render and return correct values
- Options are selectable via keyboard navigation
- Dynamic population from variables works

---

## Phase 4: Platform Layer (2 tasks, $0.04)

### PHASE-07 — evdev keyboard detection
| Field | Value |
|-------|-------|
| Agent | agent-platform |
| Model | opencode-go/deepseek-v4-flash |
| Status | pending |
| Depends | PHASE-01-qc |

**Requirements:**
- Open `/dev/input/event*` using `evdev` crate
- Map raw keycodes to characters using `xkbcommon` (keyboard layout aware)
- Track currently focused window (Wayland: `wlr-data-control`, TTY: heuristics)
- Filter self-generated events (from uinput injection) to avoid echo loops
- Fallback chain: check `$DISPLAY` for X11, `$WAYLAND_DISPLAY` for Wayland
- Permission: `input` group for `/dev/input/event*`
- Clear user-facing error messages for missing permissions

**QC (PHASE-07-qc):**
- Works on Wayland, X11, and TTY
- Self-filtering prevents echo loops
- Permission errors produce actionable messages

---

### PHASE-08 — uinput text injection
| Field | Value |
|-------|-------|
| Agent | agent-platform |
| Model | opencode-go/deepseek-v4-flash |
| Status | pending |
| Depends | PHASE-07-qc |

**Requirements:**
- Create virtual keyboard via `/dev/uinput` using `evdev::VirtualDeviceBuilder`
- Translate characters to evdev keycodes + modifier states via xkbcommon
- Timing: 5ms after key press, 12ms after key release, 20ms after modifier release (from srkt project)
- Name the device `"terminal-expander"` for self-filtering
- Permission: `uinput` group or `CAP_DAC_OVERRIDE`
- Clipboard fallback injection for when uinput is unavailable

**QC (PHASE-08-qc):**
- Text injected without modifier bleed (no `HELLO` when `hello` expected)
- Permission errors produce actionable messages
- Clipboard fallback works without uinput

---

## Phase 5: Shell Integration (3 tasks, $0.06)

### PHASE-09 — zle widget (zsh abbreviation hooks)
| Field | Value |
|-------|-------|
| Agent | agent-shell |
| Model | opencode-go/deepseek-v4-flash |
| Status | pending |
| Depends | PHASE-06-qc |

**Requirements:**
- Override `self-insert` widget via `zle -N`
- Accumulate typed characters in buffer
- When buffer matches trigger + word separator, call `terminal-expander expand`
- Replace `BUFFER` and position `CURSOR` with expanded text
- Compatible with zsh-autosuggestions and zsh-syntax-highlighting
- Install via `source` script in `.zshrc`

**QC (PHASE-09-qc):**
- Works without breaking other zle plugins
- Trigger detection in zsh prompt is instantaneous

---

### PHASE-10 — bash readline bindings
| Field | Value |
|-------|-------|
| Agent | agent-shell |
| Model | opencode-go/deepseek-v4-flash |
| Status | pending |
| Depends | PHASE-06-qc |

**Requirements:**
- `bind -x` for key sequences bound to expansion check
- `READLINE_LINE` and `READLINE_POINT` manipulation
- Space/Enter as expansion triggers
- Install via `source` script in `.bashrc`

**QC (PHASE-10-qc):**
- Works in bash prompt
- Handles multi-line expansions

---

### PHASE-11 — fish shell abbreviation function
| Field | Value |
|-------|-------|
| Agent | agent-shell |
| Model | opencode-go/deepseek-v4-flash |
| Status | pending |
| Depends | PHASE-06-qc |

**Requirements:**
- Native `abbr` command for simple expansions
- Custom fish function for form-based expansions
- Install via `conf.d/` snippets

**QC (PHASE-11-qc):**
- Fish users can use forms without breaking shell

---

## Phase 6: Tests & CI (2 tasks, $0.04)

### PHASE-12 — Integration tests (end-to-end)
| Field | Value |
|-------|-------|
| Agent | agent-test-build |
| Model | opencode-go/deepseek-v4-flash |
| Status | pending |
| Depends | PHASE-06-qc, PHASE-08-qc, PHASE-09-qc, PHASE-10-qc, PHASE-11-qc |

**Requirements:**
- YAML config → match engine → form submit → render output
- Config parser: load test Espanso match files, verify correct parsing
- Form renderer: programmatic form submission, verify output
- Variable system: clipboard/date/shell/form variable tests
- Shell plugin: trigger detection in mock readline environment

**QC (PHASE-12-qc):**
- All critical paths tested
- Coverage >= 70%

---

### PHASE-13 — CI config and lint setup
| Field | Value |
|-------|-------|
| Agent | agent-test-build |
| Model | opencode-go/deepseek-v4-flash |
| Status | pending |
| Depends | PHASE-12-qc |

**Requirements:**
- GitHub Actions: `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`
- Pre-commit hooks via cargo-husky
- Rust toolchain pinned via `rust-toolchain.toml`
- Test matrix: ubuntu-latest (x86_64), with and without evdev features

**QC (PHASE-13-qc):**
- CI passes on clean repo
- Build completes in under 5 minutes

---

## Phase 7: Documentation (2 tasks, $0.04)

### PHASE-14 — User guide and configuration reference
| Field | Value |
|-------|-------|
| Agent | agent-docs |
| Model | opencode-go/deepseek-v4-flash |
| Status | pending |
| Depends | PHASE-13-qc |

**Deliverables:**
- Installation guide (cargo install, pre-built binaries, distro packages)
- Quick start: first match file, first expansion
- Configuration: YAML format, match files, config files
- Trigger syntax: simple, regex, multi-trigger
- Form reference: text, multiline, choice, list fields
- Shell integration setup (zsh/bash/fish)
- Troubleshooting: permissions, detection issues, injection problems

**QC (PHASE-14-qc):**
- Docs match actual CLI help output and config behavior

---

### PHASE-15 — Developer docs and architecture guide
| Field | Value |
|-------|-------|
| Agent | agent-docs |
| Model | opencode-go/deepseek-v4-flash |
| Status | pending |
| Depends | PHASE-13-qc |

**Deliverables:**
- Repository layout and crate map
- How to add a new variable type
- How to add a new form field type
- Extension API reference
- Testing strategy and how to run tests
- Release process

**QC (PHASE-15-qc):**
- Developer can set up and extend the project from docs alone

---

## Execution Guide

### Per-task sequence

Each worker task follows this sequence:
```
agent-librarian read  →  worker agent  →  agent-librarian write  →  QC review
```

### Run phases

```bash
# Setup
./run.sh PHASE-00

# Architecture (uses gpt-5.5, ~$0.30)
./run.sh PHASE-01

# Core engine (uses flash, ~$0.02 per phase)
./run.sh PHASE-02
./run.sh PHASE-03
./run.sh PHASE-06

# Form renderer (uses flash, ~$0.02 per phase)
./run.sh PHASE-04
./run.sh PHASE-05

# Platform layer (uses flash, ~$0.02 per phase)
./run.sh PHASE-07
./run.sh PHASE-08

# Shell integration (uses flash, ~$0.02 per phase)
./run.sh PHASE-09
./run.sh PHASE-10
./run.sh PHASE-11

# Tests + CI (uses flash, ~$0.02 per phase)
./run.sh PHASE-12
./run.sh PHASE-13

# Docs (uses flash, ~$0.02 per phase)
./run.sh PHASE-14
./run.sh PHASE-15
```

### Or run entire pipeline at once
```bash
./run.sh PHASE-00 PHASE-01 PHASE-02 PHASE-03 PHASE-04 PHASE-05 PHASE-06 PHASE-07 PHASE-08 PHASE-09 PHASE-10 PHASE-11 PHASE-12 PHASE-13 PHASE-14 PHASE-15
```

---

## Cost Summary

| Phase | Tasks | Worker Model | QC Model | Est. Total |
|-------|-------|-------------|----------|-----------|
| P0: Setup | 1 | flash | — | $0.01 |
| P1: Architecture | 1 + QC | gpt-5.5 | gpt-5.5 | $0.30 |
| P2: Core engine | 3 + 3 QC | flash | gpt-5.5 | $0.10 |
| P3: Form renderer | 2 + 2 QC | flash | gpt-5.5 | $0.04 |
| P4: Platform | 2 + 2 QC | flash | gpt-5.5 | $0.04 |
| P5: Shell | 3 + 3 QC | flash | gpt-5.5 | $0.06 |
| P6: Tests & CI | 2 + 2 QC | flash | gpt-5.5 | $0.04 |
| P7: Docs | 2 + 2 QC | flash | gpt-5.5 | $0.04 |
| **Total** | **16 + 15 QC** | **16 flash** | **15 gpt-5.5** | **~$0.53** |

## Agent Model Assignments

| Agent | Model | Cost (in/out per 1M) | Used for |
|-------|-------|---------------------|----------|
| agent-planner | openai/gpt-5.5 | $5 / $30 | Architecture + planning (1 run) |
| agent-qc | openai/gpt-5.5 | $5 / $30 | Quality review (15 runs) |
| All workers | opencode-go/deepseek-v4-flash | $0.14 / $0.28 | Implementation (16 runs) |
| agent-librarian | opencode-go/deepseek-v4-flash | $0.14 / $0.28 | Memory read/write (32 runs) |
