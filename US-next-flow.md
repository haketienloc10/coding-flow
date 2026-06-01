Add workflow orchestration commands so ship does not depend on skill auto-triggering.

Implement:

1. cflow next --task current

Behavior:
- Resolve current task.
- Inspect artifacts in task folder.
- Determine next safe command.

Rules:
- If REQUEST.md missing: next = request
- Else if PLAN.md missing: next = agent plan
- Else if CODING.md missing: next = agent coding
- Else if VERIFY.md missing: next = verify
- Else if VERIFY.md exists and status is passed and SHIP.md missing: next = ship --dry-run
- Else if SHIP.md exists: next = done or commit pending
- If VERIFY.md status is failed/partial/skipped: next = fix or verify again
- Never suggest --commit unless user explicitly asks to commit.

Output must be short:
Next: <command>
Reason: <one sentence>

2. cflow run --task current

Behavior:
- Run only safe automatic steps.
- May run:
  - agent plan
  - agent coding
  - ship --dry-run
- Must not run:
  - ship --commit
- Must stop when human input is needed:
  - request JSON needed
  - verify JSON needed
  - failed/partial verification
  - commit decision needed

Important:
- Do not rely on skills auto-triggering.
- Skills are instruction files only.
- cflow owns workflow orchestration.
- Main context should call cflow next or cflow run instead of expecting ship skill to run automatically.