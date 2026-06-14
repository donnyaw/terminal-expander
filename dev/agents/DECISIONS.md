# Architectural Decisions

## ADR-001: Cursive over Ratatui for form rendering
**Date**: 2026-06-14
**Status**: Proposed
**Context**: Need a Rust TUI framework for multi-field forms.
**Decision**: Cursive has built-in EditView, TextArea, SelectView.popup(), Dialog, and Button — all form widgets needed. Ratatui needs third-party crates for text input and manual focus management.
**Consequences**: Smaller dependency tree, faster development. Less popular ecosystem (4.8k vs 21k stars).

## ADR-002: serde_norway over serde_yaml for config parsing
**Date**: 2026-06-14
**Status**: Proposed
**Context**: Espanso config uses multi-line YAML (`form: |` syntax) that serde_yaml handles poorly.
**Decision**: Use serde_norway for Espanso-compatible YAML parsing.
**Consequences**: Compatible with existing Espanso match files. One extra dependency.

## ADR-003: evdev + uinput for system-wide operation
**Date**: 2026-06-14
**Status**: Proposed
**Context**: Need system-wide keyboard detection and text injection on Linux.
**Decision**: Use evdev crate for detection (read /dev/input/event*) and uinput for injection (virtual keyboard device). This works on Wayland, X11, and TTY.
**Consequences**: Requires `input` group for detection and `uinput` group or CAP_DAC_OVERRIDE for injection. Users without these permissions fall back to shell-only mode.
