# /adversarial — Hostile Environment and Complex Interaction Testing

## Instructions

Your goal is to find failure modes that a standard code review would miss.

Do NOT repeat findings that a basic code review covers. Assume that error
handling completeness, null/empty input edge cases, basic type safety, and
standard boundary conditions have already been verified by /review. If you
find those categories of issues, skip them — they belong in /review, not here.

Instead, focus on:
- Complex INTERACTIONS between components that create emergent failures
- HOSTILE ENVIRONMENTS where external systems misbehave
- TEMPORAL issues: race conditions, ordering dependencies, timing windows
- ADVERSARIAL INPUTS: not just "empty string" but intentionally crafted
  payloads designed to exploit assumptions

## Step 1: Understand Intent

Read the plan/spec to understand what the code SHOULD do.
Read the implementation to understand what it ACTUALLY does.
Identify assumptions the code makes about its environment and inputs.
Those assumptions are your attack surface.

## Step 2: Attack Vectors

Focus exclusively on these categories:

### State and Sequence Attacks
- Invalid state transitions (calling methods in unexpected order)
- Interleaved operations from multiple callers or threads
- Operation interrupted mid-execution (crash, timeout, signal, cancel)
- Retry after partial failure (is state consistent? are side effects idempotent?)
- State that drifts over time (counter overflow, cache staleness, connection pool exhaustion)

### Concurrency Attacks
- Race conditions between concurrent access paths
- Deadlock potential (lock ordering violations)
- Livelock under contention
- ABA problems in lock-free structures
- Stale reads after concurrent writes

### Environment Attacks
- External service timeout, connection reset, partial response
- Filesystem: permission denied, disk full, file locked by another process
- Network: DNS failure, TLS certificate expiry, proxy interference
- Resource exhaustion: file handle limit, thread pool saturation, memory pressure
- Clock: NTP jump, timezone change, daylight saving transition
- Platform: OS signal during critical section, OOM killer

### Adversarial Input Attacks
- Inputs crafted to trigger worst-case algorithmic complexity (e.g., hash collision
  floods, regex backtracking, deeply nested structures)
- Inputs that exploit string encoding assumptions (mixed encodings,
  overlong UTF-8 sequences, null bytes mid-string)
- Inputs that exceed implicit size assumptions not enforced by the code
- Serialized data from a previous version (schema evolution attacks)

### Cascade Failures
- One component failure propagating to unrelated components
- Error handling that itself causes errors (e.g., logging failure during error path)
- Retry storms amplifying a partial outage
- Circular dependencies revealed under failure conditions

## Step 3: Report

For each finding:

```markdown
## Finding <N>: <title>

**Severity:** critical / high / medium / low
**Vector:** <state / concurrency / environment / adversarial input / cascade>

**Reproduction:**
<exact steps, inputs, or conditions to trigger the failure>

**Expected behavior:**
<what should happen according to the spec>

**Actual behavior:**
<what actually happens — crash, wrong result, hang, data corruption, etc.>

**Root cause:**
<why the code fails — which assumption is violated>

**Suggested fix:**
<how to address it>

**Test case:**
<test code that would catch this regression>
```

Prioritize findings by: severity × likelihood of occurrence in production.
Findings that require rare but plausible conditions rank higher than findings
that require attacker access or extremely unusual environments.
