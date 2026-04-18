# /build-interfaces — Contract-First Interface Scaffolding

## Instructions

You are building shared interfaces that parallel workstreams will depend on.
These are the contracts between workstreams — they must be correct, complete,
and stable. Once committed, implementation workstreams treat these files as
read-only.

## Step 1: Load Plan

Read the Tier 2 plan. Extract the "Shared Interfaces" section.
For each interface listed, gather:
- Type/trait/interface name
- File path where it will live
- Which workstreams produce it and which consume it
- Expected behavior contract

## Step 2: Implement Interfaces

For each shared interface:

1. Create the file at the specified path
2. Implement the type definitions, trait declarations, and function signatures
3. Add documentation comments explaining:
   - What this interface represents
   - Which workstreams produce and consume it
   - Invariants that implementations must maintain
   - Example usage (for complex interfaces)
4. Where full implementation is deferred to a workstream:
   - Use `todo!()` / `unimplemented!()` (Rust)
   - Use stub bodies with `// TODO: implemented by workstream-X` (C++)
   - Use `throw new Error('Not implemented')` (TypeScript)

## Step 3: Validate Contracts

For each interface, verify:
- [ ] All types referenced in the plan exist
- [ ] All function signatures match the plan
- [ ] No circular dependencies between interface files
- [ ] Documentation comments are present on all public items
- [ ] The code compiles/type-checks (stubs are fine, type errors are not)

## Step 4: Commit

```bash
git add <interface files>
git commit -m "feat: add shared interfaces for <feature name>

Contracts for parallel workstreams:
- <interface 1>: <purpose>
- <interface 2>: <purpose>

These files are read-only for implementation workstreams."
```

## Step 5: Report

Tell the user:
- Interfaces committed on current branch
- Each worktree should branch from this commit
- List which interfaces each workstream depends on and produces
- Ready to launch parallel sessions
