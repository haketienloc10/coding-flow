# Code Plan

## 1. Objective

Remove the examples directory and eliminate repository dependencies on examples/*.

## 2. Scope

### In Scope

- Remove examples/ files from the repository.
- Replace tests that used examples/* with inline fixtures.
- Update documentation that referenced examples/* paths.

### Out of Scope

- Changing CLI runtime validation behavior.
- Removing schemas/ or templates/.

## 3. Requirements

- cargo test must pass after examples/ is removed.
- No non-ignored source or docs should reference examples/ paths.
- Do not revert unrelated existing changes.

## 4. Technical Approach

- Move compact representative JSON fixtures into test helper functions in src/lib.rs.
- Patch docs to use generic input filenames instead of examples/*.
- Delete examples/ and verify with cargo test plus ripgrep.

## 5. Files to Change

- src/lib.rs
- README.md
- US-next-flow.md
- examples/

## 6. Implementation Steps

- [done] Replace include_str!(../examples/...) test fixtures with inline test helper fixtures.
- [done] Remove documentation references to examples/*.
- [done] Delete the examples directory.
- [done] Run cargo fmt, cargo test, and examples reference search.

## 7. Test Plan

### Planned

- cargo fmt
- cargo test
- rg -n "examples/|examples\b" . -g '!target' -g '!node_modules' -g '!vendor'
- test ! -d examples

### Result

- cargo fmt passed.
- cargo test passed: 38 tests passed.
- examples reference search returned no matches.
- examples directory is absent.

## 8. Risks

- Inline fixtures make src/lib.rs test module longer.

## 9. Done Criteria

### Criteria

- examples/ no longer exists.
- Tests do not include files from examples/.
- Docs do not point to examples/ paths.
- cargo test passes.

### Verified

- examples/ no longer exists.
- Tests do not include files from examples/.
- Docs do not point to examples/ paths.
- cargo test passes.
