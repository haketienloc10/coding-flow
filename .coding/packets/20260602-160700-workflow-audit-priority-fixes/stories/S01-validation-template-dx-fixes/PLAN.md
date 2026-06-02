# Code Plan

## 1. Objective

Implement scoped validation, template, example, rendering, and launcher fixes from the workflow audit S01 story.

## 2. Scope

### In Scope

- Add problem and decision JSON schemas matching current CLI inputs
- Normalize schema metadata and response_format compatibility for touched schemas
- Add packet verify and packet ship templates plus packet flow example JSON files
- Render verify checks arrays as readable markdown
- Change bin/cflow launcher priority to avoid stale release binary during development

### Out of Scope

- Full main.rs modularization
- State repair behavior changes
- Task/packet model unification

## 3. Requirements

- Preserve existing CLI command behavior
- Use draft 2020-12 schema URI consistently
- Examples must be valid JSON and align with existing schema contracts
- Template/rendering changes must not break current markdown output

## 4. Technical Approach

- Inspect existing validators and renderers in src/main.rs
- Add schema files and missing $schema/required fields where needed
- Add templates and examples using existing naming conventions
- Update render_verify to format checks object arrays as bullets
- Update bin/cflow binary selection order to prefer debug before release

## 5. Files to Change

- schemas/problem.schema.json
- schemas/decision.schema.json
- schemas/coding.schema.json
- schemas/plan.schema.json
- templates/packet_verify.md
- templates/packet_ship.md
- examples/intake.json
- examples/packet.json
- examples/stories.json
- examples/packet_verify.json
- examples/packet_ship.json
- templates/verify.md
- src/main.rs
- bin/cflow

## 6. Implementation Steps

- [todo] Inspect existing schema validators and render helpers
- [todo] Add and normalize schema/template/example files
- [todo] Patch verify rendering and launcher binary priority
- [todo] Run formatting, tests, release build, and smoke checks

## 7. Test Plan

### Planned

- cargo fmt --check
- cargo test
- cargo build --release
- bin/cflow story status
- smoke render packet verify/ship with example JSON if supported

### Result

- _None_

## 8. Risks

- plan.schema.json compatibility fix may affect agent response validation
- launcher priority change may alter user expectations when both debug and release exist

## 9. Done Criteria

### Criteria

- All S01 acceptance criteria are met or explicitly recorded as blocked
- Verification commands pass
- No direct manual edits are made to .coding markdown artifacts

### Verified

- _None_
