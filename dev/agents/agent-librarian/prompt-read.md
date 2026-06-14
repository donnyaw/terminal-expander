# Librarian — Read Mode

**Run before every worker agent.**

## Task

Inject relevant historical learnings into the upcoming worker agent's context.

## Steps

1. Read `MEMORY.csv`
2. Identify the target worker agent from `tasks.csv` (current `assigned_agent`)
3. Filter MEMORY.csv entries where `agent` matches the target worker
4. Also filter entries where `tags` overlap with the target's task area
5. For each relevant entry, read the full markdown from `memories/<artifact_folder>/README.md`
6. Write the most relevant learnings into `instructions-from-orchestrator.md` under a `## MEMORY` section

## Output

Updated `instructions-from-orchestrator.md` with memory context that will help the worker avoid past mistakes and follow established conventions.
