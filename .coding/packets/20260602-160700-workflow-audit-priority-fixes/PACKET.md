# Packet

## Goal

Address the workflow audit priority summary with controlled, reviewable changes that improve validation, templates, examples, state consistency, and documented follow-up paths for larger architecture items.

## Scope

### In Scope

- Add missing schemas for problem and decision inputs
- Normalize schema metadata where currently inconsistent
- Add missing packet verify/ship templates and packet flow examples
- Fix markdown template rendering for object arrays where feasible
- Improve bin/cflow launcher freshness behavior
- Repair or document current workflow state inconsistencies using CLI-managed operations
- Split large architecture items into follow-up stories

### Out of Scope

- Full src/main.rs modularization in the first implementation story
- Full task/packet model unification in the first implementation story
- Manual edits to .coding markdown artifacts

## Global Acceptance Criteria

- Current story changes compile and pass cargo test
- Added schemas/templates/examples are present and valid JSON/Markdown
- Launcher no longer prefers stale release binaries during local development
- Workflow state is not made less consistent
- Large P0/P1 architecture items remain traceable as stories or decisions

## Technical Constraints

- Use cflow commands for .coding knowledge/workflow artifacts
- Do not edit .coding markdown artifacts directly
- Keep source edits scoped to the current story
- Preserve existing CLI behavior unless explicitly fixing an audited defect

## Shared Data / Contracts

- JSON schema files use draft 2020-12
- Transient JSON examples match their schemas
- Problem and decision command input contracts are represented as schemas

## Validation Strategy

- cargo fmt --check
- cargo test
- cargo build --release
- CLI smoke tests for impacted commands
- Manual inspection of generated/rendered markdown where relevant
