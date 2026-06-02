Bạn đang sửa project `coding-flow-v0.4`.

Mục tiêu: nâng cấp lên `coding-flow-v0.5` để hỗ trợ loop giữa coding và verify.

## Problem

Hiện tại sau khi `agent coding` tạo `CODING.md`, nếu verify phát hiện lỗi hoặc lệch acceptance criteria thì main context có xu hướng tự sửa code trực tiếp.

Điều này sai với mục tiêu giữ main context sạch.

Workflow mới phải đảm bảo:

```text
CODING.md -> VERIFY.md -> nếu có findings thì quay lại agent coding --fix -> VERIFY.md lại
```

Main context không được tự implement khi verify failed.

---

## Core Rules

1. Main context không được tự sửa code khi verify fail.
2. Verify phase chỉ được kiểm tra và ghi findings vào `VERIFY.md`.
3. Coding agent là bên duy nhất được fix code trong loop.
4. Nếu `VERIFY.md` có status `failed` hoặc `partial`, next action phải là:

```bash
cflow agent coding --task current --fix
```

5. Không ship nếu verify chưa `passed`.
6. Không commit nếu user chưa yêu cầu rõ.
7. Không lưu JSON trong `.coding/`.
8. JSON vẫn chỉ là transient stdin/stdout.
9. Markdown artifact vẫn là source of truth.

---

## Artifact structure

Giữ cấu trúc hiện tại:

```text
.coding/tasks/<task-id>/
├── REQUEST.md
├── PLAN.md
├── CODING.md
├── VERIFY.md
└── SHIP.md
```

Không thêm JSON files.

---

## Update VERIFY schema

Update verify transient JSON shape thành:

```json
{
  "status": "failed",
  "checks": [],
  "manual_checks": [],
  "acceptance_criteria_checked": [],
  "findings": [
    {
      "id": "F001",
      "severity": "high",
      "type": "acceptance_mismatch",
      "expected": "",
      "actual": "",
      "evidence": "",
      "suggested_fix": ""
    }
  ],
  "known_issues": [],
  "done_criteria_verified": []
}
```

Allowed `status`:

```text
passed
failed
partial
skipped
```

Allowed `findings[].severity`:

```text
low
medium
high
blocking
```

Allowed `findings[].type`:

```text
acceptance_mismatch
test_failure
runtime_error
copy_mismatch
ui_mismatch
regression
missing_behavior
other
```

Validation rules:

* If `status = passed`, findings should be empty.
* If `status = failed` or `partial`, findings must not be empty.
* Each finding must have `id`, `severity`, `type`, `expected`, `actual`, `evidence`.
* `suggested_fix` is optional but recommended.

---

## Update VERIFY.md template

Render `VERIFY.md` with findings.

Required sections:

```md
# Verify

## Status

{{status}}

## Automated Checks

{{checks}}

## Manual Checks

{{manual_checks}}

## Acceptance Criteria Checked

{{acceptance_criteria_checked}}

## Findings

{{findings}}

## Known Issues

{{known_issues}}

## Done Criteria Verified

{{done_criteria_verified}}
```

Finding render format:

```md
- [F001] acceptance_mismatch
  - Severity: high
  - Expected: ...
  - Actual: ...
  - Evidence: ...
  - Suggested fix: ...
```

---

## Update CODING schema

Update coding transient JSON shape thành:

```json
{
  "mode": "fix",
  "status": "ready_for_verify",
  "summary": [],
  "fixed_findings": [],
  "changed_files": [],
  "notes": [],
  "next": "verify"
}
```

Allowed `mode`:

```text
initial
fix
```

Allowed `status`:

```text
ready_for_verify
blocked
partial
failed
```

Allowed `next`:

```text
verify
plan
clarify
none
```

Validation rules:

* If mode is `fix`, `fixed_findings` should reference finding ids from latest `VERIFY.md`.
* If status is `ready_for_verify`, next should normally be `verify`.

