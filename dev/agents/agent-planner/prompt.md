# Planner Agent

**Task ID**: PHASE-01
**Model**: `openai/gpt-5.5`
**Dependencies**: None

## Objective

Design the terminal-expander system architecture, crate decomposition, and trait interfaces. Produce a concrete plan ready for implementation agents.

## Required Inputs

- `agents/CONTEXT.md`
- `agents/AGENTS.md`

## Tasks

1. Define crate boundaries and responsibilities
2. Design key traits: `ConfigProvider`, `MatchDetector`, `FormRenderer`, `TextInjector`, `KeySource`
3. Design the form extension type system (parsing `form_fields` -> `FieldConfig`)
4. Document the template variable injection pipeline
5. Define CLI command structure (subcommands: run, configure, list, etc.)
6. Identify all dependencies (cargo crates) needed

## Deliverables

- `agents/planner/architecture.md` - crate map, trait interfaces, data flow
- `agents/planner/api-spec.md` - CLI commands, config schema, form YAML spec
- Update `tasks.csv` PHASE-01 when complete

## Quality Criteria

- Crate boundaries are clean with minimal cross-crate coupling
- Form extension API matches Espanso's semantics (text, choice, list fields)
- Template variable system supports: `{{form.field}}`, `{{clipboard}}`, `{{date:format}}`, `{{shell:cmd}}`
- Espanso config files (`match/*.yml`) parse without changes
