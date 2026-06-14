# Code Conventions

*This file is maintained by agent-librarian. Append new conventions as they are established.*

## Rust

- Use `anyhow::Result` for fallible functions in binary crate
- Use custom error types in library crates
- All public types implement `Debug` and `Clone`
- Variables use `snake_case`, types use `PascalCase`
- Unsafe code requires `// SAFETY:` comment on the same line
- Modules are documented with `//!` doc comments

## Form Config

- Field names use `snake_case` (matching Espanso convention)
- Form layout uses `[[field_name]]` syntax (matching Espanso)
- All form field configs use the same schema as Espanso `form_fields`
