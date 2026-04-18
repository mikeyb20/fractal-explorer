# /docs — Generate and Sync Documentation

## Instructions

You are performing a comprehensive documentation pass. This covers two
complementary tasks: adding inline documentation to code, and syncing
external architecture documentation with the codebase. Both tasks ensure
that documentation matches the code as it exists right now.

## Part 1: Inline API Documentation

### Step 1.1: Identify Undocumented Interfaces

Scan the specified files (or recent diff if no files specified) for public
interfaces lacking documentation:
- **Rust:** pub fn, pub struct, pub enum, pub trait without /// comments
- **C++:** public class members, free functions in headers without /** */ or ///
- **Go:** exported functions/types without preceding comment
- **TypeScript:** exported functions/types/interfaces without JSDoc

### Step 1.2: Write Inline Documentation

For each undocumented interface, add documentation following the project's
existing style. Include:

**Functions/Methods:**
```
/// <One-line summary of what this does>
///
/// <Longer description if behavior is non-obvious>
///
/// # Arguments
/// * `<name>` - <what it is, valid range, constraints>
///
/// # Returns
/// <What is returned and under what conditions>
///
/// # Errors
/// <When this fails and what error types are returned>
///
/// # Examples
/// ```
/// <usage example for complex interfaces>
/// ```
```

**Types/Structs:**
```
/// <What this type represents>
///
/// <When to use this type, invariants it maintains>
```

**Traits/Interfaces:**
```
/// <What contract implementors must satisfy>
///
/// <Key methods and their relationship>
```

## Part 2: Architecture Documentation Sync

### Step 2.1: Inventory Documentation

Scan `docs/` and README files for architecture-related content:
- Architecture overviews
- Module descriptions
- Component diagrams (text-based)
- API specifications
- Setup/build instructions

### Step 2.2: Compare Against Code

For each documentation file:

1. Read the documented claims about project structure
2. Verify against actual codebase:
   - Do documented modules still exist?
   - Are module responsibilities accurately described?
   - Are documented file paths still correct?
   - Do documented interfaces match actual signatures?
   - Are build/test/run commands still accurate?

3. Identify:
   - **Stale content:** describes things that no longer exist or work differently
   - **Missing content:** new modules/components not yet documented
   - **Inaccurate content:** describes things incorrectly

### Step 2.3: Update

- Remove or correct stale content
- Add documentation for new modules/components
- Update file paths, command examples, and interface descriptions
- Preserve the documentation's existing style and level of detail
- Do NOT add excessive detail — match the granularity of existing docs

## Part 3: Verify and Commit

### Step 3.1: Verify

- All file paths mentioned in docs exist
- All commands mentioned in docs work
- All module names match actual directory/file names
- CLAUDE.md "Key Docs" section points to files that exist
- Inline documentation compiles (doc tests pass if applicable)
- No existing documentation was removed or degraded
- Style is consistent with the rest of the project

### Step 3.2: Commit

```bash
git add <all documentation changes>
git commit -m "docs: update inline API docs and sync architecture documentation

Inline docs added/updated for:
- <module 1>
- <module 2>

Architecture docs synced:
- <doc file>: <what changed>
"
```
