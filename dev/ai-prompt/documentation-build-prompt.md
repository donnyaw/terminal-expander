# AI Prompt: Build Product Documentation for cli-expander

## Role

You are a technical documentation specialist. Create clear, user-facing and developer-facing documentation for `cli-expander`, a terminal-native trigger expander and TUI command form builder written in Rust.

## Repository

Work in:

```text
/home/rezriz/github/cli-projects/cli-expander
```

Primary branch/repo:

```text
donnyaw/cli-expander on main
```

Only the `cli-expander` repo is active. The previous `texpand` repo has been archived.

## Product Positioning

Document the product as:

```text
Forms for shell commands, launched from the prompt.
```

More specifically:

```text
cli-expander lets users type short triggers like :findname, press Space, fill a terminal TUI form, and insert the generated command directly into the current shell prompt for review/editing before execution.
```

Closest tools to compare against:

- `navi`: searchable cheatsheets
- `pet`: command snippet manager
- `espanso`: general text expander
- `gum`: CLI form primitive

Key differentiator:

```text
:trigger[Space] -> TUI form -> generated result appears in current shell prompt
```

## Important Workflow Rule

The official primary workflow is:

```text
:trigger[Space]
```

Examples:

```text
:find[Space]
:findmenu[Space]
:findname[Space]
:formdemo[Space]
:cascade[Space]
```

Avoid documenting `:trigger[Enter]` as the main workflow. Enter executes the typed trigger as a shell command. The Bash plugin now warns users to use Space instead.

Correct explanation:

```text
Use Space to expand into the editable prompt. Press Enter only after the generated command is visible and you are ready to run it.
```

## Current Working Features

Document these as working or available features:

1. Plain text expansion
2. Multi-trigger aliases
3. Multi-line expansion
4. Date variables
5. Clipboard variables
6. Shell command variables
7. Template rendering with `{{variable}}`
8. Direct CLI expansion with `ce :trigger`
9. Bash prompt expansion with `:trigger[Space]`
10. TUI forms rendered with Cursive
11. Clean stdout form output for command capture
12. Cascading/dependent dropdowns
13. Config loading from `~/.config/cli-expander/matches/*.yml`
14. Example command-builder packs for Linux `find`
15. Searchable dropdowns with `/` keybinding (substring filter)
16. Mixed text-after-dropdown form layouts

## Current Form Field Types

Document all supported field types:

### Text

Single-line input.

```yaml
path:
  default: "."
  placeholder: "/home/user"
```

### Choice

Single-select dropdown for short option sets.

```yaml
type:
  type: choice
  values:
    - "-type f"
    - "-type d"
    - "-type l"
```

### List

Single-select list for longer/file-like option sets. In the TUI it displays with `- item`, but returns the raw value.

```yaml
file:
  type: list
  values:
    - report.pdf
    - notes.md
    - script.sh
```

### Multiline

Text area input.

```yaml
notes:
  multiline: true
  default: "line one\nline two"
```

### Checkbox

Boolean input. Returns `true` or `false`.

```yaml
confirm:
  type: checkbox
  default: "true"
```

### Password

Hidden/masked single-line input.

```yaml
secret:
  type: password
  placeholder: "hidden input"
```

### Cascading Choice

A child dropdown whose options depend on a parent dropdown.

```yaml
category:
  type: choice
  values:
    - Fruits
    - Animals

item:
  type: choice
  depends_on: category
  values:
    Fruits:
      - Apple
      - Banana
    Animals:
      - Cat
      - Dog
```

## Current Test Triggers

Document these useful test triggers. The user has tested them successfully.

General:

```text
:hello
:date
:greet
:choose
:name
:note
:priority
:ticket
:meeting
:config
:review
:select
:profile
:quick
:emoji
```

Linux find command builders:

```text
:find
:findx
:findrm
:findexec
:findsize
:findmtime
:findgrep
:findmenu
:findname
```

Field/system demos:

```text
:formdemo
:cascade
:cascade2
:demo-text
:demo-alias
:demo-alt
:demo-multiline
:demo-date
:demo-time
:demo-shell
:demo-user
:demo-clipboard
:demo-template
:demo-form
```

Recommended demo set:

```text
:hello[Space]
:demo-date[Space]
:demo-template[Space]
:formdemo[Space]
:find[Space]
:findmenu[Space]
:findname[Space]
:cascade[Space]
:demo-form[Space]
```

## Documentation Tasks

Create or update documentation with a practical user-first structure.

Recommended files:

```text
README.md
docs/getting-started.md
docs/shell-workflow.md
docs/forms.md
docs/examples-find.md
docs/configuration.md
docs/developer-notes.md
```

If existing docs already cover these topics, update them instead of duplicating. Keep docs consistent and avoid conflicting instructions.

## README Requirements

The root `README.md` should clearly explain:

1. What the product is
2. Why it exists
3. The primary `:trigger[Space]` workflow
4. How it differs from `navi`, `pet`, and `espanso`
5. Installation/build basics
6. Bash plugin setup
7. A minimal text expansion example
8. A minimal form example
9. A `find` command-builder example
10. A list of supported field types
11. How to list triggers with `ce list`

Keep README concise. Move deeper explanations into `docs/`.

## Getting Started Requirements

Include a copy-pasteable quick start:

```bash
cargo build --release
cp target/release/ce ~/.local/bin/ce
source shell/cli-expander.bash
```

Then show:

```text
:hello[Space]
:findname[Space]
```

Explain:

```text
Use Space to expand. Press Enter only after the generated command appears on the prompt.
```

## Forms Documentation Requirements

Document every form field type with YAML examples:

- text
- choice
- list
- multiline
- checkbox
- password
- cascading choice with `depends_on`

Explain that `list` currently returns a single selected value, visually displayed with `- item` in the TUI.

## Examples Documentation Requirements

Create a `find` examples guide showing:

```text
:find[Space]
:findmenu[Space]
:findname[Space]
```

Show generated examples:

```bash
find . -type f -name '*.log' -print
find . -type f -size +100M -print
find . -type d -name '*.rs' -exec ls -lh {} \;
```

Add a safety note for destructive actions:

```text
Review generated commands before pressing Enter, especially commands using -delete, rm, chmod -R, or other destructive operations.
```

## Configuration Documentation Requirements

Document:

- Config directory: `~/.config/cli-expander/matches/`
- Match files use YAML
- Top-level structure: `matches:`
- Basic `trigger` and `replace`
- `triggers` aliases
- `vars`
- `type: form`
- `params.layout`
- `params.fields`

## Developer Notes Requirements

Briefly document the crate layout:

```text
cli-expander-cli      CLI and field config mapping
cli-expander-config   YAML parsing
cli-expander-match    trigger matching
cli-expander-render   template and variable rendering
cli-expander-ui       Cursive TUI forms
cli-expander-detect   Linux keyboard detection
cli-expander-inject   text injection backends
shell/           Bash/Zsh/Fish plugins
```

Mention that the current focus is Bash and the Cursive TUI version.

## Verification

After editing docs, run:

```bash
ce list
```

If code was changed, also run:

```bash
cargo test
```

For docs-only changes, no build is required.

## Style Requirements

- Use concise, practical language.
- Prefer examples over theory.
- Avoid overpromising untested Zsh/Fish behavior.
- Do not present `:trigger[Enter]` as a recommended workflow.
- Keep destructive command examples clearly marked as requiring review before execution.
- Keep docs beginner-friendly but accurate for developers.

## Deliverable

Return a summary of:

- Files created/updated
- Main documentation sections added
- Any assumptions or known gaps
- Verification performed
