# /catalog-debt — Tech Debt Cataloging

## Instructions

You are cataloging tech debt after a development sprint. Your job is to
produce a prioritized, actionable backlog — not a vague list of complaints.
Every item must be specific enough that an agent could implement the fix
from your description alone.

## Step 1: Gather Sources

1. **Changed files:** `git diff <last-tag>..HEAD --name-only`
2. **Scratchpads:** read all scratchpad*.md files for "Known Issues"
   and workarounds noted during the build
3. **TODO/FIXME scan:** `grep -rn "TODO\|FIXME\|HACK\|XXX" <src dirs>`
4. **Plan deviations:** check scratchpad "Decisions Made" sections for
   intentional shortcuts taken during the sprint

## Step 2: Analyze Each Source

For each file changed since last release:
- Is naming clear and consistent?
- Is error handling complete or were corners cut?
- Is there duplicated logic (especially if multiple workstreams added similar code)?
- Have module boundaries drifted from the documented architecture?
- Are there missing tests for new code paths?
- Are there performance concerns (allocations in loops, N+1 queries)?

For each TODO/FIXME/HACK:
- Is it still relevant?
- What's the fix?
- What's the risk of leaving it?

For each known issue from scratchpads:
- Is the workaround still in place?
- What's the proper fix?

## Step 3: Prioritize

Score each item on three dimensions:
- **Blast radius (B):** How much breaks if this causes a problem?
  - 3 = system-wide impact
  - 2 = module-level impact
  - 1 = isolated impact
- **Likelihood (L):** How likely is this to cause a problem?
  - 3 = will definitely cause issues soon
  - 2 = might cause issues under certain conditions
  - 1 = unlikely unless specific rare conditions
- **Effort (E):** How hard is the fix?
  - S = < 1 hour
  - M = 1-4 hours
  - L = 4+ hours

Priority score = B × L (effort helps with scheduling, not prioritization)

## Step 4: Output

Create or update `docs/debt-backlog.md`:

```markdown
# Tech Debt Backlog
Generated: <date>
Scope: changes since <last-tag>

## Critical Priority (score 6-9)
- [ ] **<title>** — <file:line or module>
  - Issue: <specific description>
  - Impact: <what goes wrong>
  - Fix: <specific recommendation>
  - Blast radius: <1-3> | Likelihood: <1-3> | Effort: <S/M/L>

## Medium Priority (score 3-5)
<same format>

## Low Priority (score 1-2)
<same format>

## Metrics
- Total items: <N>
- Critical: <N> | Medium: <N> | Low: <N>
- Quick wins (critical + small effort): <list>
```
