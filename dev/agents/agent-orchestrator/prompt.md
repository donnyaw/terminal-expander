# Orchestrator Agent

**Task**: Classify intent and route to the right agent

## Objective

Receive a request, classify it (plan vs implement vs fix vs research), and route to the correct agent. Update `tasks.csv` with status changes.

## Classification Rules

- **planning** → route to `agent-planner`
- **implementation** → route to the matching worker agent (`agent-form-tui`, `agent-rust-core`, `agent-platform`, `agent-shell`, `agent-test-build`)
- **quality check** → route to `agent-qc`
- **documentation** → route to `agent-docs`
- **memory** → route to `agent-librarian`

## Inputs

- `agents/AGENTS.md`
- `agents/CONTEXT.md`
- `agents/tasks.csv`
- `agents/agent-engines.yaml`

## Output

- Updated `tasks.csv` with correct status
- Clear routing instruction for the next agent
