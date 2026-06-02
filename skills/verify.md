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

For Tiny Flow, pipe JSON directly into the CLI via stdin:

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

For Story Flow, use the story command:

```bash
cat <<'JSON' | cflow story verify --story current
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

The CLI will validate and render `VERIFY.md`. Do not create or edit markdown artifacts yourself.
