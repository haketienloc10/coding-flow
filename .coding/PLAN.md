# Code Plan

## 1. Objective

Build a single-page web app MVP called Focus Garden that allows users to run focus sessions and visualize their completed/cancelled sessions as a virtual garden.

## 2. Scope

### In Scope

- Main UI with idle state, running state, and garden state
- Session duration selection (15, 25, or 45 mins)
- Goal input field
- Countdown timer
- Session completion logic (adds 'grown' tree)
- Session cancellation logic (adds 'withered' tree)
- Garden display using a simple grid
- LocalStorage persistence for garden data under key 'focus_garden_sessions'
- Clear garden button
- Basic UI layout

### Out of Scope

- Authentication
- Backend API
- Database
- Cross-device sync
- OS notifications
- Audio/sounds
- Advanced stats/charts
- Dark mode at MVP
- Perfect mobile responsiveness

## 3. Requirements

- Must be a single-page app (SPA).
- Must have 3 primary states: idle, running, complete/cancel.
- Must use local component state and standard hooks or vanilla JS modules.
- No external state management libraries.
- No large external packages for timers.
- Data model: FocusSession with id, goal, durationMinutes, status (grown/withered), startedAt, endedAt.

## 4. Technical Approach

- Initialize a new Vite project for a Vanilla JS app in a subfolder 'focus-garden'.
- Data persistence via a utility file (`storage.js`).
- Timer logic via a simple JS interval.
- CSS: Premium aesthetics in Vanilla CSS (glassmorphism, vibrant colors/gradients, smooth transitions).

## 5. Files to Change

- focus-garden/index.html
- focus-garden/src/main.js
- focus-garden/src/style.css

## 6. Implementation Steps

- [todo] Initialize Vite project in focus-garden directory
- [todo] Implement LocalStorage utilities (focus_garden_sessions key)
- [todo] Implement state variables and basic HTML structure
- [todo] Implement Timer & Session logic (Start, Cancel, Complete)
- [todo] Implement Garden UI rendering from LocalStorage
- [todo] Add premium styling (CSS) and animations

## 7. Test Plan

### Planned

- Start a session and verify timer counts down.
- Cancel session and verify withered tree appears.
- Complete session and verify grown tree appears.
- Reload page and verify items persist.
- Clear garden and verify storage/UI resets.

### Result

- _None_

## 8. Risks

- State desync between timer interval and UI rendering.

## 9. Done Criteria

### Criteria

- Goal input and duration selection work.
- Timer ticks accurately and automatically ends session when 0.
- Trees (grown/withered) are correctly added to garden upon session end.
- Data persists across reloads.
- Empty state handles properly.
- Clear garden works.

### Verified

- _None_
