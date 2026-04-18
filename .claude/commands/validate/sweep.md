# /sweep — Focused Validation Sweep

## Instructions

You are running a focused validation sweep. Examine ONLY the specified
concern — do not mix concerns. A safety sweep does not comment on
architecture. A performance sweep does not flag naming issues.

## Step 0: Sweep Selection (if type not specified)

If the user hasn't specified which sweep type to run, recommend based on the
type of change. Read the plan file and recent diff to determine:

| Change Type | Recommended Sweeps |
|------------|-------------------|
| New code (new modules, features) | safety + architecture |
| Refactor (restructuring existing code) | architecture + duplication |
| Performance-sensitive (game loops, hot paths, high-throughput) | performance |
| Multi-workstream merge | duplication (cross-workstream duplicate detection) |
| C++ or unsafe Rust code | safety (always) |
| Bug fix only | safety (focused on the fix area) |

Recommend 1-2 sweep types. The user may override. Do NOT default to all 4 —
that's only appropriate for major releases or comprehensive audits.

## Sweep Types

### safety
Focus: memory safety, undefined behavior, panics, error handling completeness.

Examine:
- **C++:** UB (signed overflow, null deref, dangling refs, buffer access),
  uninitialized variables, missing virtual destructors, raw pointer usage
  in non-RAII contexts, exception safety guarantees
- **Rust:** unwrap/expect without justification, unsafe blocks (are they
  necessary? are invariants documented?), panic paths in library code,
  error types that lose context
- **Go:** unchecked errors (especially from Close, Write), nil pointer
  risks, goroutine leaks, race conditions
- **TypeScript:** any casts, non-null assertions (!), uncaught promise
  rejections, type narrowing that could fail at runtime

### architecture
Focus: module boundaries, dependency direction, abstraction quality.

Examine:
- Does the module structure match the documented architecture?
- Are there circular dependencies?
- Are abstractions at the right level? (too thin = wrapper noise, too thick = god objects)
- Does data flow in the expected direction? (e.g., UI → service → data, not reverse)
- Are there layering violations? (e.g., data layer importing UI types)
- Have modules grown beyond their stated responsibility?

### performance
Focus: unnecessary allocations, algorithmic complexity, cache behavior.

Examine:
- Allocations in hot loops (per-frame in games, per-request in servers)
- Algorithmic complexity mismatches (O(n²) where O(n log n) is achievable)
- Unnecessary copies (especially large structs)
- Cache-unfriendly access patterns (pointer chasing, random access on large arrays)
- Missing batching opportunities
- Database N+1 query patterns
- Blocking operations on async paths

### duplication
Focus: repeated logic, near-duplicate code, extraction opportunities.

Examine:
- Functions/methods that do nearly the same thing with slight variations
- Copy-pasted code blocks (especially across workstream boundaries after merge)
- Repeated error handling patterns that should be abstracted
- Configuration or constants duplicated across files
- Test setup code that should be shared fixtures

## Output Format

```markdown
## <Sweep Type> Sweep: <scope description>

### Critical (high blast radius, likely to cause issues)
1. **<file:line>** — <finding>
   - Impact: <what goes wrong>
   - Fix: <specific recommendation>

### Important (moderate risk)
<same format>

### Minor (low risk, worth addressing when nearby)
<same format>

### Summary
- <N> critical, <N> important, <N> minor findings
- Highest priority: <top 3 items to fix first>
```
