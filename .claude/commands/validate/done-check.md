# /done-check — Verify Done Criteria (Final Gate)

## Instructions

You are the final gate before merge. Every done criterion from the plan must
be verified. This runs AFTER code review — the code should already be correct.
Your job is to confirm completeness, not find bugs (that was /review's job).

If any criterion fails, the feature is not ready to merge. Be rigorous.
"Probably works" is not a pass.

## Step 1: Load Criteria

Read the plan file and extract the "Done Criteria" section.
Each criterion should be a testable statement.

## Step 2: Verify Each Criterion

For each criterion:

1. **Determine how to verify it:**
   - Can it be checked by reading code? → read the relevant files
   - Can it be checked by running tests? → run them and report results
   - Can it be checked by running the application? → note it requires manual check
   - Can it be checked by inspecting git history? → check commits

2. **Execute the verification**

3. **Record the result:**
   - ✅ **PASS** — criterion is met, with evidence
   - ❌ **FAIL** — criterion is not met, with explanation of what's missing
   - ⚠️ **MANUAL** — requires human verification (e.g., "UI looks correct")

## Step 3: Run Full Test Suite

Execute the project's test command. Report:
- Total tests: <count>
- Passing: <count>
- Failing: <count> (with details)
- Skipped: <count>

## Step 4: Report

```markdown
## Done Check: <feature name> (Final Gate)

| # | Criterion | Status | Evidence |
|---|-----------|--------|----------|
| 1 | <criterion> | ✅ PASS | <evidence> |
| 2 | <criterion> | ❌ FAIL | <what's missing> |

### Test Suite
<pass/fail summary>

### Verdict: <READY TO MERGE / NOT READY>

### Remaining Items (if not ready)
<specific list of what needs to happen before re-running done-check>
```