---

## Update CODING.md template

Render `CODING.md` as:

```md
# Coding

## Mode

{{mode}}

## Status

{{status}}

## Summary

{{summary}}

## Fixed Findings

{{fixed_findings}}

## Changed Files

{{changed_files}}

## Notes

{{notes}}

## Next

{{next}}
```

---

## Add `--fix` support to `cflow agent coding`

Command:

```bash
bin/cflow agent coding --task current --fix
```

Behavior:

1. Resolve task.
2. Require `PLAN.md`.
3. If `--fix` is present, require `VERIFY.md`.
4. Read `PLAN.md`.
5. Read latest `VERIFY.md`.
6. Read `CODING.md` if it exists.
7. Build prompt for coding agent.
8. Prompt must tell agent:

   * do not re-plan;
   * do not broaden scope;
   * fix only findings from `VERIFY.md`;
   * preserve already-correct work;
   * return coding JSON only;
   * do not edit `.coding/tasks/<task-id>/*.md`;
   * do not verify;
   * do not ship;
   * do not commit.
9. Parse returned JSON.
10. Validate coding JSON.
11. Render `CODING.md`.
12. Print short summary only.

Short stdout:

```text
Coding fix completed: .coding/tasks/<task-id>/CODING.md
Mode: fix
Fixed findings: <count>
Status: <status>
Next: cflow verify --task current
```

Do not dump full agent output.

Do not dump diff.

Do not dump full CODING.md.

---

## Update `skills/agent-coding.md`

Update the skill to support two modes.

### Initial mode

Used by:

```bash
cflow agent coding --task current
```

Rules:

* Read PLAN.md.
* Implement the planned code changes.
* Return coding JSON with `mode: "initial"`.

### Fix mode

Used by:

```bash
cflow agent coding --task current --fix
```

Rules:

* Read PLAN.md.
* Read VERIFY.md.
* Read CODING.md if present.
* Fix only unresolved findings from VERIFY.md.
* Do not expand scope.
* Do not rewrite unrelated code.
* Do not re-plan.
* Do not edit markdown artifacts.
* Return coding JSON with `mode: "fix"`.
* Put fixed finding ids into `fixed_findings`.

Output JSON:

```json
{
  "mode": "fix",
  "status": "ready_for_verify",
  "summary": [],
  "fixed_findings": [],
  "changed_files": [],
  "notes": [],
  "next": "verify"
}
```

---

## Update `skills/verify.md`

Make verify stricter.

Rules:

* Verify against REQUEST.md, PLAN.md, and latest CODING.md.
* Do not modify source code.
* Do not fix bugs.
* Do not edit markdown artifacts directly.
* If behavior is wrong, produce verify JSON with `status: "failed"` or `partial`.
* Put concrete findings into `findings`.
* Each finding should include expected vs actual.
* Verify must not silently pass if acceptance criteria are missing.

Output JSON:

```json
{
  "status": "failed",
  "checks": [],
  "manual_checks": [],
  "acceptance_criteria_checked": [],
  "findings": [],
  "known_issues": [],
  "done_criteria_verified": []
}
```

---

## Update `AGENTS.md`

Add this section:

````md
## Coding / Verify Loop

If verification fails, do not fix code in the main context.

Use this loop:

```text
CODING.md -> VERIFY.md -> agent coding --fix -> VERIFY.md
````

Rules:

* Verify records findings only.
* Main does not implement fixes.
* Use `cflow agent coding --task current --fix` to fix findings.
* Re-run verify after each fix.
* Ship only when verify is `passed`.

````

---

## Update `cflow next`

If `cflow next` exists, update it.

Decision rules:

```text
REQUEST.md missing:
  next = request

PLAN.md missing:
  next = cflow agent plan --task current

CODING.md missing:
  next = cflow agent coding --task current

VERIFY.md missing:
  next = cflow verify --task current

