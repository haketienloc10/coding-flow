#!/usr/bin/env bash
cat <<'JSON' | node ../bin/cflow plan --task current 
{
  "objective": "Add a lightweight copy button to documentation code blocks.",
  "scope": {
    "in": [
      "Show a copy button on code blocks in documentation pages.",
      "Copy code block content to clipboard.",
      "Show basic success/failure feedback."
    ],
    "out": [
      "No backend changes.",
      "No syntax highlighting redesign.",
      "No analytics tracking."
    ]
  },
  "requirements": [
    "The copy button must not break existing code block rendering.",
    "Clipboard copy must work in supported browsers.",
    "The implementation should degrade safely when clipboard API is unavailable."
  ],
  "technical_approach": [
    "Add a small UI component for code block copy actions.",
    "Use navigator.clipboard when available.",
    "Keep state local to each code block."
  ],
  "files_to_change": [
    "src/components/CodeBlock.tsx"
  ],
  "implementation_steps": [
    {
      "text": "Inspect current code block rendering path.",
      "status": "todo"
    },
    {
      "text": "Add copy button UI and clipboard handler.",
      "status": "todo"
    },
    {
      "text": "Add basic tests or manual verification notes.",
      "status": "todo"
    }
  ],
  "test_plan": {
    "planned": [
      "Verify copy works for a simple code block.",
      "Verify fallback behavior when clipboard API is unavailable.",
      "Verify existing code block layout remains acceptable."
    ],
    "result": []
  },
  "risks": [
    "Clipboard API behavior may differ across browsers."
  ],
  "done_criteria": {
    "items": [
      "Copy button appears on documentation code blocks.",
      "Clicking the button copies the code block text.",
      "Existing code block rendering remains intact."
    ],
    "verified": []
  }
}
JSON
