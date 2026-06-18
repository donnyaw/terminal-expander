import type {
  TuiDialogSelectOption,
  TuiDialogStack,
  TuiPlugin,
} from "@opencode-ai/plugin/tui"
import { execFile } from "node:child_process"
import { promisify } from "node:util"

const execFileAsync = promisify(execFile)

type PromptRecord = {
  trigger: string
  description: string
  category: string
  type: string
  source_file: string
}

type PluginOptions = {
  cePath?: string
  category?: string
  sourcePathPart?: string
}

const DEFAULT_CATEGORY = "ai-prompts"
const DEFAULT_SOURCE_PART = "/ai-prompts/"

export const tui: TuiPlugin = async (api, options) => {
  const pluginOptions = (options ?? {}) as PluginOptions
  const cePath = pluginOptions.cePath || process.env.CLI_EXPANDER_BIN || "ce"
  const category = pluginOptions.category || DEFAULT_CATEGORY
  const sourcePathPart = pluginOptions.sourcePathPart || DEFAULT_SOURCE_PART

  async function runCe(args: string[]): Promise<string> {
    const { stdout } = await execFileAsync(cePath, args, {
      maxBuffer: 1024 * 1024 * 8,
    })
    return stdout.trimEnd()
  }

  async function loadPrompts(): Promise<PromptRecord[]> {
    const output = await runCe(["list", "--json"])
    const records = JSON.parse(output) as PromptRecord[]
    return records
      .filter((record) => {
        return (
          record.category === category ||
          record.source_file.includes(sourcePathPart)
        )
      })
      .sort((a, b) => a.trigger.localeCompare(b.trigger))
  }

  async function insertPrompt(record: PromptRecord): Promise<void> {
    try {
      const prompt = await runCe(["expand", record.trigger])
      await api.client.tui.appendPrompt({ text: prompt })
      api.ui.toast({
        variant: "success",
        title: "Prompt inserted",
        message: record.trigger,
      })
    } catch (error) {
      api.ui.toast({
        variant: "error",
        title: "Prompt expansion failed",
        message: error instanceof Error ? error.message : String(error),
      })
    }
  }

  function showPromptPicker(dialog?: TuiDialogStack): void {
    const stack = dialog ?? api.ui.dialog

    void loadPrompts()
      .then((records) => {
        if (records.length === 0) {
          api.ui.toast({
            variant: "warning",
            title: "No cli-expander prompts",
            message: `No records found for category '${category}'`,
          })
          return
        }

        const options: TuiDialogSelectOption<PromptRecord>[] = records.map(
          (record) => ({
            title: record.trigger,
            value: record,
            description: `${record.description || "No description"} (${record.category}, ${record.type})`,
            footer: record.source_file,
          }),
        )

        stack.replace(() =>
          api.ui.DialogSelect<PromptRecord>({
            title: "CLI Expander AI Prompts",
            placeholder: "Search prompts by trigger or description",
            options,
            onSelect: (option) => {
              stack.clear()
              void insertPrompt(option.value)
            },
          }),
        )
      })
      .catch((error) => {
        api.ui.toast({
          variant: "error",
          title: "Prompt picker failed",
          message: error instanceof Error ? error.message : String(error),
        })
      })
  }

  if (api.command) {
    api.command.register(() => [
      {
        title: "CLI Expander Prompts",
        value: "cli-expander.prompts",
        description: "Search cli-expander AI prompts and insert the selected prompt",
        category: "cli-expander",
        keybind: "ctrl+f",
        slash: {
          name: "prompt",
          aliases: ["prompts", "ce-prompt"],
        },
        onSelect: (dialog) => showPromptPicker(dialog),
      },
    ])
  } else {
    api.ui.toast({
      variant: "warning",
      title: "CLI Expander plugin loaded",
      message: "This OpenCode version does not expose legacy command registration.",
    })
  }
}
