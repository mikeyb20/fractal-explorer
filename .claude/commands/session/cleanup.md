# /cleanup — Post-Completion Housekeeping

## Instructions

You are cleaning up after a completed feature or release. Development leaves
behind plan files, scratchpads, worktrees, and branches that served their
purpose but now clutter the repo. Remove or archive them so the next cycle
starts clean.

## Step 1: Identify Completed Work

Determine what was just completed:
1. Check scratchpad.md for the most recent feature
2. Check recent merge commits: `git log --merges --oneline -5`
3. Ask the user if ambiguous

## Step 2: Archive Plan Files

For each plan file related to the completed work:

```bash
mkdir -p docs/plans/archive
mv docs/plans/<completed-feature>.md docs/plans/archive/
```

Do NOT delete plan files — they're valuable historical records of decisions.

## Step 3: Clean Up Branches

```bash
# List merged branches
git branch --merged main

# For each merged feature branch (not main, not develop):
git branch -d <branch>
```

Confirm with the user before deleting any branches. List them first.

## Step 4: Prune Worktrees

```bash
# List all worktrees
git worktree list

# Remove stale worktree metadata
git worktree prune

# Check for leftover worktree directories
ls .claude/worktrees/ 2>/dev/null
```

For any remaining worktree directories with no uncommitted changes,
offer to remove them. For worktrees with uncommitted changes, warn the user.

## Step 5: Reset Scratchpads

For the completed feature:
- If `scratchpad.md` references the completed feature: clear its contents
  and replace with a blank template ready for the next task
- If `scratchpad-<workstream>.md` files exist for completed workstreams:
  archive them to `docs/plans/archive/` alongside the plan, then delete

Blank scratchpad template:

```markdown
# Scratchpad

## Current State
- Working on: <next task — to be filled>
- Branch: main
- Last completed step: —
- Next step: —

## Decisions Made During Implementation

## Open Questions

## Known Issues
```

## Step 6: Report

```
Cleanup complete:
- Plan archived: docs/plans/archive/<feature>.md
- Branches deleted: <list>
- Worktrees pruned: <list>
- Scratchpads reset: <list>
- Repo ready for next development cycle.
```
