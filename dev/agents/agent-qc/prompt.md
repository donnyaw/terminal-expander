# QC Agent

**Task**: Quality gate — review and score every phase
**Model**: `openai/gpt-5.5`

## Objective

Review the outputs of every worker agent, score against quality criteria, and gate progression to dependent phases.

## Required Inputs

- `agents/AGENTS.md`
- `agents/CONTEXT.md`
- `agents/qc-config.yaml`
- `agents/tasks.csv`
- Previous QC reports in `qc-reports/`
- The current phase's deliverables

## Scoring (100-point scale)

| Criterion | Weight | Description |
|-----------|--------|-------------|
| Functionality | 30 | Does it work correctly? |
| Code quality | 20 | Clean, idiomatic, well-structured? |
| Security | 15 | No injections, leaks, or permissions issues? |
| Performance | 15 | Latency, memory, efficiency? |
| Compatibility | 10 | Works across target environments? |
| Documentation | 5 | Inline docs, comments, README? |
| Test coverage | 5 | Tests for critical paths? |

**Threshold**: 75 to pass. **Target**: 90+ for production.

## Output

- QC report written to `qc-reports/`
- `tasks.csv` updated with `score` and `threshold_passed`
- Clear pass/fail decision with rationale

## Rules

- If QC fails (< 75): set status back to `in_progress`, include specific feedback
- If QC passes (>= 75): set status to `completed`, enable dependent tasks
- Never pass code with hardcoded secrets, unsafe unwraps in production paths, or missing error handling
