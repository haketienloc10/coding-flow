use serde_json::Value;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::{self, Command, Stdio};

type CflowResult<T> = Result<T, String>;

const REQUEST_TYPES: &[&str] = &[
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

const REQUEST_LANES: &[&str] = &["none", "needs_clarification", "tiny", "normal", "high_risk"];

const REQUEST_NEXT_ACTIONS: &[&str] = &["answer", "clarify", "investigate", "plan", "none"];

const STEP_STATUSES: &[&str] = &["todo", "in_progress", "done", "blocked"];

const VERIFY_STATUSES: &[&str] = &["passed", "failed", "partial", "skipped"];

const COMMIT_TYPES: &[&str] = &[
    "feat", "fix", "refactor", "docs", "test", "chore", "ci", "build", "perf",
];

fn get_arg(args: &[String], name: &str, fallback: &str) -> String {
    let Some(index) = args.iter().position(|arg| arg == name) else {
        return fallback.to_string();
    };

    let Some(value) = args.get(index + 1) else {
        return fallback.to_string();
    };

    if value.starts_with("--") {
        fallback.to_string()
    } else {
        value.clone()
    }
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|arg| arg == name)
}

fn ensure_dir(file_path: &str) -> CflowResult<()> {
    let path = Path::new(file_path);
    let Some(dir) = path.parent() else {
        return Ok(());
    };

    if dir.as_os_str().is_empty() || dir == Path::new(".") {
        return Ok(());
    }

    fs::create_dir_all(dir).map_err(|error| error.to_string())
}

fn read_json_input(args: &[String]) -> CflowResult<Value> {
    let input_opt = args.iter().position(|arg| arg == "--input").and_then(|idx| args.get(idx + 1));
    let content = if let Some(input_file) = input_opt {
        if !Path::new(input_file).exists() {
            return Err(format!("Input file not found: {}", input_file));
        }
        fs::read_to_string(input_file).map_err(|error| error.to_string())?
    } else {
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).map_err(|error| error.to_string())?;
        if buffer.trim().is_empty() {
            return Err("No input provided via stdin. Use --input or pipe JSON.".to_string());
        }
        buffer
    };
    serde_json::from_str(&content).map_err(|error| format!("Invalid JSON: {}", error))
}

fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '-')
        .map(|c| if c.is_whitespace() { '-' } else { c })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn resolve_task(args: &[String]) -> CflowResult<String> {
    let task = get_arg(args, "--task", "current");
    if task == "current" {
        if !Path::new(".coding/current").exists() {
            return Err("No current task. Run `cflow new \"<task-name>\"` first.".to_string());
        }
        let current_task = fs::read_to_string(".coding/current").map_err(|e| e.to_string())?;
        let current_task = current_task.trim();
        Ok(format!(".coding/{}", current_task))
    } else if task.starts_with(".coding/tasks/") {
        Ok(task)
    } else {
        Ok(format!(".coding/tasks/{}", task))
    }
}


fn write_text(file_path: &str, content: &str) -> CflowResult<()> {
    ensure_dir(file_path)?;
    fs::write(file_path, content).map_err(|error| error.to_string())
}

fn get_path<'a>(value: &'a Value, path: &[&str]) -> Option<&'a Value> {
    let mut current = value;
    for key in path {
        current = current.get(*key)?;
    }
    Some(current)
}

fn is_empty(value: Option<&Value>) -> bool {
    match value {
        None | Some(Value::Null) => true,
        Some(Value::String(text)) => text.trim().is_empty(),
        Some(Value::Array(items)) => items.is_empty(),
        Some(Value::Object(map)) => map.is_empty(),
        Some(_) => false,
    }
}

fn is_truthy(value: Option<&Value>) -> bool {
    match value {
        None | Some(Value::Null) => false,
        Some(Value::Bool(value)) => *value,
        Some(Value::Number(number)) => number.as_f64().is_some_and(|value| value != 0.0),
        Some(Value::String(text)) => !text.is_empty(),
        Some(Value::Array(_)) | Some(Value::Object(_)) => true,
    }
}

fn value_to_js_string(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        Value::String(value) => value.clone(),
        Value::Array(items) => items
            .iter()
            .map(value_to_js_string)
            .collect::<Vec<_>>()
            .join(","),
        Value::Object(_) => "[object Object]".to_string(),
    }
}

