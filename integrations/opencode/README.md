# OpenCode Prompt Commands

> Status: canceled. This OpenCode integration branch is no longer being developed.
> Future prompt expansion work should use Espanso instead.

This integration uses cli-expander as an AI prompt registry for OpenCode.

## What It Does

- Adds OpenCode slash commands: `/prompt`, `/prompts`, and `/ce-prompt`.
- Expands a selected trigger with `ce expand <trigger>`.
- Lists available triggers with `ce list` when no trigger is provided.

OpenCode `1.17.8` loads files in `~/.config/opencode/plugins/` with the server
plugin loader. That loader rejects TUI-only prompt picker modules, so the
supported integration path is OpenCode command files.

## Install Sample Prompts

```bash
mkdir -p ~/.config/cli-expander/matches/ai-prompts
cp examples/ai-prompts/*.yml ~/.config/cli-expander/matches/ai-prompts/
ce list --json
```

The parent folder name becomes the prompt category: `ai-prompts`.

## Install The OpenCode Commands

Copy the command files into OpenCode's global command directory:

```bash
mkdir -p ~/.config/opencode/commands
cp integrations/opencode/commands/*.md ~/.config/opencode/commands/
```

Restart OpenCode after installing or changing commands. OpenCode loads command
files at startup.

## Usage

Inside OpenCode:

- Run `/prompts` to list available triggers.
- Run `/prompt :ai-review` to expand and submit the `:ai-review` prompt.
- Run `/ce-prompt :ai-plan` as an equivalent explicit cli-expander command.

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

## Experimental TUI Picker

`cli-expander-prompts.ts` is an experimental OpenCode TUI plugin that uses native
dialogs and appends the expanded prompt without auto-submitting. Do not copy it
into `~/.config/opencode/plugins/` on OpenCode `1.17.8`; that path is for server
plugins and will reject TUI-only modules.

## Current Limitations

- The MVP handles text prompts only.
- Existing cli-expander Cursive forms are intentionally not embedded in OpenCode.
- Prompt variables such as `{{task}}` are not prompted for yet; they remain in
  the command output for manual editing in cli-expander.
- `Ctrl+F` is already used by OpenCode for input movement and other TUI actions.
  A fuzzy prompt picker needs a supported OpenCode TUI plugin installation path.
