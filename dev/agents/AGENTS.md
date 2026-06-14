# Agent Roster

## Pipeline Flow

```
orchestrator → planner → librarian-read → worker → librarian-write → qc → next
```

## Agent Assignments

| # | Agent | Phase | Model | Purpose |
|---|-------|-------|-------|---------|
| 1 | orchestrator | Orchestration | deepseek-v4-pro | Classify intent, route tasks, manage pipeline state |
| 2 | planner | Planning | gpt-5.5 | Architecture, crate decomposition, trait design, spec |
| 3 | form-tui | Implementation | deepseek-v4-flash | Cursive form renderer: text, multiline, choice/list |
| 4 | rust-core | Implementation | deepseek-v4-flash | Match engine, YAML config, render pipeline, variables |
| 5 | platform | Implementation | deepseek-v4-flash | evdev detection, uinput injection, permissions |
| 6 | shell | Implementation | deepseek-v4-flash | zle hooks, bash bindings, fish abbr, installer |
| 7 | test-build | QA | deepseek-v4-flash | Tests, cargo build/clippy, CI config |
| 8 | librarian | Memory | deepseek-v4-flash | Read/write persistent memory across agent runs |
| 9 | qc | QC | gpt-5.5 | Quality gate — review and score every phase |
| 10 | docs | Documentation | deepseek-v4-flash | User guide, config reference, form syntax |

## Operating Rules

1. Read `CONTEXT.md`, `tasks.csv`, and dependency outputs before editing code.
2. Run `librarian-read` before starting any worker task.
3. Run `librarian-write` after completing any worker task.
4. Require QC pass before starting dependent phases.
5. Keep changes minimal and correct — no speculative code.
6. Never expose secrets, tokens, or raw credentials in prompts or outputs.
7. Document environment variable names only, never values.
8. Update `tasks.csv` status after each phase completes.
9. Record learnings in `MEMORY.csv` via librarian after every worker run.

## Invocation

```bash
opencode run --format json --model <MODEL> --prompt-file agents/agent-XX/prompt.md --dir $PROJECT_ROOT
```
