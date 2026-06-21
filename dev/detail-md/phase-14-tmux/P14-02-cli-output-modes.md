# P14-02: Add CLI Output Modes For Stdout Tmux Auto Clipboard

## Objective

Add a user-facing output selector to `ce expand` so expanded text can go to stdout, tmux, auto mode, or clipboard.

## Implementation Status

Completed on branch `feature/tmux-integration` with tag `p14-02`.

## Why This Exists

The shell plugins currently depend on stdout. Tmux needs direct pane injection. Clipboard is already implemented as a fallback option. A single `--output` flag makes output behavior explicit while keeping existing behavior safe.

## Scope

- Add `--output stdout|tmux|auto|clipboard` to the expand command.
- Keep `stdout` as the default.
- Use the existing `cli-expander-inject` crate.
- Do not add target pane handling yet. That belongs to `P14-03`.
- Keep output dispatch separate from expansion logic introduced in `P14-01`.
- Keep all shell plugin behavior backward compatible.

## Proposed CLI

```bash
ce expand ":hello" --output stdout
ce expand ":hello" --output tmux
ce expand ":hello" --output auto
ce expand ":hello" --output clipboard
```

Bare shorthand should also support the same global or command-level option if practical:

```bash
ce ":hello" --output tmux
```

## Implementation Steps

1. Add a Clap enum:

```rust
#[derive(clap::ValueEnum, Clone, Debug)]
enum OutputMode {
    Stdout,
    Tmux,
    Auto,
    Clipboard,
}
```

2. Add `output: OutputMode` to the `Expand` command with default `stdout`.

3. Add a small dispatch helper so output decisions are testable:

```rust
fn emit_output(text: &str, mode: OutputMode) -> anyhow::Result<()>
```

4. After `expand_input` returns text, dispatch output:

- `Stdout`: `println!`.
- `Tmux`: use `TmuxInjector`.
- `Clipboard`: use `ClipboardInjector`.
- `Auto`: use tmux when `$TMUX` is present; otherwise stdout.

5. Keep shell plugin behavior unchanged by default.

6. Validate unsupported flag combinations at the CLI boundary rather than inside individual injectors.

## Design Decisions

`stdout` remains the contract for shell plugins. Do not change `shell/cli-expander.bash`, `shell/cli-expander.zsh`, or `shell/cli-expander.fish` in this task.

`auto` should be conservative. In this phase it should select tmux only when `$TMUX` is set; otherwise it should behave like stdout. Do not attempt ydotool or global keyboard injection here.

`tmux` without a specific pane target is allowed in this task because `P14-03` owns pane targeting. The target defaults to tmux's current pane.

## Acceptance Criteria

- Existing shell plugins still work without changes.
- `ce expand :hello --output stdout` prints the expansion.
- `ce expand :hello --output clipboard` copies the expansion where clipboard is available.
- `ce expand :hello --output tmux` attempts tmux injection.
- `ce expand :hello --output auto` does not break outside tmux.
- Unsupported combinations return clear errors once later flags are added.

## Test Plan

- Unit-test enum parsing if separated.
- Integration-test stdout remains default.
- Test `auto` outside tmux falls back to stdout or the documented fallback.
- Manual-test tmux mode inside tmux.
- Manual-test auto mode outside tmux.

## Dependencies

- `P14-01`.

## Follow-Up

`P14-03` adds pane targeting so tmux output can address the original selected pane.
