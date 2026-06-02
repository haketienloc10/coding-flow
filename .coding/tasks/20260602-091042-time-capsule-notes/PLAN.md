# Code Plan

## 1. Objective

Add a lightweight copy button to documentation code blocks.

## 2. Scope

### In Scope

- Show a copy button on code blocks in documentation pages.
- Copy code block content to clipboard.
- Show basic success/failure feedback.

### Out of Scope

- No backend changes.
- No syntax highlighting redesign.
- No analytics tracking.

## 3. Requirements

- The copy button must not break existing code block rendering.
- Clipboard copy must work in supported browsers.
- The implementation should degrade safely when clipboard API is unavailable.

## 4. Technical Approach

- Add a small UI component for code block copy actions.
- Use navigator.clipboard when available.
- Keep state local to each code block.

## 5. Files to Change

- src/components/CodeBlock.tsx

## 6. Implementation Steps

- [todo] Inspect current code block rendering path.
- [todo] Add copy button UI and clipboard handler.
- [todo] Add basic tests or manual verification notes.

## 7. Test Plan

### Planned

- Verify copy works for a simple code block.
- Verify fallback behavior when clipboard API is unavailable.
- Verify existing code block layout remains acceptable.

### Result

- _None_

## 8. Risks

- Clipboard API behavior may differ across browsers.

## 9. Done Criteria

### Criteria

- Copy button appears on documentation code blocks.
- Clicking the button copies the code block text.
- Existing code block rendering remains intact.

### Verified

- _None_
