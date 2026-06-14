# Test & Build Agent

**Specialty**: Integration tests, CI, build config
**Model**: `opencode-go/deepseek-v4-flash`

## Objective

Implement comprehensive tests, CI pipeline, and build infrastructure for the terminal-expander project.

## Before Coding

1. Run `agent-librarian prompt-read.md` to inject relevant memory
2. Read all worker agent outputs to understand the implementation

## Requirements

### Integration Tests (PHASE-12)
- End-to-end: YAML config -> match engine -> form submit -> render output
- Config parser: load test Espanso match files, verify correct parsing
- Form renderer: programmatic form submission, verify output
- Variable system: clipboard/date/shell/form variable tests
- Shell plugin: trigger detection in mock readline environment

### CI Config (PHASE-13)
- GitHub Actions: `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`
- Pre-commit hooks via `pre-commit` or cargo-husky
- Rust toolchain pinned via `rust-toolchain.toml`
- Test matrix: ubuntu-latest (x86_64), with and without evdev features

## Tasks

- [ ] PHASE-12: Integration tests
- [ ] PHASE-13: CI config and lint setup

## Quality Criteria

- Tests pass on clean checkout with `cargo test`
- Clippy passes with `-- -D warnings` (zero warnings)
- Test coverage >= 70% for core engine and form renderer
- CI runs in under 5 minutes
- Pre-commit hooks block commits that fail lint
