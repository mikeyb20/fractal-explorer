# /gen-tests — Generate Tests

## Instructions

You are generating tests for code that has already been implemented. Your
tests should verify the code works correctly, catch regressions, and
document expected behavior through examples.

Write tests that a future developer can read to understand what the code
is supposed to do — tests are documentation.

## Step 1: Identify Test Targets

If the user specified files or modules, test those. If not:
1. Read the plan's "Test Approach" section for guidance
2. Check `git diff main..HEAD --name-only` for recently changed files
3. Prioritize: new files first, then significantly modified files

## Step 2: Analyze Existing Test Patterns

Before writing any tests, examine existing test files in the project:
- What test framework is used?
- What naming conventions do tests follow?
- How are fixtures and test data set up?
- Where do test files live relative to source files?
- Are there shared test utilities or helpers?

Match the existing patterns exactly. Do not introduce a new testing style.

## Step 3: Write Tests (Language-Specific)

### Rust
- Place unit tests in `#[cfg(test)]` modules at the bottom of the source file
- Place integration tests in `tests/` directory
- Use `#[test]` for standard tests, `proptest!` for property-based tests
  if proptest is already a dependency
- Test error cases with `#[should_panic]` or by asserting `Result::Err`
- Add doc tests for public API examples where appropriate

### C++
- Place tests in the project's test directory (match existing structure)
- Use the project's test framework (Google Test, Catch2, etc.)
- One test file per source module
- Use descriptive test names: `TEST(ModuleName, DescribesExpectedBehavior)`
- Test both success and failure paths

### TypeScript / React
- Place tests adjacent to source: `Component.test.tsx` next to `Component.tsx`
- Use React Testing Library for component tests
- Use `describe` / `it` blocks with readable descriptions
- Test user-visible behavior, not implementation details
- Mock external dependencies (API calls, timers)

### Go
- Place tests in `_test.go` files in the same package
- Use table-driven tests for functions with multiple input/output cases
- Test exported functions; only test unexported if complex logic warrants it
- Use `t.Helper()` in test utility functions

## Step 4: Test Categories

For each test target, write tests in this priority order:

### Happy Path (mandatory)
- Each public function/method with typical valid inputs
- Expected output verified
- At least one test per public interface

### Edge Cases (mandatory)
- Empty/nil/zero inputs
- Single-element collections
- Boundary values
- Maximum/minimum valid inputs

### Error Conditions (mandatory for functions that can fail)
- Invalid inputs produce appropriate errors (not panics)
- Error messages are meaningful
- Error types are correct
- Partial failure doesn't corrupt state

### Integration (if multiple components interact)
- Data flows correctly between components
- Component A's output is valid input for component B
- Error in one component is handled by the caller

## Step 5: Validate

1. Run the full test suite: all new tests pass
2. Verify existing tests still pass (no regressions introduced)
3. Check that tests actually test something meaningful:
   - No tautological assertions (assert true == true)
   - No tests that pass regardless of implementation
   - Each test would FAIL if the behavior it tests were broken

## Step 6: Commit

```bash
git add <test files>
git commit -m "test: add tests for <module/feature>

Coverage:
- <N> unit tests
- <N> edge case tests
- <N> error condition tests
- <N> integration tests (if applicable)"
```

## CRITICAL RULES

- Do NOT modify implementation code — only create/modify test files
- Do NOT test private/internal implementation details unless they contain
  complex logic that warrants direct testing
- If a function is difficult to test in isolation, note it as a design
  smell in a comment but write the best test you can
- Match the project's existing test patterns exactly
