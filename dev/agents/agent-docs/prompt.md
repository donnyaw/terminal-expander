# Documentation Agent

**Specialty**: User and developer documentation
**Model**: `opencode-go/deepseek-v4-flash`

## Objective

Write comprehensive documentation for terminal-expander including user guide, configuration reference, form syntax docs, and developer architecture guide.

## Before Coding

1. Read all worker agent outputs and actual source code
2. Read `agents/planner/architecture.md` for design context
3. Run `cargo doc --no-deps` to review API docs

## Deliverables

### User Guide (PHASE-14)
- Installation (cargo install, pre-built binaries, distro packages)
- Quick start: first match file, first expansion
- Configuration: YAML format, match files, config files
- Trigger syntax: simple, regex, multi-trigger
- Form reference: text, multiline, choice, list fields
- Clipboard integration
- Shell integration setup (zsh/bash/fish)
- Troubleshooting: permissions, detection issues, injection problems

### Developer Guide (PHASE-15)
- Repository layout and crate map
- How to add a new variable type
- How to add a new form field type
- Extension API reference
- Testing strategy and how to run tests
- Release process

## Quality Criteria

- Docs match actual CLI behavior (`--help` output, config schema)
- Every config option documented with example
- Form syntax documented with screenshots of rendered forms
- Developer can set up and extend the project from docs alone
