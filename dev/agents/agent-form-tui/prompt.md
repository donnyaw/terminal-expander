# Form TUI Agent

**Specialty**: Cursive form renderer
**Model**: `opencode-go/deepseek-v4-flash`

## Objective

Implement a terminal-based form renderer using Cursive that supports all Espanso form field types: text, multiline, choice dropdown, and list selection.

## Before Coding

1. Run `agent-librarian prompt-read.md` to inject relevant memory
2. Read `raw-docs/docs-matches-forms.md` for Espanso form syntax reference
3. Read `agents/planner/architecture.md` for the form extension API design

## Requirements

- `FormConfig` struct with `layout: String` and `fields: HashMap<String, FieldConfig>`
- `FieldType`: Text (single-line), TextArea (multiline), Choice (dropdown), List
- `FieldConfig`: `default`, `placeholder`, `values` (for choice/list), `multiline`
- Cursive widgets: `EditView` (text), `TextArea` (multiline), `SelectView::new().popup()` (choice), `ListView` (list)
- `Dialog` wrapper with Submit and Cancel buttons
- Tab key navigates between fields
- Ctrl+Enter submits, Esc cancels
- Returns `HashMap<String, String>` on submit, `None` on cancel

## Tasks

- [ ] PHASE-04: Text fields + multiline + Dialog wrapper
- [ ] PHASE-05: Choice dropdown + List selection

## Quality Criteria

- All form fields display simultaneously on one screen
- Field focus cycles correctly with Tab
- Default values pre-populate correctly
- Choice/list options are selectable via keyboard
- Form layout preserves Espanso's `[[field_name]]` syntax in layout string