fn option_to_js_string(value: Option<&Value>) -> String {
    value
        .map(value_to_js_string)
        .unwrap_or_else(|| "undefined".to_string())
}

fn require_field(value: Option<&Value>, name: &str) -> CflowResult<()> {
    if is_empty(value) {
        Err(format!("Missing or empty required field: {name}"))
    } else {
        Ok(())
    }
}

fn assert_allowed(value: Option<&Value>, allowed: &[&str], name: &str) -> CflowResult<()> {
    let actual = option_to_js_string(value);
    if allowed.contains(&actual.as_str()) {
        Ok(())
    } else {
        Err(format!(
            "Invalid {name}: {actual}. Allowed: {}",
            allowed.join(", ")
        ))
    }
}

fn list(items: Option<&Value>) -> String {
    let Some(Value::Array(items)) = items else {
        return "- _None_".to_string();
    };

    if items.is_empty() {
        return "- _None_".to_string();
    }

    items
        .iter()
        .map(|item| match item {
            Value::String(text) => format!("- {text}"),
            Value::Object(map) => {
                if map.contains_key("name") && map.contains_key("status") {
                    let command = if is_truthy(map.get("command")) {
                        format!(
                            "\n  - Command: `{}`",
                            option_to_js_string(map.get("command"))
                        )
                    } else {
                        String::new()
                    };
                    let notes = if is_truthy(map.get("notes")) {
                        format!("\n  - Notes: {}", option_to_js_string(map.get("notes")))
                    } else {
                        String::new()
                    };

                    format!(
                        "- {}: {}{command}{notes}",
                        option_to_js_string(map.get("name")),
                        option_to_js_string(map.get("status"))
                    )
                } else if map.contains_key("text") && map.contains_key("status") {
                    format!(
                        "- [{}] {}",
                        option_to_js_string(map.get("status")),
                        option_to_js_string(map.get("text"))
                    )
                } else {
                    format!(
                        "- {}",
                        serde_json::to_string(item).unwrap_or_else(|_| value_to_js_string(item))
                    )
                }
            }
            _ => format!("- {}", value_to_js_string(item)),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn validate_request(data: &Value) -> CflowResult<()> {
    require_field(get_path(data, &["summary"]), "summary")?;
    require_field(get_path(data, &["type"]), "type")?;
    require_field(get_path(data, &["lane"]), "lane")?;
    require_field(get_path(data, &["next_action"]), "next_action")?;

    assert_allowed(get_path(data, &["type"]), REQUEST_TYPES, "type")?;
    assert_allowed(get_path(data, &["lane"]), REQUEST_LANES, "lane")?;
    assert_allowed(
        get_path(data, &["next_action"]),
        REQUEST_NEXT_ACTIONS,
        "next_action",
    )?;

    if matches!(
        get_path(data, &["planning_needed"]),
        Some(Value::Bool(true))
    ) && option_to_js_string(get_path(data, &["next_action"])) != "plan"
    {
        return Err("Invalid request: planning_needed=true requires next_action=plan".to_string());
    }

    if option_to_js_string(get_path(data, &["lane"])) == "needs_clarification"
        && is_empty(get_path(data, &["clarifying_questions"]))
    {
        return Err(
            "Invalid request: needs_clarification requires clarifying_questions".to_string(),
        );
    }

    Ok(())
}

fn validate_plan(data: &Value) -> CflowResult<()> {
    require_field(get_path(data, &["objective"]), "objective")?;
    require_field(get_path(data, &["scope"]), "scope")?;
    require_field(get_path(data, &["scope", "in"]), "scope.in")?;
    require_field(get_path(data, &["scope", "out"]), "scope.out")?;
    require_field(get_path(data, &["requirements"]), "requirements")?;
    require_field(
        get_path(data, &["technical_approach"]),
        "technical_approach",
    )?;
    require_field(
        get_path(data, &["done_criteria", "items"]),
        "done_criteria.items",
    )?;

    if let Some(Value::Array(steps)) = get_path(data, &["implementation_steps"]) {
        for (index, step) in steps.iter().enumerate() {
            require_field(
                step.get("text"),
                &format!("implementation_steps[{index}].text"),
            )?;
            require_field(
                step.get("status"),
                &format!("implementation_steps[{index}].status"),
            )?;
            assert_allowed(
                step.get("status"),
                STEP_STATUSES,
                &format!("implementation_steps[{index}].status"),
            )?;
        }
    }

    Ok(())
}

fn validate_verify(data: &Value) -> CflowResult<()> {
    require_field(get_path(data, &["status"]), "status")?;
    assert_allowed(get_path(data, &["status"]), VERIFY_STATUSES, "status")
}

fn validate_ship(data: &Value) -> CflowResult<()> {
    require_field(get_path(data, &["ready"]), "ready")?;
    require_field(get_path(data, &["commit"]), "commit")?;
    require_field(get_path(data, &["commit", "type"]), "commit.type")?;
    require_field(get_path(data, &["commit", "message"]), "commit.message")?;
    require_field(
        get_path(data, &["verification", "status"]),
        "verification.status",
    )?;

    assert_allowed(
        get_path(data, &["commit", "type"]),
        COMMIT_TYPES,
        "commit.type",
    )?;

    if !matches!(get_path(data, &["ready"]), Some(Value::Bool(true))) {
        return Err("Ship rejected: ready must be true".to_string());
    }

    if option_to_js_string(get_path(data, &["verification", "status"])) != "passed" {
        return Err("Ship rejected: verification.status must be passed".to_string());
    }

    Ok(())
}

fn render_request(data: &Value) -> String {
    let planning_needed = if matches!(
        get_path(data, &["planning_needed"]),
        Some(Value::Bool(true))
    ) {
        "true"
    } else {
        "false"
    };

    format!(
        "# Request Intake\n\n## Summary\n\n{}\n\n## Type\n\n{}\n\n## Planning Needed\n\n{}\n\n## Lane\n\n{}\n\n## Risk Flags\n\n{}\n\n## Hard Gates\n\n{}\n\n## Assumptions\n\n{}\n\n## Clarifying Questions\n\n{}\n\n## Next Action\n\n{}\n",
        option_to_js_string(get_path(data, &["summary"])),
        option_to_js_string(get_path(data, &["type"])),
        planning_needed,
        option_to_js_string(get_path(data, &["lane"])),
        list(get_path(data, &["risk_flags"])),
        list(get_path(data, &["hard_gates"])),
        list(get_path(data, &["assumptions"])),
        list(get_path(data, &["clarifying_questions"])),
        option_to_js_string(get_path(data, &["next_action"]))
    )
}

fn render_plan(data: &Value) -> String {
    format!(
        "# Code Plan\n\n## 1. Objective\n\n{}\n\n## 2. Scope\n\n### In Scope\n\n{}\n\n### Out of Scope\n\n{}\n\n## 3. Requirements\n\n{}\n\n## 4. Technical Approach\n\n{}\n\n## 5. Files to Change\n\n{}\n\n## 6. Implementation Steps\n\n{}\n\n## 7. Test Plan\n\n### Planned\n\n{}\n\n### Result\n\n{}\n\n## 8. Risks\n\n{}\n\n## 9. Done Criteria\n\n### Criteria\n\n{}\n\n### Verified\n\n{}\n",
        option_to_js_string(get_path(data, &["objective"])),
        list(get_path(data, &["scope", "in"])),
        list(get_path(data, &["scope", "out"])),
        list(get_path(data, &["requirements"])),
        list(get_path(data, &["technical_approach"])),
        list(get_path(data, &["files_to_change"])),
        list(get_path(data, &["implementation_steps"])),
        list(get_path(data, &["test_plan", "planned"])),
        list(get_path(data, &["test_plan", "result"])),
        list(get_path(data, &["risks"])),
        list(get_path(data, &["done_criteria", "items"])),
        list(get_path(data, &["done_criteria", "verified"]))
    )
}

fn render_verify(data: &Value) -> String {
    format!(
        "# Verify\n\n## Status\n\n{}\n\n## Automated Checks\n\n{}\n\n## Manual Checks\n\n{}\n\n## Regressions Checked\n\n{}\n\n## Known Issues\n\n{}\n\n## Done Criteria Verified\n\n{}\n",
        option_to_js_string(get_path(data, &["status"])),
        list(get_path(data, &["checks"])),
        list(get_path(data, &["manual_checks"])),
        list(get_path(data, &["regressions_checked"])),
        list(get_path(data, &["known_issues"])),
        list(get_path(data, &["done_criteria_verified"]))
    )
}

fn render_ship(data: &Value) -> String {
    let scope = if is_truthy(get_path(data, &["commit", "scope"])) {
        format!(
            "({})",
            option_to_js_string(get_path(data, &["commit", "scope"]))
        )
    } else {
        String::new()
    };

    let source = if is_truthy(get_path(data, &["verification", "source"])) {
        option_to_js_string(get_path(data, &["verification", "source"]))
    } else {
        "_None_".to_string()
    };

    format!(
        "# Ship\n\n## Ready\n\n{}\n\n## Commit Message\n\n```text\n{}{}: {}\n```\n\n## Commit Body\n\n{}\n\n## Changed Files\n\n{}\n\n## Summary\n\n{}\n\n## Verification\n\nStatus: {}\n\nSource: {}\n\n## Notes\n\n{}\n",
        option_to_js_string(get_path(data, &["ready"])),
        option_to_js_string(get_path(data, &["commit", "type"])),
        scope,
        option_to_js_string(get_path(data, &["commit", "message"])),
        list(get_path(data, &["commit", "body"])),
        list(get_path(data, &["changed_files"])),
        list(get_path(data, &["summary"])),
        option_to_js_string(get_path(data, &["verification", "status"])),
        source,
        list(get_path(data, &["notes"]))
    )
}

fn is_git_repo() -> bool {
    Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

fn git<I, S>(args: I) -> CflowResult<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let args = args
        .into_iter()
        .map(|arg| arg.as_ref().to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    let status = Command::new("git")
        .args(&args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|error| error.to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("git command failed: git {}", args.join(" ")))
    }
}

fn command_new(args: &[String]) -> CflowResult<()> {
    let name = args.first().cloned().unwrap_or_default();
    if name.is_empty() {
        return Err("Missing task name. Usage: cflow new \"<task-name>\"".to_string());
    }
    
    let now = chrono::Local::now();
    let timestamp = now.format("%Y%m%d-%H%M%S").to_string();
    let slug = slugify(&name);
    let task_id = if slug.is_empty() { timestamp.clone() } else { format!("{}-{}", timestamp, slug) };
    
    let task_path = format!("tasks/{}", task_id);
    let full_path = format!(".coding/{}", task_path);
    
    ensure_dir(&format!("{}/.placeholder", full_path))?;
    
    write_text(".coding/current", &task_path)?;
    
    println!("Task created: {}", task_id);
    println!("Path: {}", full_path);
    
    Ok(())
}

fn command_request(args: &[String]) -> CflowResult<()> {
    let task_path = resolve_task(args)?;
    if !Path::new(&task_path).exists() {
        return Err(format!("Task folder does not exist: {}", task_path));
    }
    let output = format!("{}/REQUEST.md", task_path);
    let data = read_json_input(args)?;
    validate_request(&data)?;
    write_text(&output, &render_request(&data))?;
    println!("created {}", output);
    println!(
        "next_action={}",
        option_to_js_string(get_path(&data, &["next_action"]))
    );
    Ok(())
}

fn command_plan(args: &[String]) -> CflowResult<()> {
    let task_path = resolve_task(args)?;
    if !Path::new(&task_path).exists() {
        return Err(format!("Task folder does not exist: {}", task_path));
    }
    let output = format!("{}/PLAN.md", task_path);
    let data = read_json_input(args)?;
    validate_plan(&data)?;
    write_text(&output, &render_plan(&data))?;
    println!("created {}", output);
    Ok(())
}

fn command_verify(args: &[String]) -> CflowResult<()> {
    let task_path = resolve_task(args)?;
    if !Path::new(&task_path).exists() {
        return Err(format!("Task folder does not exist: {}", task_path));
    }
    let output = format!("{}/VERIFY.md", task_path);
    let data = read_json_input(args)?;
    validate_verify(&data)?;
    write_text(&output, &render_verify(&data))?;
    println!("created {}", output);
    println!(
        "status={}",
        option_to_js_string(get_path(&data, &["status"]))
    );
    Ok(())
}

fn command_ship(args: &[String]) -> CflowResult<()> {
    let task_path = resolve_task(args)?;
    if !Path::new(&task_path).exists() {
        return Err(format!("Task folder does not exist: {}", task_path));
    }
    let verify_path = format!("{}/VERIFY.md", task_path);
    if !Path::new(&verify_path).exists() {
        return Err("Ship rejected: VERIFY.md chưa tồn tại trong task folder".to_string());
    }
    let output = format!("{}/SHIP.md", task_path);
    let data = read_json_input(args)?;
    validate_ship(&data)?;
    write_text(&output, &render_ship(&data))?;
    println!("created {}", output);

    if has_flag(args, "--dry-run") {
        if is_git_repo() {
            git(["status", "--short"])?;
        } else {
            println!("not a git repository; skipped git status");
        }
    }

    if has_flag(args, "--commit") {
        if !is_git_repo() {
            return Err("Ship rejected: not a git repository".to_string());
        }

        let scope = if is_truthy(get_path(&data, &["commit", "scope"])) {
            format!(
                "({})",
                option_to_js_string(get_path(&data, &["commit", "scope"]))
            )
        } else {
            String::new()
        };

        let subject = format!(
            "{}{}: {}",
            option_to_js_string(get_path(&data, &["commit", "type"])),
            scope,
            option_to_js_string(get_path(&data, &["commit", "message"]))
        );

        let mut commit_args = vec!["commit".to_string(), "-m".to_string(), subject];
        if let Some(Value::Array(body)) = get_path(&data, &["commit", "body"]) {
            for line in body {
                commit_args.push("-m".to_string());
                commit_args.push(value_to_js_string(line));
            }
        }

        git(["add", "."])?;
        git(commit_args)?;
    }

    if !has_flag(args, "--dry-run") && !has_flag(args, "--commit") {
        println!("ship artifact created. use --dry-run or --commit for git actions.");
    }

    Ok(())
}

fn command_status() {
    if !Path::new(".coding/current").exists() {
        println!("No current task. Run `cflow new \"<task-name>\"` first.");
        return;
    }
    
    let current_task = fs::read_to_string(".coding/current").unwrap_or_default();
    let current_task = current_task.trim();
    let task_id = current_task.strip_prefix("tasks/").unwrap_or(current_task);
    
    println!("Current task: {}", task_id);
    println!("Path: .coding/{}", current_task);
    println!("Artifacts:");
    
    let files = ["REQUEST.md", "PLAN.md", "VERIFY.md", "SHIP.md"];
    for file in files {
        let file_path = format!(".coding/{}/{}", current_task, file);
        let status = if Path::new(&file_path).exists() { "exists" } else { "missing" };
        println!("- {}: {}", file, status);
    }
}


fn print_usage() {
    println!(
        "Usage:
  cflow new \"<task-name>\"
  cflow request [--task current] [--input file]
  cflow plan    [--task current] [--input file]
  cflow verify  [--task current] [--input file]
  cflow ship    [--task current] [--input file] [--dry-run|--commit]
  cflow status

Examples:
  cflow new \"focus garden\"
  cat request.json | cflow request
  cflow plan --input examples/plan.json
"
    );
}

fn run() -> CflowResult<()> {
    let mut raw_args = env::args().skip(1).collect::<Vec<_>>();
    let cmd = if raw_args.is_empty() {
        None
    } else {
        Some(raw_args.remove(0))
    };

    match cmd.as_deref() {
        Some("new") => command_new(&raw_args),
        Some("request") => command_request(&raw_args),
        Some("plan") => command_plan(&raw_args),
        Some("verify") => command_verify(&raw_args),
        Some("ship") => command_ship(&raw_args),
        Some("status") => {
            command_status();
            Ok(())
        }
        _ => {
            print_usage();
            Ok(())
        }
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("cflow error: {error}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_json(input: &str) -> Value {
        serde_json::from_str(input).expect("fixture should be valid JSON")
    }

    #[test]
    fn validates_and_renders_request_example() {
        let data = parse_json(include_str!("../examples/request.json"));

        validate_request(&data).expect("request should validate");
        let rendered = render_request(&data);

        assert!(rendered.contains("# Request Intake"));
        assert!(rendered.contains("## Next Action\n\nplan\n"));
    }

    #[test]
    fn validates_and_renders_plan_example() {
        let data = parse_json(include_str!("../examples/plan.json"));

        validate_plan(&data).expect("plan should validate");
        let rendered = render_plan(&data);

        assert!(rendered.contains("# Code Plan"));
        assert!(rendered.contains("## 9. Done Criteria"));
    }

    #[test]
    fn ship_rejects_unpassed_verification() {
        let mut data = parse_json(include_str!("../examples/ship.json"));
        data["verification"]["status"] = Value::String("failed".to_string());

        assert_eq!(
            validate_ship(&data).expect_err("ship should reject failed verification"),
            "Ship rejected: verification.status must be passed"
        );
    }
}
