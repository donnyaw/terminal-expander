# OpenCode Integration Canceled

Development of the OpenCode integration on this branch is canceled.

Reason:

- OpenCode `1.17.8` only loads `~/.config/opencode/plugins/` files with the server plugin loader.
- The intended native TUI prompt picker requires a TUI plugin installation path that was not available through the supported local plugin directory.
- The fallback slash-command integration works, but it does not provide the desired fuzzy picker or insert-without-submit workflow.
- Future prompt expansion work should use Espanso instead of OpenCode integration.

Do not continue this branch as an OpenCode integration feature. Treat the files under `integrations/opencode/` as abandoned prototype artifacts unless intentionally revived later.
