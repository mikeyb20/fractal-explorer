# /integrate-review — Post-Merge Integration Validation

## Instructions

Multiple workstreams have been merged. Your job is to find integration
issues — problems that exist BECAUSE separate agents built separate
parts, not bugs within a single workstream. Think: mismatched
assumptions, inconsistent patterns, missing glue code.

## Step 1: Load Context

Read the plan to understand:
- What each workstream was responsible for
- What interfaces were defined between them
- What the expected integration points are

## Step 2: Interface Verification

For each shared interface defined in the plan:
1. Find the interface definition
2. Find all implementations (from producer workstreams)
3. Find all usages (from consumer workstreams)
4. Verify: does usage match implementation?
   - Correct types passed?
   - Error cases handled?
   - Return values used correctly?
   - Null/optional/error states accounted for?

## Step 3: Cross-Workstream Consistency

Check for:
- **Naming inconsistency:** same concept named differently across workstreams
  (e.g., "user" vs "account" vs "player")
- **Error handling inconsistency:** one workstream uses Result, another throws
- **Logging inconsistency:** different log levels or formats for similar events
- **Config inconsistency:** same setting read from different env vars or files
- **Pattern inconsistency:** one workstream uses async, another blocks

## Step 4: Missing Integration

Look for:
- Frontend components that reference backend endpoints that don't exist
- Backend handlers that expect request formats the frontend doesn't send
- Database schemas that don't match the ORM models
- Event publishers without corresponding subscribers
- Config values referenced but never defined

## Step 5: Report

```markdown
## Integration Review: <feature name>

### Interface Compliance
| Interface | Status | Details |
|-----------|--------|---------|
| <name> | ✅ / ❌ | <specifics> |

### Integration Issues
<numbered list with file references and fix recommendations>

### Consistency Issues
<numbered list — lower priority but worth cleaning up>

### Missing Integration Points
<anything that needs glue code to connect workstreams>

### Verdict: <INTEGRATED / NEEDS WORK>
```
