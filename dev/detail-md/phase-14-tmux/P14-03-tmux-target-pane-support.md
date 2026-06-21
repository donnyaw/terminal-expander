# P14-03: Add Tmux Target Pane Support

## Objective

Allow tmux output mode to inject into a specific tmux pane using a `--target-pane` option.

## Implementation Status

Completed on branch `feature/tmux-integration` with tag `p14-03`.

## Why This Exists

When a tmux popup or helper script runs, the current active pane may no longer be the user's original selected pane. The integration must explicitly target the pane that had focus before the popup opened.

## Scope

- Add `--target-pane` to `ce expand`.
- Pass the target to tmux `send-keys` using `-t`.
- Keep target optional.
- Do not implement popup picker yet.
- Preserve a clean API that later supports `--enter` and multiline strategy without rewriting this work.

## Proposed CLI

```bash
ce expand ":hello" --output tmux --target-pane "%1"
ce expand ":hello" --output tmux --target-pane "$TMUX_PANE"
```

## Implementation Steps

1. Add `target_pane: Option<String>` to `Commands::Expand`.

2. Extend tmux injection around an options struct rather than adding many ad hoc parameters:

```rust
struct TmuxInjectOptions {
    target_pane: Option<String>,
    enter: bool,
}
```

For this task, `enter` stays false and is reserved for `P14-09`.

3. Extend `TmuxInjector` to support an optional target. Minimal options:

- Add a new constructor or struct field.
- Add a separate `inject_to_target` method.
- Add a new `TmuxTargetInjector` if that keeps the trait simple.

4. Construct tmux command as:

```bash
tmux send-keys -t <target> -l <text>
```

5. When no target is provided, preserve the current behavior:

```bash
tmux send-keys -l <text>
```

6. Validate only enough to avoid empty target values. Let tmux validate actual pane existence.

7. Include the target pane value in error context when tmux fails.

## Acceptance Criteria

- `--target-pane` is accepted with `--output tmux`.
- If a target is provided, tmux command includes `-t`.
- If target is omitted, existing tmux behavior remains.
- The option does not affect stdout or clipboard output.
- Empty `--target-pane ""` fails before invoking tmux.

## Test Plan

- Unit-test command construction if process invocation is abstracted.
- Manual-test with two panes and a known pane id.
- Confirm pane A receives text when pane B is active only if pane A was passed as target.

## Dependencies

- `P14-02`.

## Follow-Up

`P14-07` will depend on this to preserve the original pane across popup execution.
