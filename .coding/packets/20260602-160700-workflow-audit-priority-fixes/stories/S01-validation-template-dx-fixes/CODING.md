# Coding

## Mode

initial

## Status

ready_for_verify

## Summary

- Added problem and decision JSON schemas for knowledge command inputs.
- Normalized coding and plan schemas for draft 2020-12 metadata and provider response_format compatibility.
- Added packet verify/ship templates and switched packet renderers to those templates.
- Added packet flow example JSON files and tests that validate/render them.
- Covered readable verify check rendering with a regression test.
- Changed bin/cflow to prefer target/debug before target/release to avoid stale release binaries during local development.
- Added workflow state guard around packet-create tests so cargo test no longer leaves .coding state or PKT-0001 artifacts behind.

## Fixed Findings

- _None_

## Changed Files

- bin/cflow
- schemas/coding.schema.json
- schemas/plan.schema.json
- schemas/problem.schema.json
- schemas/decision.schema.json
- templates/packet_verify.md
- templates/packet_ship.md
- examples/intake.json
- examples/packet.json
- examples/stories.json
- examples/packet_verify.json
- examples/packet_ship.json
- src/main.rs

## Notes

- story agent plan/coding failed before implementation because existing schemas were rejected by Codex response_format; fallback recorded in P008/P009 and D-0002.
- cargo test exposed workflow state mutation from packet-create tests; P010 records the issue and this story adds a guard-based fix.

## Next

verify
