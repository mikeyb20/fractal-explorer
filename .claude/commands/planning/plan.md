# /plan — Create Development Plan

## Instructions

You are creating a development plan. Your goal is to produce a plan file that
eliminates all ambiguity before implementation begins. Every architectural
decision must be made explicitly in this plan — the implementation agent should
never need to guess.

## Step 1: Assess Tier

Determine the scope by analyzing the user's description:
- **Tier 1 (Standard):** 3-10 files, single concern, one workstream
- **Tier 2 (Complex):** 10+ files, multiple concerns, cross-cutting, or
  requires parallel workstreams

If the task is 1-2 files with an obvious fix, tell the user this doesn't
need a plan — just implement it directly.

## Step 2: Gather Context

Before writing the plan:
1. Read CLAUDE.md for project conventions and architecture
2. Explore the codebase areas relevant to the user's description
3. Identify all files that will need to change
4. Identify all interfaces (types, function signatures, API contracts) that
   will be created, modified, or consumed

Ask the user clarifying questions if you cannot determine:
- The intended approach or algorithm
- Which existing patterns to follow
- How errors should be handled
- What the boundaries of the change are

## Step 3: Write Plan

Create `docs/plans/<feature-name>.md` using the appropriate template.

### Tier 1 Template:

```
# <Feature Name>

## What
<One sentence describing the deliverable>

## Why
<What problem this solves — prevents scope creep later>

## Boundaries
<What this change does NOT touch — be explicit>

## Files to Create/Modify
- <path>: <what changes and why>

## Interfaces
<Function signatures, types, or API contracts introduced or changed>

## Done Criteria
- [ ] <Specific, testable condition>
- [ ] All existing tests pass
- [ ] New tests cover <specific scenarios>

## Test Approach
<What to test, which framework, TDD or post-implementation>
```

### Tier 2 Template:

Includes everything in Tier 1, plus the parallelism analysis sections below.

```
## Architecture Impact
<How this changes the system structure, new dependencies, affected modules>

## Workstream Breakdown

### Shared Interfaces (build first)
- <type/interface>: <file path> — <description>

### Workstream A: <name>
- **Files:** <exact list of files this workstream owns>
- **Depends on:** <interfaces from shared or other workstreams>
- **Exposes:** <interfaces other workstreams consume>
- **Estimated complexity:** <low/medium/high>

### Workstream B: <name>
<same structure>

## Dependency Graph
<Which workstreams depend on which, in what direction.
  Producer workstreams must complete before consumers begin.>

## Conflict Risk Assessment
For each pair of workstreams, rate the conflict risk:
- NONE: completely independent file sets
- LOW: shared read dependencies only
- MEDIUM: one produces interfaces the other consumes (ordered merge resolves)
- HIGH: both need to modify the same file (must serialize or redesign)

Specific conflicts:
- <File/module> touched by workstreams <A> and <B> — resolution: <who owns it>
- <Shared config/build files> — resolution: <which workstream owns>

If any HIGH conflict risks exist, redesign the split or serialize those tasks.
Do NOT approve a parallel split with unresolved HIGH conflict risks.

## Merge Order
1. Shared interfaces
2. <Producer workstream> — rationale: <why this merges first>
3. <Consumer workstream> — rationale: <depends on producer>
4. <Test/docs workstream> — rationale: <needs final codebase state>
```

## Step 4: Parallelism Validation (Tier 2 only)

For Tier 2 plans, validate the workstream breakdown before presenting:

### 4a. Build File Dependency Graph
For every file listed in the plan:
1. Classify as CREATE (new file) or MODIFY (existing file)
2. For MODIFY: identify what is being changed (types, functions, imports, config)
3. Map which files import/include/depend on which other files
4. Identify shared files: types, configs, build files, utilities

### 4b. Verify Independence
Confirm that:
- No two workstreams WRITE to the same file
- If workstream A reads a file that workstream B writes, A depends on B
  and the merge order reflects this
- Shared type definition files are owned by ONE workstream, read-only for others
- Build system files (CMakeLists, Cargo.toml, package.json) are owned by ONE
  workstream, typically the one adding dependencies
- If a file MUST be written by two workstreams, those tasks are serialized

### 4c. Define Contracts
For each workstream boundary, verify:
- Interface: exact type/function signatures exchanged between workstreams
- Direction: which workstream produces, which consumes
- Build order: producer must complete and commit before consumer begins

## Step 5: Self-Review

Before presenting the plan, verify:
- [ ] Every file that will be touched is listed explicitly
- [ ] No interface is consumed by one workstream but not produced by any
- [ ] Done criteria are testable (not vague like "works correctly")
- [ ] Boundaries are stated — what is explicitly OUT of scope
- [ ] For Tier 2: workstream file lists don't overlap (no write conflicts)
- [ ] For Tier 2: merge order respects dependency direction
- [ ] For Tier 2: no unresolved HIGH conflict risks

## Step 6: Present for Review

Present the plan to the user. Explicitly call out:
- Decisions you made that the user should confirm
- Risks or uncertainties you identified
- Alternative approaches you considered and why you chose this one

Do NOT begin implementation. Wait for user approval.
