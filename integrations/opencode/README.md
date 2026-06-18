# OpenCode Prompt Picker

This integration uses cli-expander as an AI prompt registry and adds an
OpenCode-native prompt picker.

## What It Does

- Loads prompt records from `ce list --json`.
- Filters records in the `ai-prompts` category or source path.
- Opens an OpenCode fuzzy picker with `Ctrl+F`.
- Expands the selected trigger with `ce expand <trigger>`.
- Inserts the expanded prompt into the OpenCode input box without submitting it.

The plugin does not run raw `fzf` or cli-expander's Cursive forms inside
OpenCode. It uses OpenCode's native TUI dialogs so the interaction stays inside
OpenCode.

## Install Sample Prompts

```bash
mkdir -p ~/.config/cli-expander/matches/ai-prompts
cp examples/ai-prompts/*.yml ~/.config/cli-expander/matches/ai-prompts/
ce list --json
```

The parent folder name becomes the prompt category: `ai-prompts`.

## Install The OpenCode Plugin

Copy or symlink the plugin into OpenCode's global plugin directory:

```bash
mkdir -p ~/.config/opencode/plugins
cp integrations/opencode/cli-expander-prompts.ts ~/.config/opencode/plugins/
```

Restart OpenCode after installing or changing plugins. OpenCode loads plugin
files at startup.

## Usage

Inside OpenCode:

- Press `Ctrl+F` to open the cli-expander prompt picker.
- Or run `/prompt`, `/prompts`, or `/ce-prompt` if slash command registration is available.
- Search by trigger or description.
- Select a prompt.
- Edit the inserted prompt if needed, then submit normally.

## Prompt Format

Store AI prompts as normal cli-expander matches:

```yaml
matches:
  - trigger: ":ai-review"
    search_label: "Review code for bugs, risks, regressions, and missing tests"
    replace: |
      Review the current code change with a code-review mindset.

      Return findings first, ordered by severity, with file and line references.
```

Recommended conventions:

- Use `:ai-*` triggers for AI prompts.
- Use `search_label` as the picker description.
- Store prompt packs under `matches/ai-prompts/`.
- Keep shell command builders outside `ai-prompts/` so the OpenCode picker only shows prompts.

## Configuration

The plugin supports optional OpenCode plugin options when loaded from
`opencode.json`:

```json
{
  "plugin": [
    [
      "./integrations/opencode/cli-expander-prompts.ts",
      {
        "cePath": "/home/rezriz/.local/bin/ce",
        "category": "ai-prompts",
        "sourcePathPart": "/ai-prompts/"
      }
    ]
  ]
}
```

When installed from `~/.config/opencode/plugins/`, the defaults are usually
enough. You can also set `CLI_EXPANDER_BIN` to override the `ce` binary path.

## Current Limitations

- The MVP handles text prompts only.
- Existing cli-expander Cursive forms are intentionally not embedded in OpenCode.
- Prompt variables such as `{{task}}` are not prompted for yet; they remain in
  the inserted text for manual editing.
- `Ctrl+F` may conflict with an OpenCode or terminal shortcut. If that happens,
  change the plugin keybind or use the slash command.
