# /implement — Implementation Loop

## Instructions

You are implementing a feature from an approved plan. Follow the plan precisely.
Do not make architectural decisions that the plan doesn't address — if you
encounter an ambiguity, STOP and ask the user rather than guessing.

## Step 1: Load Context

1. Read the plan at the specified path (or from scratchpad's active plan)
2. Read the appropriate scratchpad for current state and any decisions already made:
   - Workstream session: `scratchpad-<workstream>.md` + `scratchpad.md`
   - Single workstream: `scratchpad.md`
3. Determine your scope:
   - If a workstream is specified: you may ONLY modify files listed for that workstream
   - If no workstream: you may modify any file listed in the plan
   - Files not listed in the plan are OFF LIMITS unless you get user approval

## Step 2: Implementation Loop

For each task in the plan (or continuing from scratchpad's "next step"):

### 2a. Implement
Write the code for this task. Follow conventions from CLAUDE.md.
Reference existing patterns in the codebase for consistency.

### 2b. Build + Test
Run the project's build command and test suite.
- Build fails → fix immediately before proceeding
- Tests fail → fix immediately before proceeding
- Do not move to the next task with a broken build

### 2c. Self-Review (before committing)
Review your own changes for this task:
- Edge cases: what inputs or states could cause this to fail?
- Error handling: are all failure paths handled? No silent failures?
- Plan compliance: does this match what the plan specified?
- Scope: did you modify any files outside your scope? If yes, revert those changes.
- Naming: do names match project conventions?

If self-review finds issues, fix them before committing.

### 2d. Commit
Commit with a descriptive message following project conventions.
Message format: "<type>: <what changed and why>"
Examples:
- "feat: add pathfinding A* implementation for ground units"
- "fix: handle empty input in resource parser"
- "refactor: extract shared validation logic to utils"

### 2e. Progress Check
- If context usage > 50%: run /handoff and advise user to start fresh session
- If the plan needs to change: STOP, explain what you've discovered,
  and ask user whether to update the plan or proceed differently
- If scope has grown beyond the current tier: advise user to run
  /handoff --escalate

## Step 3: Completion

When all tasks in scope are complete:
1. Run full test suite one final time
2. Update the appropriate scratchpad: mark as
   "Implementation complete, ready for validation"
3. Report to user: what was built, how many commits, any deviations from plan

## CRITICAL RULES

- NEVER modify files outside your declared scope without user approval
- NEVER make architectural decisions the plan doesn't cover — ask instead
- NEVER continue past a failing build or test suite
- ALWAYS commit after each completed subtask, not in bulk at the end
- If you discover the plan is wrong: STOP and say so. Do not silently diverge.
