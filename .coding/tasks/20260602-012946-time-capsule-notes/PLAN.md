# Code Plan

## 1. Objective

Build the Time Capsule Notes MVP in the existing focus-garden local single-page app, allowing users to create notes with a future unlock time, keep locked note content hidden, reveal content after unlock, persist notes in localStorage using the key time_capsule_notes, delete notes, and verify the core flows.

## 2. Scope

### In Scope

- Add a Time Capsule Notes UI to the existing focus-garden single-page app
- Create notes with title/content and a future unlock date/time
- Store and load notes from browser localStorage using key time_capsule_notes
- Hide content for locked notes and show locked state metadata
- Reveal content automatically when notes are unlocked based on native Date comparisons
- Delete saved notes
- Verify create, persistence, locked, unlocked, and delete flows

### Out of Scope

- Backend services
- Authentication or user accounts
- Database integration
- Heavy date-time libraries
- Cross-device sync
- Encryption or security-grade secrecy
- Complex notification/reminder behavior

## 3. Requirements

- Use localStorage key time_capsule_notes
- Use native Date APIs for time comparison and formatting
- Do not add backend, auth, database, or heavy date-time packages
- Locked notes must not display their content before unlock time
- Unlocked notes must display their content once current time is at or after unlock time
- Notes must survive page reloads through localStorage persistence
- Users must be able to delete notes
- The UI must handle empty state and invalid form input
- Verification must pass before ship

## 4. Technical Approach

- Inspect the focus-garden app structure to identify the SPA entry point, component patterns, styling approach, and available build/test scripts.
- Model each note with a stable id, title, content, unlockAt timestamp/string, and createdAt timestamp.
- Create localStorage helpers for reading, validating, writing, and deleting notes under time_capsule_notes, with graceful fallback for malformed stored data.
- Build or integrate a Time Capsule Notes component containing a controlled form, notes list, lock/unlock rendering logic, and delete actions.
- Use Date.now() or new Date() comparisons against each note unlock time to determine locked state.
- Refresh time-based UI state on load and with a lightweight interval so notes can unlock while the page remains open.
- Keep styling consistent with the existing focus-garden app and avoid unrelated layout or design rewrites.
- Run existing build/test/lint commands as available, then manually verify localStorage and time-based behavior in the browser if a dev server is available.

## 5. Files to Change

- focus-garden app entry/component files discovered during implementation
- focus-garden styling files discovered during implementation
- Optional focused helper/module for time capsule localStorage behavior if the app structure supports it
- Optional focused tests if an existing test setup is discovered

## 6. Implementation Steps

- [todo] Inspect focus-garden package scripts, app structure, existing components, and styling conventions.
- [todo] Identify the correct SPA integration point for the Time Capsule Notes MVP.
- [todo] Implement note data handling with localStorage key time_capsule_notes, including safe load/save/delete behavior.
- [todo] Implement the note creation form with validation for required fields and future unlock time.
- [todo] Implement note list rendering that hides locked content and reveals unlocked content based on native Date comparisons.
- [todo] Add delete behavior and empty state handling.
- [todo] Add or update focused tests if the repository has a suitable existing test setup.
- [todo] Run available verification commands such as build, lint, or tests.
- [todo] Manually verify create, locked, unlocked, persistence after reload, and delete flows.

## 7. Test Plan

### Planned

- Run existing package verification scripts discovered in focus-garden, prioritizing test, lint, and build.
- Verify a note cannot be created without required content or a valid future unlock time.
- Verify a newly created future note is saved to localStorage under time_capsule_notes.
- Verify locked note content is hidden before unlock time.
- Verify note content is revealed when current time reaches or passes unlock time.
- Verify notes remain after page reload.
- Verify deleting a note removes it from UI and localStorage.
- Verify malformed localStorage data does not crash the app.

### Result

- _None_

## 8. Risks

- Existing focus-garden structure may not have a formal test setup, requiring manual browser verification.
- Time-based UI can become stale if the page does not re-render when unlock time passes.
- localStorage can contain malformed or older data and must be handled defensively.
- Date input timezone behavior may be confusing if browser local time is not handled consistently.
- Integrating into the existing app may require preserving unrelated user changes in a dirty worktree.

## 9. Done Criteria

### Criteria

- Users can create a time capsule note with content and a future unlock time.
- Locked notes do not show their hidden content before unlock time.
- Unlocked notes show their content at or after unlock time.
- Notes persist through reloads using localStorage key time_capsule_notes.
- Users can delete notes and deletion persists.
- No backend, auth, database, or heavy date-time package is introduced.
- Available automated verification commands pass, or any unavailable verification is explicitly documented.
- Core flows are manually verified if automated coverage is not available.

### Verified

- _None_
