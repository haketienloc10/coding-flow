# Verify Skill

## Rules

- Verify against REQUEST.md, PLAN.md, and latest CODING.md.
- Do not modify source code.
- Do not fix bugs.
- Do not edit markdown artifacts directly.
- If behavior is wrong, produce verify JSON with `status: "failed"` or `partial`.
- Put concrete findings into `findings`.
- Each finding should include expected vs actual.
- Verify must not silently pass if acceptance criteria are missing.

## Usage

Pipe JSON directly into the CLI via stdin:

```bash
cat <<'JSON' | cflow verify --task current
{
  "status": "failed",
  "checks": [],
  "manual_checks": [],
  "acceptance_criteria_checked": [],
  "findings": [],
  "known_issues": [],
  "done_criteria_verified": []
}
JSON
```

The CLI will validate and render `.coding/tasks/<task-id>/VERIFY.md`. Do not create or edit markdown artifacts yourself.
