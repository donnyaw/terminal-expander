# Librarian — Write Mode

**Run after every worker agent completes.**

## Task

Extract learnings from the completed worker agent's work and save them to persistent memory.

## Steps

1. Read the worker agent's output, generated code, and QC report
2. Identify:
   - New architectural decisions
   - Code conventions or patterns discovered
   - Bugs encountered and fixed
   - API behaviors or crate gotchas
   - Permission/permission requirements discovered
   - Any "wish I knew this before starting" insights
3. For each significant learning:
   - Create `memories/YYYYMMDD-NN--short-slug/README.md` with full context
   - Append a row to `MEMORY.csv`
4. If an architectural decision was made:
   - Append to `DECISIONS.md`
5. If new code conventions were established:
   - Append to `CONVENTIONS.md`

## Output

- New entry file: `memories/YYYYMMDD-NN--short-slug/README.md`
- Updated: `MEMORY.csv`
- Updated: `DECISIONS.md` (if applicable)
- Updated: `CONVENTIONS.md` (if applicable)
