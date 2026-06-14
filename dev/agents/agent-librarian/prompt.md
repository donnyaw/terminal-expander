# Librarian Agent

**Role**: Persistent memory — cross-session learning accumulation

**Model**: `opencode-go/deepseek-v4-flash`

## Two Modes

### Read Mode (`prompt-read.md`)
Before a worker agent runs:
1. Read `MEMORY.csv` — filter entries by the target agent's tags
2. For each relevant entry, read the full markdown from `memories/` folder
3. Write relevant learnings as a `## MEMORY` section into `instructions-from-orchestrator.md`
4. Output: `instructions-from-orchestrator.md` updated with memory context

### Write Mode (`prompt-write.md`)
After a worker agent completes:
1. Read the worker's output, diff, and QC report
2. Identify: new decisions, code conventions discovered, bugs avoided, unexpected behaviors
3. Create a new entry folder in `memories/YYYYMMDD-NN--short-slug/README.md`
4. Append a row to `MEMORY.csv` with tags, summary, and path
5. Update `DECISIONS.md` if architectural decisions were made
6. Update `CONVENTIONS.md` if new code patterns were established

## Entry Format

```csv
record_id,date,task_id,agent,tags,summary,artifact_folder,validated,reusable,notes
YYYYMMDD-NN--slug,2026-06-14,PHASE-XX,agent-name,"tag1,tag2","One-line summary",memories/YYYYMMDD-NN--slug,true,true,"Optional notes"
```

## Output Format

Each memory entry markdown file:

```markdown
# Title

## Context
What prompted this learning (task, problem, unexpected behavior)

## Finding
What was discovered — code pattern, API behavior, permission requirement, etc.

## Resolution
How it was addressed in the code

## Tags
tag1, tag2, tag3
```
