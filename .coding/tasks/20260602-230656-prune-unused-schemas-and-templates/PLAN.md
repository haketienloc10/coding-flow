# Code Plan

## 1. Objective

Remove unused schemas and templates while preserving the two agent output schemas used by CLI commands.

## 2. Scope

### In Scope

- Inline packet verify and packet ship Markdown renderers in Rust.
- Delete the templates directory.
- Delete schema files not read by CLI runtime.
- Update docs that referenced templates/* or schemas/* broadly.

### Out of Scope

- Changing plan/coding agent schema behavior.
- Changing Rust validation logic for request, verify, ship, packet, problem, or decision inputs.

## 3. Requirements

- schemas/plan.schema.json and schemas/coding.schema.json must remain.
- templates/ must be removed.
- No source should include templates files.
- cargo test must pass.

## 4. Technical Approach

- Replace render_template(include_str!(...)) calls with direct format! renderers for packet verify and packet ship.
- Remove the render_template helper once unused.
- Delete unused schema files and all template files.
- Search for stale references and run tests.

## 5. Files to Change

- src/lib.rs
- US-update-packet-workflow.md
- schemas/
- templates/

## 6. Implementation Steps

- [done] Inline packet verify and packet ship renderers in src/lib.rs.
- [done] Remove templates/ and unused schema files.
- [done] Update stale documentation references.
- [done] Run formatter, reference checks, and cargo test.

## 7. Test Plan

### Planned

- cargo fmt
- cargo test
- test ! -d templates
- find schemas -maxdepth 1 -type f -print | sort
- rg for template includes and removed schema names

### Result

- cargo fmt passed.
- cargo test passed: 38 tests passed.
- templates directory is absent.
- schemas contains only coding.schema.json and plan.schema.json.
- No stale template or removed-schema references were found.

## 8. Risks

- Standalone packet templates are no longer editable outside Rust code.

## 9. Done Criteria

### Criteria

- Only the two agent schemas remain under schemas/.
- templates/ no longer exists.
- Packet verify and packet ship rendering still pass tests.
- cargo test passes.

### Verified

- Only the two agent schemas remain under schemas/.
- templates/ no longer exists.
- Packet verify and packet ship rendering still pass tests.
- cargo test passes.
