pub const REQUEST_TYPES: &[&str] = &[
    "question",
    "unclear",
    "investigation",
    "new_feature",
    "bug_fix",
    "refactor",
    "maintenance",
    "documentation",
    "test_only",
];

pub const REQUEST_LANES: &[&str] = &["none", "needs_clarification", "tiny", "normal", "high_risk"];

pub const REQUEST_NEXT_ACTIONS: &[&str] = &["answer", "clarify", "investigate", "plan", "none"];

pub const STEP_STATUSES: &[&str] = &["todo", "in_progress", "done", "blocked"];

pub const CODING_MODES: &[&str] = &["initial", "fix"];

pub const CODING_STATUSES: &[&str] = &["ready_for_verify", "blocked", "partial", "failed"];

pub const CODING_NEXT_ACTIONS: &[&str] = &["verify", "plan", "clarify", "none"];

pub const VERIFY_STATUSES: &[&str] = &["passed", "failed", "partial", "skipped"];

pub const FINDING_SEVERITIES: &[&str] = &["low", "medium", "high", "blocking"];

pub const FINDING_TYPES: &[&str] = &[
    "acceptance_mismatch",
    "test_failure",
    "runtime_error",
    "copy_mismatch",
    "ui_mismatch",
    "regression",
    "missing_behavior",
    "other",
];

pub const COMMIT_TYPES: &[&str] = &[
    "feat", "fix", "refactor", "docs", "test", "chore", "ci", "build", "perf",
];

pub const PROBLEMS_PATH: &str = ".coding/knowledge/PROBLEMS.md";

pub const DECISIONS_PATH: &str = ".coding/knowledge/decisions.md";

pub const PROBLEM_STATUSES: &[&str] = &["open", "resolved", "cancelled"];

pub const DECISION_STATUSES: &[&str] = &["proposed", "accepted", "rejected", "superseded"];

pub const PROBLEM_SEVERITIES: &[&str] = &["low", "medium", "high", "blocking"];

pub const PROBLEM_PHASES: &[&str] = &[
    "request",
    "intake",
    "plan",
    "coding",
    "coding_fix",
    "verify",
    "ship",
    "state",
    "agent",
    "workflow",
    "unknown",
];
