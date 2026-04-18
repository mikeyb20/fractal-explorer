# /review — Post-Build Diff Review

## Instructions

You are reviewing code you did NOT write. You have no context from the
implementation session. This is intentional — you catch things the
building agent is blind to because you don't share its assumptions.

Focus on correctness and robustness. Do NOT comment on style unless
it impacts readability or correctness.

## Step 1: Load Context

1. Identify the feature branch: `git branch --show-current`
2. Load the diff: `git diff main..HEAD`
3. Read the plan: check scratchpad(s) for the plan path,
   or find it in `docs/plans/`
4. Read CLAUDE.md for project conventions

## Step 2: Spec Compliance

For each item in the plan:
- Is it implemented? (yes / no / partial)
- Does the implementation match the specified approach?
- Are the interfaces exactly as defined in the plan?

Flag any deviations. Check scratchpad "Decisions Made" section —
documented deviations are acceptable if the rationale is sound.
Undocumented deviations are findings.

## Step 3: Correctness Review

For each file in the diff:

### Error Handling
- Are all error paths handled?
- Any panics, unwraps, or bare exceptions that should be Results/Options?
- Any silent failures (errors caught but not logged or propagated)?

### Edge Cases
- Empty/null/zero inputs
- Boundary values (max int, empty collections, single element)
- Concurrent access (if applicable)
- Resource cleanup on error paths

### Safety (language-specific)
- **C++:** undefined behavior, dangling references, buffer overflows,
  uninitialized memory, missing virtual destructors
- **Rust:** unsafe blocks justified?, lifetime correctness, unwrap audit
- **Go:** unchecked errors, goroutine leaks, nil pointer risks
- **TypeScript:** any casts, null assertions, type narrowing gaps

### Logic
- Do conditionals cover all cases?
- Are loop bounds correct?
- Are off-by-one errors present?
- Do mathematical operations handle overflow/underflow?

## Step 4: Report

```markdown
## Review: <feature name>

### Spec Compliance
<pass/fail per plan item, with notes on documented deviations>

### Issues Found

#### Critical (must fix before merge)
<numbered list with file:line, description, and fix recommendation>

#### Important (should fix)
<numbered list>

#### Minor (optional improvements)
<numbered list>

### Verdict: <APPROVE / REVISE>
```

If REVISE: list specific items that must be addressed. After fixes are made,
a new `/review` session should be run before proceeding to `/done-check`.
