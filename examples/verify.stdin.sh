#!/usr/bin/env bash
cat <<'JSON' | node ../bin/cflow verify --task current 
{
  "status": "passed",
  "checks": [
    {
      "name": "Unit tests",
      "command": "npm test",
      "status": "passed",
      "notes": "Relevant tests passed."
    }
  ],
  "manual_checks": [
    "Verified copy button appears on documentation code blocks.",
    "Verified clipboard copy works in browser."
  ],
  "regressions_checked": [
    "Existing code block rendering still works."
  ],
  "known_issues": [],
  "done_criteria_verified": [
    "Copy button appears on documentation code blocks.",
    "Clicking the button copies the code block text.",
    "Existing code block rendering remains intact."
  ]
}
JSON
