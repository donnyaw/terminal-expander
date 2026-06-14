#!/usr/bin/env bash
# run.sh — Non-interactive opencode run pipeline for terminal-expander
# Usage: ./run.sh <phase-task-id>
#
# Example: ./run.sh PHASE-01
# Example: ./run.sh PHASE-04 PHASE-05  (run multiple)
#
# Runs the librarian-read → worker → librarian-write → QC sequence.
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
AGENTS="$PROJECT_ROOT/dev/agents"
TASKS_CSV="$PROJECT_ROOT/dev/tasks.csv"

run_agent() {
    local task_id="$1"
    local agent_name="$2"
    local model="$3"
    local prompt_file="$4"

    echo "=== [$task_id] Running $agent_name ($model) ==="

    opencode run --format json \
        --model "$model" \
        --prompt-file "$prompt_file" \
        --dir "$PROJECT_ROOT"

    echo "=== [$task_id] $agent_name complete ==="
}

run_librarian_read() {
    echo "--- Librarian: injecting memory ---"
    opencode run --format json \
        --model "opencode-go/deepseek-v4-flash" \
        --prompt-file "$AGENTS/agent-librarian/prompt-read.md" \
        --dir "$PROJECT_ROOT"
}

run_librarian_write() {
    echo "--- Librarian: extracting learnings ---"
    opencode run --format json \
        --model "opencode-go/deepseek-v4-flash" \
        --prompt-file "$AGENTS/agent-librarian/prompt-write.md" \
        --dir "$PROJECT_ROOT"
}

run_qc() {
    local task_id="$1"
    echo "=== QC: $task_id ==="
    opencode run --format json \
        --model "openai/gpt-5.5" \
        --prompt-file "$AGENTS/agent-qc/prompt.md" \
        --dir "$PROJECT_ROOT"
}

# Map task IDs to agent + model
run_task() {
    local task_id="$1"
    case "$task_id" in
        PHASE-00)
            run_agent "$task_id" "agent-rust-core" "opencode-go/deepseek-v4-flash" "$AGENTS/agent-rust-core/prompt.md"
            ;;
        PHASE-01)
            run_agent "$task_id" "agent-planner" "openai/gpt-5.5" "$AGENTS/agent-planner/prompt.md"
            ;;
        PHASE-01-qc|PHASE-02-qc|PHASE-03-qc|PHASE-04-qc|PHASE-05-qc|PHASE-06-qc|PHASE-07-qc|PHASE-08-qc|PHASE-09-qc|PHASE-10-qc|PHASE-11-qc|PHASE-12-qc|PHASE-13-qc|PHASE-14-qc|PHASE-15-qc)
            run_qc "$task_id"
            ;;
        PHASE-02|PHASE-03|PHASE-06)
            run_librarian_read
            run_agent "$task_id" "agent-rust-core" "opencode-go/deepseek-v4-flash" "$AGENTS/agent-rust-core/prompt.md"
            run_librarian_write
            run_qc "$task_id"
            ;;
        PHASE-04|PHASE-05)
            run_librarian_read
            run_agent "$task_id" "agent-form-tui" "opencode-go/deepseek-v4-flash" "$AGENTS/agent-form-tui/prompt.md"
            run_librarian_write
            run_qc "$task_id"
            ;;
        PHASE-07|PHASE-08)
            run_librarian_read
            run_agent "$task_id" "agent-platform" "opencode-go/deepseek-v4-flash" "$AGENTS/agent-platform/prompt.md"
            run_librarian_write
            run_qc "$task_id"
            ;;
        PHASE-09|PHASE-10|PHASE-11)
            run_librarian_read
            run_agent "$task_id" "agent-shell" "opencode-go/deepseek-v4-flash" "$AGENTS/agent-shell/prompt.md"
            run_librarian_write
            run_qc "$task_id"
            ;;
        PHASE-12|PHASE-13)
            run_librarian_read
            run_agent "$task_id" "agent-test-build" "opencode-go/deepseek-v4-flash" "$AGENTS/agent-test-build/prompt.md"
            run_librarian_write
            run_qc "$task_id"
            ;;
        PHASE-14|PHASE-15)
            run_agent "$task_id" "agent-docs" "opencode-go/deepseek-v4-flash" "$AGENTS/agent-docs/prompt.md"
            run_qc "$task_id"
            ;;
        *)
            echo "Unknown task: $task_id"
            echo "Valid tasks: PHASE-00 through PHASE-15"
            exit 1
            ;;
    esac
}

# Main
if [ $# -eq 0 ]; then
    echo "Usage: $0 <task-id> [task-id ...]"
    echo ""
    echo "Run full pipeline in order:"
    echo "  PHASE-00  → Initialize workspace"
    echo "  PHASE-01  → Architecture planning"
    echo "  PHASE-02  → Config parser"
    echo "  PHASE-03  → Match engine"
    echo "  PHASE-04  → Form: text fields"
    echo "  PHASE-05  → Form: choice/list"
    echo "  PHASE-06  → Template renderer"
    echo "  PHASE-07  → evdev detection"
    echo "  PHASE-08  → uinput injection"
    echo "  PHASE-09  → zle widget"
    echo "  PHASE-10  → bash bindings"
    echo "  PHASE-11  → fish integration"
    echo "  PHASE-12  → Integration tests"
    echo "  PHASE-13  → CI config"
    echo "  PHASE-14  → User docs"
    echo "  PHASE-15  → Developer docs"
    exit 0
fi

for task in "$@"; do
    run_task "$task"
done

echo "=== Pipeline complete ==="
