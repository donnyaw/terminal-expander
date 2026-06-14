# Rust Core Agent

**Specialty**: Match engine, config, render pipeline
**Model**: `opencode-go/deepseek-v4-flash`

## Objective

Implement the core engine: YAML config parser, match detection engine, template renderer, and variable system.

## Before Coding

1. Run `agent-librarian prompt-read.md` to inject relevant memory
2. Read `raw-docs/docs-matches-basics.md` and `raw-docs/docs-matches-variables.md`

## Requirements

### Config Parser (PHASE-02)
- Parse Espanso-compatible YAML match files using `serde_norway`
- Support `trigger:`, `triggers:`, `replace:`, `form:`, `form_fields:`, `vars:`
- Support multi-line YAML (`|` and `>`)
- File watching via `notify` crate for hot-reload

### Match Engine (PHASE-03)
- Rolling key buffer for trigger detection
- Prefix matching (`:trigger`), word boundary detection
- Match disambiguation (multiple matches for same trigger)
- Search labels for form filtering
- App-specific include/exclude rules

### Template Renderer (PHASE-06)
- `{{field_name}}` substitution in replace text
- `$|$` cursor position marker
- Variable types: `clipboard`, `date`, `form`, `shell`, `match`
- Variable chaining (clipboard -> form default -> output)
- Form extension: `type: form` with `layout` and `fields` params

## Tasks

- [ ] PHASE-00: Initialize workspace and crate scaffolding
- [ ] PHASE-02: YAML config parser
- [ ] PHASE-03: Match engine
- [ ] PHASE-06: Template renderer + variable system

## Quality Criteria

- Parses existing Espanso `.yml` match files without changes
- Trigger detection is sub-second latency
- Form extension returns correct `HashMap<String, String>`
- Variable substitution works for all types: clipboard, date, shell, form, match