VERIFY.md exists and status is failed or partial:
  next = cflow agent coding --task current --fix

VERIFY.md exists and status is skipped:
  next = cflow verify --task current

VERIFY.md exists and status is passed and SHIP.md missing:
  next = cflow ship --task current --dry-run

SHIP.md exists:
  next = done or commit pending
````

Important:

* Never suggest direct manual code edits after failed verify.
* Never suggest `--commit` unless user explicitly requested commit.

---

## Update `cflow status`

Show verify status and findings count if `VERIFY.md` exists.

Example:

```text
Current task: 20260602-153012-time-capsule-notes
Path: .coding/tasks/20260602-153012-time-capsule-notes
Artifacts:
- REQUEST.md: exists
- PLAN.md: exists
- CODING.md: exists
- VERIFY.md: exists
- SHIP.md: missing

Verify:
- Status: failed
- Findings: 4

Next:
- cflow agent coding --task current --fix
```

---

## Ship gate

Update ship behavior:

Reject ship if latest VERIFY.md indicates:

```text
status != passed
findings count > 0
```

Even if ship JSON claims verification.status is passed, CLI must inspect latest VERIFY.md and reject if artifact says failed/partial/skipped.

VERIFY.md is the source of truth.

---

## Manual test

After implementation, test this loop manually:

```bash
rm -rf .coding

bin/cflow new "time capsule notes"

cat examples/request.json | bin/cflow request --task current
cat examples/plan.json | bin/cflow plan --task current

cat <<'JSON' | bin/cflow coding --task current
{
  "mode": "initial",
  "status": "ready_for_verify",
  "summary": ["Implemented initial version"],
  "fixed_findings": [],
  "changed_files": ["src/App.tsx"],
  "notes": [],
  "next": "verify"
}
JSON

cat <<'JSON' | bin/cflow verify --task current
{
  "status": "failed",
  "checks": [],
  "manual_checks": ["Reviewed UI against acceptance criteria"],
  "acceptance_criteria_checked": [
    "Button copy must be Seal capsule",
    "Empty state must be No time capsules yet.",
    "Locked/Unlocked badge must be visible"
  ],
  "findings": [
    {
      "id": "F001",
      "severity": "high",
      "type": "copy_mismatch",
      "expected": "Button text is Seal capsule",
      "actual": "Button text is Start focus session",
      "evidence": "UI still contains Focus Garden copy",
      "suggested_fix": "Replace old Focus Garden copy with Time Capsule Notes copy"
    }
  ],
  "known_issues": [],
  "done_criteria_verified": []
}
JSON

bin/cflow status
bin/cflow next
```

Expected:

```text
Next: cflow agent coding --task current --fix
```

Then test coding fix render:

```bash
cat <<'JSON' | bin/cflow coding --task current
{
  "mode": "fix",
  "status": "ready_for_verify",
  "summary": ["Fixed Time Capsule Notes copy mismatch"],
  "fixed_findings": ["F001"],
  "changed_files": ["src/App.tsx"],
  "notes": [],
  "next": "verify"
}
JSON
```

Then verify passed:

```bash
cat <<'JSON' | bin/cflow verify --task current
{
  "status": "passed",
  "checks": [],
  "manual_checks": ["Reviewed UI against acceptance criteria"],
  "acceptance_criteria_checked": [
    "Button copy is Seal capsule",
    "Empty state is No time capsules yet.",
    "Locked/Unlocked badge is visible"
  ],
  "findings": [],
  "known_issues": [],
  "done_criteria_verified": [
    "Locked capsule does not reveal content",
    "Unlocked capsule shows content"
  ]
}
JSON

bin/cflow next
```

Expected:

```text
Next: cflow ship --task current --dry-run
```

---

## Deliverable

Report:

```text
- Files changed
- Commands added/changed
- How failed verify loops back to coding
- Commands tested
- Known limitations
```
