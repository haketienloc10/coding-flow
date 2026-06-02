# Ship

## Ready

true

## Commit Message

```text
fix(workflow): address validation template dx audit fixes
```

## Commit Body

- Add missing problem and decision schemas.
- Normalize agent output schemas and packet templates.
- Add packet flow examples and rendering tests.

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

## Summary

- Completed S01 validation/template/DX audit fixes.

## Verification

Status: passed

Source: .coding/packets/20260602-160700-workflow-audit-priority-fixes/stories/S01-validation-template-dx-fixes/VERIFY.md

## Notes

- Dry-run only; no git commit was created.
- P011 tracks that story ship dry-run requires explicit JSON input.
