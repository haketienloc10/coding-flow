# Intake Skill

Pipe JSON directly into the CLI via stdin:

```bash
cat <<'JSON' | cflow packet intake --packet current
{
  "request_summary": "",
  "input_type": "new_feature",
  "lane": "normal",
  "risk_flags": [],
  "hard_gates": [],
  "split_required": true,
  "reason": "",
  "next_action": "packet_brief",
  "assumptions": [],
  "clarifying_questions": []
}
JSON
```

The CLI will validate and render `INTAKE.md`. Do not create or edit markdown artifacts yourself.
