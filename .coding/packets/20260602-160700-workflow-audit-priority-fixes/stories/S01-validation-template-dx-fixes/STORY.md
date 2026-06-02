# Story: Validation/template/DX audit fixes

## Description

Implement scoped audit fixes for missing problem/decision schemas, schema metadata normalization, packet verify/ship templates, packet flow examples, object-array template rendering, and launcher stale binary priority.

## Acceptance Criteria

- problem.schema.json and decision.schema.json exist and document current CLI input contracts
- coding.schema.json has a draft 2020-12 $schema like the other schemas
- packet_verify.md and packet_ship.md templates exist and are used by packet renderers where applicable
- examples include intake.json, packet.json, stories.json, packet_verify.json, and packet_ship.json
- verify template object arrays render readable markdown instead of debug/object placeholders
- bin/cflow avoids stale release binary priority during dev smoke tests
- cargo fmt --check, cargo test, and cargo build --release pass

## Files to Change

- schemas/
- templates/
- examples/
- bin/cflow
- src/main.rs
