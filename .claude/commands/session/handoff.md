# /handoff — Persist Session State

## Instructions

You are saving the current session's state so a future session (possibly with
no shared context) can continue exactly where you left off. Be precise and
complete — anything you don't write down is lost.

If the user specified `--escalate`, this is a tier escalation — the task has
grown beyond its original scope and needs a higher-tier plan before continuing.

## Step 1: Commit Pending Work

1. Check `git status` for uncommitted changes
2. If changes exist: stage and commit with a descriptive message
   - If changes are incomplete/WIP: commit with prefix "WIP: <description>"
   - If escalating: commit with "WIP: pausing for tier escalation — <reason>"
3. If no changes: note this (session may have been read-only)

## Step 2: Assess Progress

Compare current state against the plan at `docs/plans/<feature>.md`:
- Which tasks/steps from the plan are complete?
- Which task is currently in progress?
- What is the exact next step to continue?

## Step 3: Assess Escalation (if --escalate)

If escalating, additionally determine:
- How many files will this task actually touch? (revised estimate)
- Are there multiple independent concerns that could be parallelized?
- Are there shared interfaces that need to be defined first?
- What was learned during implementation that changes the approach?
- Which existing work should be kept, rebased, or discarded?

Recommend the appropriate tier:
- **Tier 1:** 3-10 files, single workstream, needs a lightweight plan
- **Tier 2:** 10+ files or multiple concerns, needs full plan with workstreams

## Step 4: Update Scratchpad

Determine the correct scratchpad file:
- If this is a workstream session: update `scratchpad-<workstream>.md`
- If this is a single-workstream feature: update `scratchpad.md`

Update the scratchpad (create if it doesn't exist). Preserve existing
content in the Decisions and Known Issues sections — append, don't overwrite.

```markdown
# Scratchpad

## Current State
- Working on: <feature name>
- Plan: docs/plans/<feature>.md
- Branch: <branch name>
- Workstream: <workstream name, or "full feature">
- Last completed step: <specific description of what was just finished>
- Next step: <specific description of what should happen next>
- Context usage at handoff: <percentage if known>
- Handoff reason: <context full / pausing / escalating / complete>

## Decisions Made During Implementation
<preserve all existing entries>
- <any new decisions from this session>: <rationale>

## Open Questions
<preserve unresolved, remove resolved>
- <any new questions discovered>

## Known Issues
<preserve existing>
- <any new issues discovered during this session>
```

If escalating, append:

```markdown
## Escalation Note
- Escalated from Tier <X> to Tier <Y> at <timestamp>
- Reason: <why the original scope was insufficient>
- Existing work on branch <branch>: <keep / discard / partial — with details>
- Revised file count estimate: <N files>
- Key learnings to carry into new plan:
  - <what was discovered during implementation>
  - <decisions that should be preserved>
  - <approaches that failed and why>
```

## Step 5: Confirm

Report to user:
- Files committed (or "no changes")
- Scratchpad updated at <path>
- Ready for session end

If escalating, additionally advise:
1. What tier the work should escalate to
2. Whether existing work should be kept, rebased, or discarded
3. Recommended next step: "Start a fresh session and run `/plan` referencing
   the scratchpad for context from this attempt"

Do NOT continue implementation after this skill runs. The session is ending.
