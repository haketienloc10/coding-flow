use serde_json::Value;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::IsTerminal;
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

const CODING_MODES: &[&str] = &["initial", "fix"];

const CODING_STATUSES: &[&str] = &["ready_for_verify", "blocked", "partial", "failed"];

const CODING_NEXT_ACTIONS: &[&str] = &["verify", "plan", "clarify", "none"];

const VERIFY_STATUSES: &[&str] = &["passed", "failed", "partial", "skipped"];

const FINDING_SEVERITIES: &[&str] = &["low", "medium", "high", "blocking"];

const FINDING_TYPES: &[&str] = &[
    "acceptance_mismatch",
    "test_failure",
    "runtime_error",
    "copy_mismatch",
    "ui_mismatch",
    "regression",
    "missing_behavior",
    "other",
];

const COMMIT_TYPES: &[&str] = &[
    "feat", "fix", "refactor", "docs", "test", "chore", "ci", "build", "perf",
];

const PROBLEMS_PATH: &str = ".coding/knowledge/PROBLEMS.md";

const PROBLEM_STATUSES: &[&str] = &["open", "resolved", "cancelled"];

const PROBLEM_SEVERITIES: &[&str] = &["low", "medium", "high", "blocking"];

const PROBLEM_PHASES: &[&str] = &[
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
    let input_opt = args
        .iter()
        .position(|arg| arg == "--input")
        .and_then(|idx| args.get(idx + 1));
    let content = if let Some(input_file) = input_opt {
        if !Path::new(input_file).exists() {
            return Err(format!("Input file not found: {}", input_file));
        }
        fs::read_to_string(input_file).map_err(|error| error.to_string())?
    } else {
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|error| error.to_string())?;
        if buffer.trim().is_empty() {
            return Err("No input provided via stdin. Use --input or pipe JSON.".to_string());
        }
        buffer
    };
    serde_json::from_str(&content).map_err(|error| format!("Invalid JSON: {}", error))
}

fn read_optional_json_input(args: &[String]) -> CflowResult<Option<Value>> {
    let input_opt = args
        .iter()
        .position(|arg| arg == "--input")
        .and_then(|idx| args.get(idx + 1));
    let content = if let Some(input_file) = input_opt {
        if !Path::new(input_file).exists() {
            return Err(format!("Input file not found: {}", input_file));
        }
        fs::read_to_string(input_file).map_err(|error| error.to_string())?
    } else {
        let mut stdin = std::io::stdin();
        if stdin.is_terminal() {
            return Ok(None);
        }

        use std::io::Read;
        let mut buffer = String::new();
        stdin
            .read_to_string(&mut buffer)
            .map_err(|error| error.to_string())?;
        if buffer.trim().is_empty() {
            return Ok(None);
        }
        buffer
    };

    serde_json::from_str(&content)
        .map(Some)
        .map_err(|error| format!("Invalid JSON: {}", error))
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

fn load_state() -> Value {
    if let Ok(content) = fs::read_to_string(".coding/state.json") {
        if let Ok(mut val) = serde_json::from_str::<Value>(&content) {
            let ver = val.get("version").and_then(|v| {
                v.as_i64().or_else(|| v.as_str().and_then(|s| s.parse::<i64>().ok()))
            }).unwrap_or(1);
            if ver < 2 {
                val["version"] = Value::Number(2.into());
                if val.get("packets").is_none() {
                    val["packets"] = Value::Object(serde_json::Map::new());
                }
                if val.get("current_packet_id").is_none() {
                    val["current_packet_id"] = Value::Null;
                }
                if val.get("current_story_id").is_none() {
                    val["current_story_id"] = Value::Null;
                }
            }
            return val;
        }
    }
    let mut map = serde_json::Map::new();
    map.insert("version".to_string(), Value::Number(2.into()));
    map.insert("current_task_id".to_string(), Value::Null);
    map.insert("current_packet_id".to_string(), Value::Null);
    map.insert("current_story_id".to_string(), Value::Null);
    map.insert("tasks".to_string(), Value::Object(serde_json::Map::new()));
    map.insert("packets".to_string(), Value::Object(serde_json::Map::new()));
    Value::Object(map)
}

fn save_state(state: &Value) -> CflowResult<()> {
    ensure_dir(".coding/state.json")?;
    let content = serde_json::to_string_pretty(state).map_err(|e| e.to_string())?;
    write_text(".coding/state.json", &content)?;
    Ok(())
}

fn extract_task_id(task_path: &str) -> String {
    let path = Path::new(task_path);
    let filename = path.file_name().and_then(|f| f.to_str()).unwrap_or("");
    filename.to_string()
}

fn update_task_state(
    task_id: &str,
    phase_override: Option<&str>,
    title_override: Option<&str>,
) -> CflowResult<()> {
    let mut state = load_state();

    let is_story = if let Some(packet_id) = state["current_packet_id"].as_str() {
        state["packets"].get(packet_id)
            .and_then(|p| p.get("stories"))
            .and_then(|s| s.get(task_id))
            .is_some()
    } else {
        false
    };

    if is_story {
        let packet_id = state["current_packet_id"].as_str().unwrap().to_string();
        let mut story_meta = state["packets"][&packet_id]["stories"][task_id].clone();
        let story_path = format!(".coding/packets/{}/stories/{}", packet_id, task_id);
        
        let has_plan = Path::new(&format!("{}/PLAN.md", story_path)).exists();
        let has_coding = Path::new(&format!("{}/CODING.md", story_path)).exists();
        let has_verify = Path::new(&format!("{}/VERIFY.md", story_path)).exists();
        let has_ship = Path::new(&format!("{}/SHIP.md", story_path)).exists();
        
        let mut status = story_meta["status"].as_str().unwrap_or("todo").to_string();
        if has_ship {
            status = "done".to_string();
        } else if has_coding {
            status = "in_progress".to_string();
        }
        
        let phase = if let Some(p) = phase_override {
            p.to_string()
        } else {
            if has_ship {
                "shipped".to_string()
            } else if has_verify {
                let verify_content = fs::read_to_string(&format!("{}/VERIFY.md", story_path)).unwrap_or_default();
                let verify_status = first_non_empty_section_line(&verify_content, "Status").unwrap_or_default();
                if verify_status == "passed" {
                    "verify_passed".to_string()
                } else {
                    "verify_failed".to_string()
                }
            } else if has_coding {
                "coding_done".to_string()
            } else if has_plan {
                "planned".to_string()
            } else {
                "new".to_string()
            }
        };
        
        let mut findings_count = 0;
        if has_verify {
            let verify_content = fs::read_to_string(&format!("{}/VERIFY.md", story_path)).unwrap_or_default();
            findings_count = verify_findings_count(&verify_content);
        }
        
        story_meta["status"] = Value::String(status);
        story_meta["phase"] = Value::String(phase);
        story_meta["findings_count"] = Value::Number(findings_count.into());
        
        state["packets"][&packet_id]["stories"][task_id] = story_meta;
        state["packets"][&packet_id]["updated_at"] = Value::String(chrono::Local::now().to_rfc3339());
        
        save_state(&state)?;
        return Ok(());
    }

    if !state["tasks"].is_object() {
        state["tasks"] = Value::Object(serde_json::Map::new());
    }

    let task_path = format!(".coding/tasks/{}", task_id);
    let is_new = state["tasks"].get(task_id).is_none();

    let mut task_meta = if is_new {
        let mut map = serde_json::Map::new();
        map.insert(
            "title".to_string(),
            Value::String(title_override.unwrap_or(task_id).to_string()),
        );
        map.insert("status".to_string(), Value::String("todo".to_string()));
        map.insert("phase".to_string(), Value::String("new".to_string()));
        map.insert("next_action".to_string(), Value::String("plan".to_string()));
        let now = chrono::Local::now().to_rfc3339();
        map.insert("created_at".to_string(), Value::String(now.clone()));
        map.insert("updated_at".to_string(), Value::String(now));
        map.insert("type".to_string(), Value::String(String::new()));
        map.insert("lane".to_string(), Value::String(String::new()));
        map.insert("summary".to_string(), Value::String(String::new()));

        let mut art_pres = serde_json::Map::new();
        art_pres.insert("REQUEST.md".to_string(), Value::Bool(false));
        art_pres.insert("PLAN.md".to_string(), Value::Bool(false));
        art_pres.insert("CODING.md".to_string(), Value::Bool(false));
        art_pres.insert("VERIFY.md".to_string(), Value::Bool(false));
        art_pres.insert("SHIP.md".to_string(), Value::Bool(false));
        map.insert("artifact_presence".to_string(), Value::Object(art_pres));

        map.insert("verify_status".to_string(), Value::Null);
        map.insert("findings_count".to_string(), Value::Number(0.into()));
        map.insert("ship_ready".to_string(), Value::Null);
        map.insert("committed".to_string(), Value::Bool(false));
        map.insert("commit_sha".to_string(), Value::Null);
        Value::Object(map)
    } else {
        state["tasks"][task_id].clone()
    };

    if let Some(title) = title_override {
        task_meta["title"] = Value::String(title.to_string());
    }

    let req_path = format!("{}/REQUEST.md", task_path);
    let plan_path = format!("{}/PLAN.md", task_path);
    let coding_path = format!("{}/CODING.md", task_path);
    let verify_path = format!("{}/VERIFY.md", task_path);
    let ship_path = format!("{}/SHIP.md", task_path);

    let has_req = Path::new(&req_path).exists();
    let has_plan = Path::new(&plan_path).exists();
    let has_coding = Path::new(&coding_path).exists();
    let has_verify = Path::new(&verify_path).exists();
    let has_ship = Path::new(&ship_path).exists();

    task_meta["artifact_presence"]["REQUEST.md"] = Value::Bool(has_req);
    task_meta["artifact_presence"]["PLAN.md"] = Value::Bool(has_plan);
    task_meta["artifact_presence"]["CODING.md"] = Value::Bool(has_coding);
    task_meta["artifact_presence"]["VERIFY.md"] = Value::Bool(has_verify);
    task_meta["artifact_presence"]["SHIP.md"] = Value::Bool(has_ship);

    if has_req {
        if let Ok(content) = fs::read_to_string(&req_path) {
            if let Some(summary) = first_non_empty_section_line(&content, "Summary") {
                task_meta["summary"] = Value::String(summary);
            }
            if let Some(t) = first_non_empty_section_line(&content, "Type") {
                task_meta["type"] = Value::String(t);
            }
            if let Some(lane) = first_non_empty_section_line(&content, "Lane") {
                task_meta["lane"] = Value::String(lane);
            }
            if let Some(next_action) = first_non_empty_section_line(&content, "Next Action") {
                task_meta["next_action"] = Value::String(next_action);
            }
        }
    }

    if has_coding {
        if let Ok(content) = fs::read_to_string(&coding_path) {
            if let Some(status) = first_non_empty_section_line(&content, "Status") {
                task_meta["status"] = Value::String(status);
            }
            if let Some(next) = first_non_empty_section_line(&content, "Next") {
                task_meta["next_action"] = Value::String(next);
            }
        }
    }

    if has_verify {
        if let Ok(content) = fs::read_to_string(&verify_path) {
            if let Some(status) = first_non_empty_section_line(&content, "Status") {
                task_meta["verify_status"] = Value::String(status);
            }
            let findings = verify_findings_count(&content);
            task_meta["findings_count"] = Value::Number(findings.into());
        }
    }

    if has_ship {
        if let Ok(content) = fs::read_to_string(&ship_path) {
            if let Some(ready) = first_non_empty_section_line(&content, "Ready") {
                task_meta["ship_ready"] = Value::String(ready);
            }
        }
    }

    let phase = if let Some(p) = phase_override {
        p.to_string()
    } else {
        if has_ship {
            if task_meta["committed"].as_bool().unwrap_or(false) {
                "committed".to_string()
            } else {
                "commit_pending".to_string()
            }
        } else if has_verify {
            if task_meta["verify_status"].as_str() == Some("passed") {
                "verify_passed".to_string()
            } else {
                "verify_failed".to_string()
            }
        } else if has_coding {
            "coding_done".to_string()
        } else if has_plan {
            "planned".to_string()
        } else if has_req {
            "requested".to_string()
        } else {
            "new".to_string()
        }
    };

    task_meta["phase"] = Value::String(phase);
    task_meta["updated_at"] = Value::String(chrono::Local::now().to_rfc3339());

    state["tasks"][task_id] = task_meta;
    save_state(&state)?;
    Ok(())
}

fn get_git_commit_sha() -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()?;
    if output.status.success() {
        let sha = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !sha.is_empty() {
            return Some(sha);
        }
    }
    None
}

fn resolve_task(args: &[String]) -> CflowResult<String> {
    let is_story = args.iter().any(|arg| arg == "--story");
    if is_story {
        let (packet_id, story_dir_name) = resolve_story_path(args)?;
        return Ok(format!(".coding/packets/{}/stories/{}", packet_id, story_dir_name));
    }

    let task = get_arg(args, "--task", "current");
    if task == "current" {
        let state = load_state();
        if let Some(id) = state["current_task_id"].as_str() {
            return Ok(format!(".coding/tasks/{}", id));
        }
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

fn markdown_section<'a>(content: &'a str, heading: &str) -> Option<Vec<&'a str>> {
    let heading = format!("## {heading}");
    let mut lines = content.lines();

    while let Some(line) = lines.next() {
        if line.trim() == heading {
            let mut section = Vec::new();
            for section_line in lines.by_ref() {
                if section_line.starts_with("## ") {
                    break;
                }
                section.push(section_line);
            }
            return Some(section);
        }
    }

    None
}

fn first_non_empty_section_line(content: &str, heading: &str) -> Option<String> {
    markdown_section(content, heading)?
        .into_iter()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(str::to_string)
}

fn parse_ship_commit_message(content: &str) -> Option<String> {
    let section = markdown_section(content, "Commit Message")?;
    let mut in_fence = false;
    let mut saw_fence = false;

    for line in &section {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            saw_fence = true;
            if in_fence {
                break;
            }
            in_fence = true;
            continue;
        }

        if in_fence && !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }

    if saw_fence {
        return None;
    }

    section
        .into_iter()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(str::to_string)
}

fn require_field(value: Option<&Value>, name: &str) -> CflowResult<()> {
    if is_empty(value) {
        Err(format!("Missing or empty required field: {name}"))
    } else {
        Ok(())
    }
}

fn require_present(value: Option<&Value>, name: &str) -> CflowResult<()> {
    match value {
        None | Some(Value::Null) => Err(format!("Missing required field: {name}")),
        Some(_) => Ok(()),
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

fn finding_ids_from_markdown(content: &str) -> Vec<String> {
    let Some(section) = markdown_section(content, "Findings") else {
        return Vec::new();
    };

    section
        .into_iter()
        .filter_map(|line| {
            let trimmed = line.trim();
            let rest = trimmed.strip_prefix("- [")?;
            let (id, _) = rest.split_once(']')?;
            if id.trim().is_empty() {
                None
            } else {
                Some(id.trim().to_string())
            }
        })
        .collect()
}

fn verify_findings_count(content: &str) -> usize {
    finding_ids_from_markdown(content).len()
}

fn render_findings(items: Option<&Value>) -> String {
    let Some(Value::Array(items)) = items else {
        return "- _None_".to_string();
    };

    if items.is_empty() {
        return "- _None_".to_string();
    }

    items
        .iter()
        .map(|item| {
            let Value::Object(map) = item else {
                return format!("- {}", value_to_js_string(item));
            };

            let suggested_fix = if is_truthy(map.get("suggested_fix")) {
                option_to_js_string(map.get("suggested_fix"))
            } else {
                "_None_".to_string()
            };

            format!(
                "- [{}] {}\n  - Severity: {}\n  - Expected: {}\n  - Actual: {}\n  - Evidence: {}\n  - Suggested fix: {}",
                option_to_js_string(map.get("id")),
                option_to_js_string(map.get("type")),
                option_to_js_string(map.get("severity")),
                option_to_js_string(map.get("expected")),
                option_to_js_string(map.get("actual")),
                option_to_js_string(map.get("evidence")),
                suggested_fix
            )
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

fn validate_coding(data: &Value) -> CflowResult<()> {
    require_field(get_path(data, &["mode"]), "mode")?;
    require_field(get_path(data, &["status"]), "status")?;
    require_present(get_path(data, &["summary"]), "summary")?;
    require_present(get_path(data, &["fixed_findings"]), "fixed_findings")?;
    require_present(get_path(data, &["changed_files"]), "changed_files")?;
    require_present(get_path(data, &["notes"]), "notes")?;
    require_field(get_path(data, &["next"]), "next")?;

    assert_allowed(get_path(data, &["mode"]), CODING_MODES, "mode")?;
    assert_allowed(get_path(data, &["status"]), CODING_STATUSES, "status")?;
    assert_allowed(get_path(data, &["next"]), CODING_NEXT_ACTIONS, "next")?;

    if option_to_js_string(get_path(data, &["status"])) == "ready_for_verify"
        && option_to_js_string(get_path(data, &["next"])) != "verify"
    {
        return Err("Invalid coding: status=ready_for_verify expects next=verify".to_string());
    }

    Ok(())
}

fn validate_coding_for_task(data: &Value, task_path: &str) -> CflowResult<()> {
    validate_coding(data)?;

    if option_to_js_string(get_path(data, &["mode"])) != "fix" {
        return Ok(());
    }

    let verify_path = format!("{}/VERIFY.md", task_path);
    if !Path::new(&verify_path).exists() {
        return Err("Invalid coding: mode=fix requires latest VERIFY.md".to_string());
    }

    let verify_md = fs::read_to_string(&verify_path).map_err(|error| error.to_string())?;
    let finding_ids = finding_ids_from_markdown(&verify_md);
    let Some(Value::Array(fixed_findings)) = get_path(data, &["fixed_findings"]) else {
        return Err("Invalid coding: fixed_findings must be an array".to_string());
    };

    for fixed in fixed_findings {
        let fixed = value_to_js_string(fixed);
        if !finding_ids.iter().any(|id| id == &fixed) {
            return Err(format!(
                "Invalid coding: fixed_findings references unknown VERIFY.md finding id {fixed}"
            ));
        }
    }

    Ok(())
}

fn validate_verify(data: &Value) -> CflowResult<()> {
    require_field(get_path(data, &["status"]), "status")?;
    require_present(get_path(data, &["checks"]), "checks")?;
    require_present(get_path(data, &["manual_checks"]), "manual_checks")?;
    require_present(
        get_path(data, &["acceptance_criteria_checked"]),
        "acceptance_criteria_checked",
    )?;
    require_present(get_path(data, &["findings"]), "findings")?;
    require_present(get_path(data, &["known_issues"]), "known_issues")?;
    require_present(
        get_path(data, &["done_criteria_verified"]),
        "done_criteria_verified",
    )?;

    assert_allowed(get_path(data, &["status"]), VERIFY_STATUSES, "status")?;

    let status = option_to_js_string(get_path(data, &["status"]));
    let findings = get_path(data, &["findings"])
        .and_then(Value::as_array)
        .ok_or_else(|| "Invalid verify: findings must be an array".to_string())?;

    if status == "passed" && !findings.is_empty() {
        return Err("Invalid verify: status=passed requires findings to be empty".to_string());
    }

    if (status == "failed" || status == "partial") && findings.is_empty() {
        return Err("Invalid verify: status=failed or partial requires findings".to_string());
    }

    for (index, finding) in findings.iter().enumerate() {
        require_field(finding.get("id"), &format!("findings[{index}].id"))?;
        require_field(
            finding.get("severity"),
            &format!("findings[{index}].severity"),
        )?;
        require_field(finding.get("type"), &format!("findings[{index}].type"))?;
        require_field(
            finding.get("expected"),
            &format!("findings[{index}].expected"),
        )?;
        require_field(finding.get("actual"), &format!("findings[{index}].actual"))?;
        require_field(
            finding.get("evidence"),
            &format!("findings[{index}].evidence"),
        )?;
        assert_allowed(
            finding.get("severity"),
            FINDING_SEVERITIES,
            &format!("findings[{index}].severity"),
        )?;
        assert_allowed(
            finding.get("type"),
            FINDING_TYPES,
            &format!("findings[{index}].type"),
        )?;
    }

    Ok(())
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

fn render_coding(data: &Value) -> String {
    format!(
        "# Coding\n\n## Mode\n\n{}\n\n## Status\n\n{}\n\n## Summary\n\n{}\n\n## Fixed Findings\n\n{}\n\n## Changed Files\n\n{}\n\n## Notes\n\n{}\n\n## Next\n\n{}\n",
        option_to_js_string(get_path(data, &["mode"])),
        option_to_js_string(get_path(data, &["status"])),
        list(get_path(data, &["summary"])),
        list(get_path(data, &["fixed_findings"])),
        list(get_path(data, &["changed_files"])),
        list(get_path(data, &["notes"])),
        option_to_js_string(get_path(data, &["next"]))
    )
}

fn render_verify(data: &Value) -> String {
    format!(
        "# Verify\n\n## Status\n\n{}\n\n## Automated Checks\n\n{}\n\n## Manual Checks\n\n{}\n\n## Acceptance Criteria Checked\n\n{}\n\n## Findings\n\n{}\n\n## Known Issues\n\n{}\n\n## Done Criteria Verified\n\n{}\n",
        option_to_js_string(get_path(data, &["status"])),
        list(get_path(data, &["checks"])),
        list(get_path(data, &["manual_checks"])),
        list(get_path(data, &["acceptance_criteria_checked"])),
        render_findings(get_path(data, &["findings"])),
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

#[derive(Clone)]
struct ProblemEntry {
    id: String,
    title: String,
    status: String,
    severity: String,
    area: String,
    content: String,
}

fn normalize_inline(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn validate_problem_status(status: &str) -> CflowResult<()> {
    if PROBLEM_STATUSES.contains(&status) {
        Ok(())
    } else {
        Err(format!(
            "Invalid status: {}. Allowed: {}",
            status,
            PROBLEM_STATUSES.join(", ")
        ))
    }
}

fn validate_problem_input(data: &Value) -> CflowResult<()> {
    if data.get("id").is_some() {
        return Err("Problem id is generated by CLI; do not provide id.".to_string());
    }
    if data.get("detected_at").is_some() {
        return Err("Problem detected_at is generated by CLI; do not provide detected_at.".to_string());
    }

    require_field(get_path(data, &["title"]), "title")?;
    require_field(get_path(data, &["severity"]), "severity")?;
    require_field(get_path(data, &["area"]), "area")?;
    require_field(get_path(data, &["detected_by", "agent"]), "detected_by.agent")?;
    require_field(get_path(data, &["detected_by", "provider"]), "detected_by.provider")?;
    require_field(get_path(data, &["detected_by", "command"]), "detected_by.command")?;
    require_field(get_path(data, &["phase"]), "phase")?;
    require_field(get_path(data, &["problem"]), "problem")?;
    require_field(get_path(data, &["impact"]), "impact")?;
    require_field(get_path(data, &["fallback"]), "fallback")?;
    require_field(get_path(data, &["follow_up"]), "follow_up")?;

    let status = option_to_js_string(get_path(data, &["status"]));
    if !status.is_empty() && status != "null" {
        validate_problem_status(&status)?;
    }
    assert_allowed(get_path(data, &["severity"]), PROBLEM_SEVERITIES, "severity")?;
    assert_allowed(get_path(data, &["phase"]), PROBLEM_PHASES, "phase")?;

    if let Some(links) = data.get("links") {
        let Some(items) = links.as_array() else {
            return Err("Invalid links: expected array".to_string());
        };
        for (index, item) in items.iter().enumerate() {
            if !item.is_string() {
                return Err(format!("Invalid links[{index}]: expected string"));
            }
        }
    }

    Ok(())
}

fn parse_problem_heading(line: &str) -> Option<(String, String)> {
    let heading = line.trim().strip_prefix("## ")?;
    let (id, title) = heading.split_once(" - ")?;
    let digits = id.strip_prefix('P')?;
    if digits.len() != 3 || !digits.chars().all(|ch| ch.is_ascii_digit()) {
        return None;
    }
    Some((id.to_string(), title.trim().to_string()))
}

fn problem_field(block: &str, field: &str) -> String {
    let prefix = format!("{field}:");
    block
        .lines()
        .find_map(|line| {
            let rest = line.trim().strip_prefix(&prefix)?;
            Some(rest.trim().trim_end_matches("  ").trim().to_string())
        })
        .unwrap_or_default()
}

fn parse_problem_block(block: &str) -> Option<ProblemEntry> {
    let first = block.lines().next()?;
    let (id, title) = parse_problem_heading(first)?;
    Some(ProblemEntry {
        id,
        title,
        status: problem_field(block, "Status"),
        severity: problem_field(block, "Severity"),
        area: problem_field(block, "Area"),
        content: block.trim_end().to_string(),
    })
}

fn parse_problem_entries(content: &str) -> Vec<ProblemEntry> {
    let mut starts = Vec::new();
    let mut offset = 0;
    for line in content.split_inclusive('\n') {
        let clean_line = line.trim_end_matches(['\r', '\n']);
        if parse_problem_heading(clean_line).is_some() {
            starts.push(offset);
        }
        offset += line.len();
    }

    starts
        .iter()
        .enumerate()
        .filter_map(|(index, start)| {
            let end = starts.get(index + 1).copied().unwrap_or(content.len());
            parse_problem_block(&content[*start..end])
        })
        .collect()
}

fn next_problem_id(content: &str) -> String {
    let max_id = parse_problem_entries(content)
        .iter()
        .filter_map(|entry| entry.id.strip_prefix('P')?.parse::<usize>().ok())
        .max()
        .unwrap_or(0);
    format!("P{:03}", max_id + 1)
}

fn markdown_list_from_links(links: Option<&Value>) -> String {
    let Some(Value::Array(items)) = links else {
        return "- _None_".to_string();
    };
    if items.is_empty() {
        return "- _None_".to_string();
    }
    items
        .iter()
        .filter_map(|item| item.as_str())
        .map(|text| format!("- `{}`", text))
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_detected_by(data: &Value) -> String {
    let agent = normalize_inline(&option_to_js_string(get_path(data, &["detected_by", "agent"])));
    let provider = normalize_inline(&option_to_js_string(get_path(data, &["detected_by", "provider"])));
    let command = option_to_js_string(get_path(data, &["detected_by", "command"]));

    let actor = if agent == provider || provider.is_empty() {
        agent
    } else {
        format!("{agent} / {provider}")
    };

    format!("{actor} / `{command}`")
}

fn render_problem_entry(id: &str, detected_at: &str, data: &Value) -> String {
    let status = {
        let value = option_to_js_string(get_path(data, &["status"]));
        if value.is_empty() || value == "null" {
            "open".to_string()
        } else {
            value
        }
    };

    format!(
        "## {} - {}\n\nStatus: {}  \nSeverity: {}  \nArea: {}  \nDetected by: {}  \nPhase: {}  \nDetected at: {}  \n\n### Problem\n\n{}\n\n### Impact\n\n{}\n\n### Fallback\n\n{}\n\n### Follow-up\n\n{}\n\n### Links\n\n{}\n",
        id,
        normalize_inline(&option_to_js_string(get_path(data, &["title"]))),
        status,
        option_to_js_string(get_path(data, &["severity"])),
        option_to_js_string(get_path(data, &["area"])),
        render_detected_by(data),
        option_to_js_string(get_path(data, &["phase"])),
        detected_at,
        option_to_js_string(get_path(data, &["problem"])),
        option_to_js_string(get_path(data, &["impact"])),
        option_to_js_string(get_path(data, &["fallback"])),
        option_to_js_string(get_path(data, &["follow_up"])),
        markdown_list_from_links(get_path(data, &["links"]))
    )
}

fn update_problem_status_line(block: &str, status: &str) -> String {
    let mut replaced = false;
    let mut output = Vec::new();

    for line in block.lines() {
        if line.trim_start().starts_with("Status:") && !replaced {
            output.push(format!("Status: {}  ", status));
            replaced = true;
        } else {
            output.push(line.to_string());
        }
    }

    if replaced {
        output.join("\n")
    } else {
        let mut lines = block.lines();
        let mut result = String::new();
        if let Some(first) = lines.next() {
            result.push_str(first);
            result.push_str("\n\n");
            result.push_str(&format!("Status: {}  \n", status));
            for line in lines {
                result.push_str(line);
                result.push('\n');
            }
            result.trim_end().to_string()
        } else {
            block.to_string()
        }
    }
}

fn lifecycle_section(status: &str) -> Option<(&'static str, &'static str)> {
    match status {
        "resolved" => Some(("Resolution", "Resolved")),
        "cancelled" => Some(("Cancellation", "Cancelled")),
        "open" => Some(("Update", "Updated")),
        _ => None,
    }
}

fn append_problem_lifecycle_note(block: &str, status: &str, note: &str, timestamp: &str) -> String {
    let Some((section, label)) = lifecycle_section(status) else {
        return block.to_string();
    };

    if (status == "resolved" || status == "cancelled")
        && block.lines().any(|line| line.trim() == format!("### {section}"))
    {
        return block.to_string();
    }

    format!(
        "{}\n\n### {}\n\n{}\n\n{} at: {}",
        block.trim_end(),
        section,
        note,
        label,
        timestamp
    )
}

fn update_problem_entry_content(block: &str, status: &str, note: &str, timestamp: &str) -> String {
    let updated = update_problem_status_line(block, status);
    append_problem_lifecycle_note(&updated, status, note, timestamp)
}

fn rewrite_problem_entry(content: &str, id: &str, status: &str, note: &str) -> CflowResult<String> {
    let timestamp = chrono::Local::now().to_rfc3339();
    let mut starts = Vec::new();
    let mut offset = 0;
    for line in content.split_inclusive('\n') {
        let clean_line = line.trim_end_matches(['\r', '\n']);
        if let Some((entry_id, _)) = parse_problem_heading(clean_line) {
            starts.push((offset, entry_id));
        }
        offset += line.len();
    }

    for (index, (start, entry_id)) in starts.iter().enumerate() {
        if entry_id != id {
            continue;
        }
        let end = starts
            .get(index + 1)
            .map(|(next_start, _)| *next_start)
            .unwrap_or(content.len());
        let mut updated = String::new();
        updated.push_str(&content[..*start]);
        updated.push_str(&update_problem_entry_content(&content[*start..end], status, note, &timestamp));
        updated.push_str("\n");
        updated.push_str(&content[end..].trim_start_matches('\n'));
        return Ok(updated);
    }

    Err(format!("Problem not found: {}", id))
}

fn read_problems_file() -> CflowResult<String> {
    if !Path::new(PROBLEMS_PATH).exists() {
        return Ok(String::new());
    }
    fs::read_to_string(PROBLEMS_PATH).map_err(|error| error.to_string())
}

fn command_problem_add(args: &[String]) -> CflowResult<()> {
    let mut data = read_json_input(args)?;
    if data.get("status").is_none() {
        data["status"] = Value::String("open".to_string());
    }
    if data.get("links").is_none() {
        data["links"] = Value::Array(Vec::new());
    }

    validate_problem_input(&data)?;

    let existing = read_problems_file()?;
    let id = next_problem_id(&existing);
    let detected_at = chrono::Local::now().to_rfc3339();
    let entry = render_problem_entry(&id, &detected_at, &data);

    let mut content = existing.trim_end().to_string();
    if !content.is_empty() {
        content.push_str("\n\n");
    }
    content.push_str(&entry);
    if !content.ends_with('\n') {
        content.push('\n');
    }

    write_text(PROBLEMS_PATH, &content)?;
    println!("Problem added: {}", id);
    println!("Path: {}", PROBLEMS_PATH);
    Ok(())
}

fn command_problem_list(status_filter: Option<&str>) -> CflowResult<()> {
    if let Some(status) = status_filter {
        validate_problem_status(status)?;
    }

    let content = read_problems_file()?;
    let entries = parse_problem_entries(&content);
    for entry in entries {
        if status_filter.is_some_and(|status| entry.status != status) {
            continue;
        }
        println!(
            "{:<5}  {:<9}  {:<8}  {:<13}  {}",
            entry.id, entry.status, entry.severity, entry.area, entry.title
        );
    }
    Ok(())
}

fn command_problem_show(args: &[String]) -> CflowResult<()> {
    let id = args.first().cloned().unwrap_or_default();
    if id.is_empty() {
        return Err("Usage: cflow problem show <id>".to_string());
    }

    let content = read_problems_file()?;
    let Some(entry) = parse_problem_entries(&content)
        .into_iter()
        .find(|entry| entry.id == id)
    else {
        return Err(format!("Problem not found: {}", id));
    };

    println!("{}", entry.content);
    Ok(())
}

fn command_problem_update_status(args: &[String], forced_status: Option<&str>) -> CflowResult<()> {
    let id = args.first().cloned().unwrap_or_default();
    if id.is_empty() {
        let usage = match forced_status {
            Some("resolved") => "Usage: cflow problem resolve <id> --note \"<text>\"",
            Some("cancelled") => "Usage: cflow problem cancel <id> --note \"<text>\"",
            _ => "Usage: cflow problem update <id> --status <open|resolved|cancelled> --note \"<text>\"",
        };
        return Err(usage.to_string());
    }

    let status = forced_status
        .map(str::to_string)
        .unwrap_or_else(|| get_arg(args, "--status", ""));
    if status.is_empty() {
        return Err("Missing --status. Allowed: open, resolved, cancelled".to_string());
    }
    validate_problem_status(&status)?;

    let note = get_arg(args, "--note", "");
    if note.trim().is_empty() {
        return Err("Missing --note \"<text>\"".to_string());
    }

    let content = read_problems_file()?;
    let updated = rewrite_problem_entry(&content, &id, &status, &note)?;
    write_text(PROBLEMS_PATH, &updated)?;
    println!("Problem updated: {}", id);
    println!("Status: {}", status);
    Ok(())
}

fn command_problem(args: &[String]) -> CflowResult<()> {
    if args.is_empty() {
        return Err("Usage: cflow problem add|list|open|resolved|cancelled|show|resolve|cancel|update".to_string());
    }

    match args[0].as_str() {
        "add" => command_problem_add(&args[1..]),
        "list" => {
            let status = get_arg(&args[1..], "--status", "");
            if status.is_empty() {
                command_problem_list(None)
            } else {
                command_problem_list(Some(&status))
            }
        }
        "open" => command_problem_list(Some("open")),
        "resolved" => command_problem_list(Some("resolved")),
        "cancelled" => command_problem_list(Some("cancelled")),
        "show" => command_problem_show(&args[1..]),
        "resolve" => command_problem_update_status(&args[1..], Some("resolved")),
        "cancel" => command_problem_update_status(&args[1..], Some("cancelled")),
        "update" => command_problem_update_status(&args[1..], None),
        _ => Err(format!("Unknown problem command: {}", args[0])),
    }
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

#[derive(Clone, Copy)]
enum AgentPhase {
    Plan,
    Coding,
}

impl AgentPhase {
    fn as_str(self) -> &'static str {
        match self {
            AgentPhase::Plan => "plan",
            AgentPhase::Coding => "coding",
        }
    }

    fn schema_path(self) -> &'static str {
        match self {
            AgentPhase::Plan => "schemas/plan.schema.json",
            AgentPhase::Coding => "schemas/coding.schema.json",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PromptMode {
    Arg,
    Stdin,
}

impl PromptMode {
    fn parse(value: Option<&str>) -> CflowResult<Self> {
        match value.unwrap_or("arg") {
            "arg" => Ok(PromptMode::Arg),
            "stdin" => Ok(PromptMode::Stdin),
            other => Err(format!(
                "Invalid prompt_mode '{}'. Expected 'arg' or 'stdin'.",
                other
            )),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            PromptMode::Arg => "arg",
            PromptMode::Stdin => "stdin",
        }
    }
}

#[derive(Clone)]
struct AgentCommand {
    cmd: String,
    args: Vec<String>,
    prompt_mode: PromptMode,
    source: String,
}

struct AgentRunOutput {
    stdout: String,
    stderr: String,
}

const AGENT_PROVIDERS: &[&str] = &["codex", "claude", "gemini", "antigravity", "custom"];

fn load_agent_config() -> CflowResult<Option<toml::Value>> {
    let path = ".coding/agent.toml";
    if !Path::new(path).exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(path).map_err(|error| error.to_string())?;
    content
        .parse::<toml::Value>()
        .map(Some)
        .map_err(|error| format!("Invalid {}: {}", path, error))
}

fn normalize_agent_provider(provider: &str) -> CflowResult<String> {
    let provider = provider.trim().to_lowercase();
    if provider.is_empty() {
        return Err("Agent provider is empty".to_string());
    }
    if AGENT_PROVIDERS.contains(&provider.as_str()) {
        Ok(provider)
    } else {
        Err(format!(
            "Unknown agent provider '{}'. Expected one of: {}",
            provider,
            AGENT_PROVIDERS.join(", ")
        ))
    }
}

fn config_default_provider(config: Option<&toml::Value>) -> Option<String> {
    config?
        .get("default_provider")?
        .as_str()
        .map(|value| value.to_string())
}

fn resolve_agent_provider(args: &[String], config: Option<&toml::Value>) -> CflowResult<String> {
    let cli_provider = get_arg(args, "--provider", "");
    if !cli_provider.is_empty() {
        return normalize_agent_provider(&cli_provider);
    }

    if let Ok(provider) = env::var("CFLOW_AGENT_PROVIDER") {
        if !provider.trim().is_empty() {
            return normalize_agent_provider(&provider);
        }
    }

    if let Some(provider) = config_default_provider(config) {
        if !provider.trim().is_empty() {
            return normalize_agent_provider(&provider);
        }
    }

    Ok("codex".to_string())
}

fn inline_schema_arg(phase: AgentPhase) -> CflowResult<String> {
    fs::read_to_string(phase.schema_path())
        .map_err(|error| format!("Failed to read {}: {}", phase.schema_path(), error))
}

fn builtin_agent_command(provider: &str, phase: AgentPhase) -> CflowResult<Option<AgentCommand>> {
    let command = match (provider, phase) {
        ("codex", AgentPhase::Plan) => AgentCommand {
            cmd: "codex".to_string(),
            args: vec![
                "exec".to_string(),
                "--ephemeral".to_string(),
                "--output-schema".to_string(),
                "schemas/plan.schema.json".to_string(),
                "--".to_string(),
            ],
            prompt_mode: PromptMode::Arg,
            source: "built-in codex default".to_string(),
        },
        ("codex", AgentPhase::Coding) => AgentCommand {
            cmd: "codex".to_string(),
            args: vec![
                "exec".to_string(),
                "--ephemeral".to_string(),
                "--sandbox".to_string(),
                "workspace-write".to_string(),
                "--output-schema".to_string(),
                "schemas/coding.schema.json".to_string(),
                "--".to_string(),
            ],
            prompt_mode: PromptMode::Arg,
            source: "built-in codex default".to_string(),
        },
        ("claude", AgentPhase::Plan) => AgentCommand {
            cmd: "claude".to_string(),
            args: vec![
                "-p".to_string(),
                "--permission-mode".to_string(),
                "plan".to_string(),
                "--output-format".to_string(),
                "json".to_string(),
                "--json-schema".to_string(),
                inline_schema_arg(phase)?,
            ],
            prompt_mode: PromptMode::Arg,
            source: "built-in claude default".to_string(),
        },
        ("claude", AgentPhase::Coding) => AgentCommand {
            cmd: "claude".to_string(),
            args: vec![
                "-p".to_string(),
                "--permission-mode".to_string(),
                "acceptEdits".to_string(),
                "--output-format".to_string(),
                "json".to_string(),
                "--json-schema".to_string(),
                inline_schema_arg(phase)?,
            ],
            prompt_mode: PromptMode::Arg,
            source: "built-in claude default".to_string(),
        },
        ("gemini", AgentPhase::Plan) | ("gemini", AgentPhase::Coding) => AgentCommand {
            cmd: "gemini".to_string(),
            args: vec![
                "-p".to_string(),
                "--output-format".to_string(),
                "json".to_string(),
            ],
            prompt_mode: PromptMode::Arg,
            source: "built-in gemini default".to_string(),
        },
        ("antigravity", AgentPhase::Plan) | ("antigravity", AgentPhase::Coding) => AgentCommand {
            cmd: "agy".to_string(),
            args: vec!["--prompt".to_string()],
            prompt_mode: PromptMode::Arg,
            source: "built-in antigravity default".to_string(),
        },
        ("custom", _) => return Ok(None),
        _ => return Ok(None),
    };

    Ok(Some(command))
}

fn config_agent_command(
    config: Option<&toml::Value>,
    provider: &str,
    phase: AgentPhase,
) -> CflowResult<Option<AgentCommand>> {
    let Some(config) = config else {
        return Ok(None);
    };
    let Some(table) = config
        .get("providers")
        .and_then(|providers| providers.get(provider))
        .and_then(|provider_cfg| provider_cfg.get(phase.as_str()))
    else {
        return Ok(None);
    };

    let cmd = table
        .get("cmd")
        .and_then(|value| value.as_str())
        .ok_or_else(|| {
            format!(
                ".coding/agent.toml providers.{}.{}.cmd is required",
                provider,
                phase.as_str()
            )
        })?
        .to_string();

    let args = match table.get("args") {
        Some(value) => value
            .as_array()
            .ok_or_else(|| {
                format!(
                    ".coding/agent.toml providers.{}.{}.args must be an array",
                    provider,
                    phase.as_str()
                )
            })?
            .iter()
            .map(|item| {
                item.as_str().map(|s| s.to_string()).ok_or_else(|| {
                    format!(
                        ".coding/agent.toml providers.{}.{}.args must contain only strings",
                        provider,
                        phase.as_str()
                    )
                })
            })
            .collect::<CflowResult<Vec<_>>>()?,
        None => Vec::new(),
    };

    let prompt_mode = PromptMode::parse(table.get("prompt_mode").and_then(|value| value.as_str()))?;

    Ok(Some(AgentCommand {
        cmd,
        args,
        prompt_mode,
        source: format!(
            ".coding/agent.toml providers.{}.{}",
            provider,
            phase.as_str()
        ),
    }))
}

fn env_agent_command() -> CflowResult<Option<AgentCommand>> {
    let Ok(cmdline) = env::var("CFLOW_AGENT_CMD") else {
        return Ok(None);
    };
    if cmdline.trim().is_empty() {
        return Err("CFLOW_AGENT_CMD is empty".to_string());
    }

    let parts = shell_words::split(&cmdline)
        .map_err(|error| format!("Invalid CFLOW_AGENT_CMD: {}", error))?;
    let Some((cmd, args)) = parts.split_first() else {
        return Err("CFLOW_AGENT_CMD is empty".to_string());
    };

    Ok(Some(AgentCommand {
        cmd: cmd.to_string(),
        args: args.to_vec(),
        prompt_mode: PromptMode::Arg,
        source: "CFLOW_AGENT_CMD".to_string(),
    }))
}

fn resolve_agent_command(
    provider: &str,
    phase: AgentPhase,
    config: Option<&toml::Value>,
) -> CflowResult<AgentCommand> {
    if provider == "custom" {
        if let Some(command) = env_agent_command()? {
            return Ok(command);
        }
        if let Some(command) = config_agent_command(config, provider, phase)? {
            return Ok(command);
        }
        return Err(format!(
            "Provider 'custom' needs CFLOW_AGENT_CMD or .coding/agent.toml providers.custom.{}",
            phase.as_str()
        ));
    }

    if let Some(command) = config_agent_command(config, provider, phase)? {
        return Ok(command);
    }

    if let Some(command) = builtin_agent_command(provider, phase)? {
        return Ok(command);
    }

    Err(format!(
        "Provider '{}' is not configured for {}. Add .coding/agent.toml providers.{}.{} or use --provider custom.",
        provider,
        phase.as_str(),
        provider,
        phase.as_str()
    ))
}

fn command_exists(cmd: &str) -> bool {
    let cmd_path = Path::new(cmd);
    if cmd_path.components().count() > 1 {
        return cmd_path.exists();
    }

    let Some(paths) = env::var_os("PATH") else {
        return false;
    };

    env::split_paths(&paths).any(|dir| dir.join(cmd).exists())
}

fn display_arg(arg: &str) -> String {
    if arg
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || "-_./:=@".contains(c))
    {
        arg.to_string()
    } else {
        format!("'{}'", arg.replace('\'', "'\\''"))
    }
}

fn display_command(command: &AgentCommand) -> String {
    std::iter::once(display_arg(&command.cmd))
        .chain(command.args.iter().map(|arg| display_arg(arg)))
        .collect::<Vec<_>>()
        .join(" ")
}

fn display_resolved_agent_command(command: &AgentCommand) -> String {
    match command.prompt_mode {
        PromptMode::Arg => format!("{} \"<PROMPT>\"", display_command(command)),
        PromptMode::Stdin => display_command(command),
    }
}

fn run_agent(
    command_cfg: &AgentCommand,
    prompt: &str,
    verbose: bool,
) -> CflowResult<AgentRunOutput> {
    let mut command = Command::new(&command_cfg.cmd);
    command.args(&command_cfg.args);

    match command_cfg.prompt_mode {
        PromptMode::Arg => {
            command.arg(prompt);
            command.stdin(Stdio::null());
        }
        PromptMode::Stdin => {
            command.stdin(Stdio::piped());
        }
    }

    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut child = command.spawn().map_err(|error| {
        format!(
            "Failed to start agent command '{}': {}",
            display_command(command_cfg),
            error
        )
    })?;

    if command_cfg.prompt_mode == PromptMode::Stdin {
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin
                .write_all(prompt.as_bytes())
                .map_err(|error| error.to_string())?;
        }
    }

    let output = child
        .wait_with_output()
        .map_err(|error| error.to_string())?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if verbose {
        eprintln!("Agent command: {}", display_command(command_cfg));
        eprintln!("Agent prompt_mode: {}", command_cfg.prompt_mode.as_str());
        eprintln!("Agent stdout:\n{}", stdout);
        eprintln!("Agent stderr:\n{}", stderr);
    }

    if !output.status.success() {
        let mut message = format!(
            "Agent command failed with exit code: {:?}",
            output.status.code()
        );
        if verbose {
            message.push_str(&format!("\nstdout:\n{}\nstderr:\n{}", stdout, stderr));
        } else {
            message.push_str(". Re-run with --verbose to see captured stdout/stderr.");
        }
        return Err(message);
    }

    Ok(AgentRunOutput { stdout, stderr })
}

fn fenced_json_payload(stdout: &str) -> Option<String> {
    let json_str = stdout.trim();
    if json_str.contains("```json") {
        Some(
            json_str
                .split("```json")
                .nth(1)
                .unwrap_or(json_str)
                .split("```")
                .next()
                .unwrap_or(json_str)
                .trim()
                .to_string(),
        )
    } else {
        None
    }
}

fn looks_like_cflow_agent_json(data: &Value) -> bool {
    data.get("objective").is_some() || data.get("mode").is_some()
}

fn collect_wrapper_text(data: &Value, out: &mut Vec<String>) {
    match data {
        Value::String(value) => out.push(value.clone()),
        Value::Array(items) => {
            for item in items {
                collect_wrapper_text(item, out);
            }
        }
        Value::Object(map) => {
            for key in ["message", "content", "text", "result", "output", "final"] {
                if let Some(value) = map.get(key) {
                    collect_wrapper_text(value, out);
                }
            }
        }
        _ => {}
    }
}

fn balanced_json_object(input: &str) -> Option<String> {
    let mut start = None;
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    let mut last = None;

    for (idx, ch) in input.char_indices() {
        if start.is_none() {
            if ch == '{' {
                start = Some(idx);
                depth = 1;
            }
            continue;
        }

        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        match ch {
            '"' => in_string = true,
            '{' => depth += 1,
            '}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    let begin = start.unwrap_or(0);
                    last = Some(input[begin..=idx].to_string());
                    start = None;
                }
            }
            _ => {}
        }
    }

    last
}

fn agent_json_payload(stdout: &str) -> Option<String> {
    if let Some(payload) = fenced_json_payload(stdout) {
        return Some(payload);
    }

    let trimmed = stdout.trim();
    if trimmed.is_empty() {
        return None;
    }

    if let Ok(data) = serde_json::from_str::<Value>(trimmed) {
        if looks_like_cflow_agent_json(&data) {
            return Some(trimmed.to_string());
        }

        let mut text_values = Vec::new();
        collect_wrapper_text(&data, &mut text_values);
        for value in text_values.into_iter().rev() {
            if let Some(payload) = agent_json_payload(&value) {
                return Some(payload);
            }
        }

        return Some(trimmed.to_string());
    }

    balanced_json_object(trimmed)
}

fn parse_agent_json(output: &AgentRunOutput, verbose: bool) -> CflowResult<Value> {
    let Some(json_str) = agent_json_payload(&output.stdout) else {
        let mut message = "Agent stdout did not contain JSON".to_string();
        if verbose {
            message.push_str(&format!(
                "\nstdout:\n{}\nstderr:\n{}",
                output.stdout, output.stderr
            ));
        } else {
            message.push_str(". Re-run with --verbose to see captured stdout/stderr.");
        }
        return Err(message);
    };

    serde_json::from_str(&json_str).map_err(|error| {
        let mut message = format!("Agent output is not valid JSON: {}", error);
        if verbose {
            message.push_str(&format!(
                "\nExtracted output:\n{}\nstdout:\n{}\nstderr:\n{}",
                json_str, output.stdout, output.stderr
            ));
        } else {
            message.push_str(". Re-run with --verbose to see captured stdout/stderr.");
        }
        message
    })
}

fn command_agent_plan(args: &[String]) -> CflowResult<()> {
    let config = load_agent_config()?;
    let provider = resolve_agent_provider(args, config.as_ref())?;
    let command_cfg = resolve_agent_command(&provider, AgentPhase::Plan, config.as_ref())?;
    let verbose = has_flag(args, "--verbose");
    let task_path = resolve_task(args)?;
    
    let is_story = task_path.contains("/stories/");
    let packet_path = if is_story {
        Path::new(&task_path)
            .parent()
            .and_then(|p| p.parent())
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|| ".coding".to_string())
    } else {
        task_path.clone()
    };

    let request_path = if is_story {
        format!("{}/STORY.md", task_path)
    } else {
        format!("{}/REQUEST.md", task_path)
    };
    if !Path::new(&request_path).exists() {
        return Err(format!(
            "Task folder does not exist or missing {}: {}",
            if is_story { "STORY.md" } else { "REQUEST.md" },
            task_path
        ));
    }

    let request_md = fs::read_to_string(&request_path).map_err(|e| e.to_string())?;
    
    let mut context_info = String::new();
    if is_story {
        let req_p = format!("{}/REQUEST.md", packet_path);
        let intake_p = format!("{}/INTAKE.md", packet_path);
        let packet_p = format!("{}/PACKET.md", packet_path);
        
        if let Ok(c) = fs::read_to_string(req_p) {
            context_info.push_str(&format!("\n# Global REQUEST.md\n\n{}\n", c));
        }
        if let Ok(c) = fs::read_to_string(intake_p) {
            context_info.push_str(&format!("\n# Global INTAKE.md\n\n{}\n", c));
        }
        if let Ok(c) = fs::read_to_string(packet_p) {
            context_info.push_str(&format!("\n# Global PACKET.md\n\n{}\n", c));
        }
    }

    let skill_path = "skills/agent-plan.md";
    let skill = fs::read_to_string(skill_path).unwrap_or_else(|_| "Provide JSON plan.".to_string());

    let prompt = format!(
        "{}\n\n# Provider Safety\n\n- Do not edit files during planning.\n- Return plan JSON only.\n\n# Current {}\n\n{}{}",
        skill,
        if is_story { "STORY.md" } else { "REQUEST.md" },
        request_md,
        context_info
    );

    let output = run_agent(&command_cfg, &prompt, verbose)?;

    let data = parse_agent_json(&output, verbose)?;

    validate_plan(&data)?;
    let output_path = format!("{}/PLAN.md", task_path);
    write_text(&output_path, &render_plan(&data))?;

    let task_id = extract_task_id(&task_path);
    update_task_state(&task_id, Some("planned"), None)?;

    let steps_len = get_path(&data, &["implementation_steps"])
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    let files_len = get_path(&data, &["files_to_change"])
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0);

    println!("Plan created: {}", output_path);
    println!("Provider: {}", provider);
    println!("Implementation steps: {}", steps_len);
    println!("Files expected: {}", files_len);
    println!("Next: cflow agent coding --task current");

    Ok(())
}

fn command_agent_coding(args: &[String]) -> CflowResult<()> {
    let config = load_agent_config()?;
    let provider = resolve_agent_provider(args, config.as_ref())?;
    let command_cfg = resolve_agent_command(&provider, AgentPhase::Coding, config.as_ref())?;
    let verbose = has_flag(args, "--verbose");
    let task_path = resolve_task(args)?;
    let plan_path = format!("{}/PLAN.md", task_path);
    if !Path::new(&plan_path).exists() {
        return Err(format!(
            "Task folder does not exist or missing PLAN.md: {}",
            task_path
        ));
    }

    let is_story = task_path.contains("/stories/");
    let packet_path = if is_story {
        Path::new(&task_path)
            .parent()
            .and_then(|p| p.parent())
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|| ".coding".to_string())
    } else {
        task_path.clone()
    };

    let fix_mode = has_flag(args, "--fix");
    let verify_path = format!("{}/VERIFY.md", task_path);
    let coding_path = format!("{}/CODING.md", task_path);

    if fix_mode && !Path::new(&verify_path).exists() {
        return Err("Coding fix rejected: VERIFY.md is missing".to_string());
    }

    let plan_md = fs::read_to_string(&plan_path).map_err(|e| e.to_string())?;
    let verify_md = if fix_mode {
        Some(fs::read_to_string(&verify_path).map_err(|e| e.to_string())?)
    } else {
        None
    };
    let existing_coding_md = if fix_mode && Path::new(&coding_path).exists() {
        Some(fs::read_to_string(&coding_path).map_err(|e| e.to_string())?)
    } else {
        None
    };

    let mut context_info = String::new();
    if is_story {
        let packet_p = format!("{}/PACKET.md", packet_path);
        let story_p = format!("{}/STORY.md", task_path);
        if let Ok(c) = fs::read_to_string(packet_p) {
            context_info.push_str(&format!("\n# Global PACKET.md\n\n{}\n", c));
        }
        if let Ok(c) = fs::read_to_string(story_p) {
            context_info.push_str(&format!("\n# STORY.md for this Story\n\n{}\n", c));
        }
    }

    let skill_path = "skills/agent-coding.md";
    let skill = fs::read_to_string(skill_path)
        .unwrap_or_else(|_| "Implement and provide JSON coding summary.".to_string());

    let prompt = if fix_mode {
        format!(
            "{skill}\n\n# Mode\n\nfix\n\n# Fix Instructions\n\n- Do not re-plan.\n- Do not broaden scope.\n- Fix only findings from VERIFY.md.\n- Preserve already-correct work.\n- Return coding JSON only.\n- Do not edit {task_path}/*.md artifacts.\n- Do not verify.\n- Do not ship.\n- Do not commit.\n\n# Current PLAN.md\n\n{plan_md}\n\n# Latest VERIFY.md\n\n{}\n\n# Existing CODING.md\n\n{}{}",
            verify_md.as_deref().unwrap_or(""),
            existing_coding_md.as_deref().unwrap_or("_None_"),
            context_info
        )
    } else {
        format!(
            "{skill}\n\n# Mode\n\ninitial\n\n# Coding Instructions\n\n- Return coding JSON only.\n- Do not edit {task_path}/*.md artifacts.\n- Do not verify.\n- Do not ship.\n- Do not commit.\n\n# Current PLAN.md\n\n{plan_md}{}",
            context_info
        )
    };

    let output = run_agent(&command_cfg, &prompt, verbose)?;

    let data = parse_agent_json(&output, verbose)?;

    validate_coding_for_task(&data, &task_path)?;
    let output_path = format!("{}/CODING.md", task_path);
    write_text(&output_path, &render_coding(&data))?;

    let task_id = extract_task_id(&task_path);
    update_task_state(&task_id, Some("coding_done"), None)?;

    let mode = option_to_js_string(get_path(&data, &["mode"]));
    let status = option_to_js_string(get_path(&data, &["status"]));
    let files_len = get_path(&data, &["changed_files"])
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    let fixed_len = get_path(&data, &["fixed_findings"])
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0);

    if fix_mode {
        println!("Coding fix completed: {}", output_path);
        println!("Provider: {}", provider);
        println!("Mode: {}", mode);
        println!("Fixed findings: {}", fixed_len);
        println!("Status: {}", status);
    } else {
        println!("Coding completed: {}", output_path);
        println!("Provider: {}", provider);
        println!("Mode: {}", mode);
        println!("Status: {}", status);
        println!("Changed files: {}", files_len);
    }
    println!("Next: cflow verify --task current");

    Ok(())
}

fn command_agent_providers() -> CflowResult<()> {
    let config = load_agent_config()?;
    let default_provider = resolve_agent_provider(&[], config.as_ref())?;

    println!("Default provider: {}", default_provider);
    println!("Providers:");

    for provider in AGENT_PROVIDERS {
        let plan = resolve_agent_command(provider, AgentPhase::Plan, config.as_ref()).ok();
        let coding = resolve_agent_command(provider, AgentPhase::Coding, config.as_ref()).ok();
        let plan_status = plan
            .as_ref()
            .map(|command| {
                if command_exists(&command.cmd) {
                    "exists"
                } else {
                    "missing"
                }
            })
            .unwrap_or("unconfigured");
        let coding_status = coding
            .as_ref()
            .map(|command| {
                if command_exists(&command.cmd) {
                    "exists"
                } else {
                    "missing"
                }
            })
            .unwrap_or("unconfigured");

        println!(
            "- {}: plan={}, coding={}",
            provider, plan_status, coding_status
        );
    }

    Ok(())
}

fn command_agent_doctor(args: &[String]) -> CflowResult<()> {
    let config = load_agent_config()?;
    let provider = if get_arg(args, "--provider", "").is_empty() {
        resolve_agent_provider(args, config.as_ref())?
    } else {
        normalize_agent_provider(&get_arg(args, "--provider", ""))?
    };

    println!("Provider: {}", provider);
    for phase in [AgentPhase::Plan, AgentPhase::Coding] {
        match resolve_agent_command(&provider, phase, config.as_ref()) {
            Ok(command) => {
                println!("{}:", phase.as_str());
                println!("  source: {}", command.source);
                println!("  command: {}", display_resolved_agent_command(&command));
                println!("  prompt_mode: {}", command.prompt_mode.as_str());
                println!(
                    "  binary: {}",
                    if command_exists(&command.cmd) {
                        "exists"
                    } else {
                        "missing"
                    }
                );
            }
            Err(error) => {
                println!("{}: unconfigured ({})", phase.as_str(), error);
            }
        }
    }

    Ok(())
}

fn command_agent(args: &[String]) -> CflowResult<()> {
    if args.is_empty() {
        return Err(
            "Usage: cflow agent plan|coding [--task current] [--provider name] [--fix] [--verbose]"
                .to_string(),
        );
    }

    match args[0].as_str() {
        "plan" => command_agent_plan(&args[1..]),
        "coding" => command_agent_coding(&args[1..]),
        "providers" => command_agent_providers(),
        "doctor" => command_agent_doctor(&args[1..]),
        _ => Err(format!("Unknown agent command: {}", args[0])),
    }
}


const INTAKE_INPUT_TYPES: &[&str] = &[
    "new_spec", "spec_slice", "change_request", "new_feature", "bug_fix",
    "refactor", "maintenance", "workflow_improvement", "documentation",
    "test_only", "question", "unclear"
];

const INTAKE_LANES: &[&str] = &[
    "tiny", "normal", "high_risk", "needs_clarification", "none"
];

const INTAKE_NEXT_ACTIONS: &[&str] = &[
    "answer_directly", "clarify", "task_flow", "packet_brief", "packet_split", "story_flow", "none"
];

fn resolve_packet(args: &[String]) -> CflowResult<String> {
    let packet = get_arg(args, "--packet", "current");
    if packet == "current" {
        let state = load_state();
        if let Some(id) = state["current_packet_id"].as_str() {
            return Ok(format!(".coding/packets/{}", id));
        }
        Err("No current packet. Run `cflow packet new \"<title>\"` first.".to_string())
    } else if packet.starts_with(".coding/packets/") {
        Ok(packet)
    } else {
        Ok(format!(".coding/packets/{}", packet))
    }
}

fn resolve_story_path(args: &[String]) -> CflowResult<(String, String)> {
    let state = load_state();
    let packet_id = match state["current_packet_id"].as_str() {
        Some(id) => id.to_string(),
        None => return Err("No current packet. Run `cflow packet new` first.".to_string()),
    };
    
    let story = get_arg(args, "--story", "current");
    let story_id = if story == "current" {
        match state["current_story_id"].as_str() {
            Some(id) => id.to_string(),
            None => return Err("No current story. Use `cflow story switch <story-id>` first.".to_string()),
        }
    } else {
        story
    };
    
    let stories_dir = format!(".coding/packets/{}/stories", packet_id);
    if !Path::new(&stories_dir).exists() {
        return Err(format!("Stories directory does not exist: {}", stories_dir));
    }
    
    let entries = fs::read_dir(&stories_dir).map_err(|e| e.to_string())?;
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name == story_id || name.starts_with(&format!("{}-", story_id)) {
                        return Ok((packet_id, name.to_string()));
                    }
                }
            }
        }
    }
    
    Ok((packet_id, story_id))
}

fn validate_intake(data: &Value) -> CflowResult<()> {
    require_field(get_path(data, &["request_summary"]), "request_summary")?;
    require_field(get_path(data, &["input_type"]), "input_type")?;
    require_field(get_path(data, &["lane"]), "lane")?;
    require_field(get_path(data, &["next_action"]), "next_action")?;

    assert_allowed(get_path(data, &["input_type"]), INTAKE_INPUT_TYPES, "input_type")?;
    assert_allowed(get_path(data, &["lane"]), INTAKE_LANES, "lane")?;
    assert_allowed(get_path(data, &["next_action"]), INTAKE_NEXT_ACTIONS, "next_action")?;

    let lane = option_to_js_string(get_path(data, &["lane"]));
    if lane == "needs_clarification" && is_empty(get_path(data, &["clarifying_questions"])) {
        return Err("Invalid intake: lane=needs_clarification requires clarifying_questions".to_string());
    }

    Ok(())
}

fn classify_intake(data: &mut Value) {
    let risk_flags = data.get("risk_flags")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let hard_gates = data.get("hard_gates")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
        
    let mut lane = option_to_js_string(data.get("lane"));
    if lane == "none" || lane.is_empty() || lane == "null" {
        if !hard_gates.is_empty() {
            lane = "high_risk".to_string();
        } else {
            let count = risk_flags.len();
            if count >= 4 {
                lane = "high_risk".to_string();
            } else if count >= 2 {
                lane = "normal".to_string();
            } else {
                lane = "normal".to_string();
            }
        }
        data["lane"] = Value::String(lane.clone());
    }
    
    if data.get("split_required").is_none() {
        let split = if lane == "high_risk" {
            true
        } else if lane == "tiny" {
            false
        } else {
            true
        };
        data["split_required"] = Value::Bool(split);
    }
}

fn render_intake(data: &Value) -> String {
    let split_required = if matches!(
        get_path(data, &["split_required"]),
        Some(Value::Bool(true))
    ) {
        "true"
    } else {
        "false"
    };

    format!(
        "# Request Intake\n\n## Request Summary\n\n{}\n\n## Input Type\n\n{}\n\n## Lane\n\n{}\n\n## Risk Flags\n\n{}\n\n## Hard Gates\n\n{}\n\n## Split Required\n\n{}\n\n## Reason\n\n{}\n\n## Next Action\n\n{}\n\n## Assumptions\n\n{}\n\n## Clarifying Questions\n\n{}\n",
        option_to_js_string(get_path(data, &["request_summary"])),
        option_to_js_string(get_path(data, &["input_type"])),
        option_to_js_string(get_path(data, &["lane"])),
        list(get_path(data, &["risk_flags"])),
        list(get_path(data, &["hard_gates"])),
        split_required,
        option_to_js_string(get_path(data, &["reason"])),
        option_to_js_string(get_path(data, &["next_action"])),
        list(get_path(data, &["assumptions"])),
        list(get_path(data, &["clarifying_questions"]))
    )
}

fn validate_packet_brief(data: &Value) -> CflowResult<()> {
    require_field(get_path(data, &["goal"]), "goal")?;
    require_field(get_path(data, &["scope"]), "scope")?;
    require_field(get_path(data, &["scope", "in"]), "scope.in")?;
    require_field(get_path(data, &["scope", "out"]), "scope.out")?;
    require_field(get_path(data, &["global_acceptance_criteria"]), "global_acceptance_criteria")?;
    Ok(())
}

fn render_packet_brief(data: &Value) -> String {
    format!(
        "# Packet\n\n## Goal\n\n{}\n\n## Scope\n\n### In Scope\n\n{}\n\n### Out of Scope\n\n{}\n\n## Global Acceptance Criteria\n\n{}\n\n## Technical Constraints\n\n{}\n\n## Shared Data / Contracts\n\n{}\n\n## Validation Strategy\n\n{}\n",
        option_to_js_string(get_path(data, &["goal"])),
        list(get_path(data, &["scope", "in"])),
        list(get_path(data, &["scope", "out"])),
        list(get_path(data, &["global_acceptance_criteria"])),
        list(get_path(data, &["technical_constraints"])),
        list(get_path(data, &["shared_data_contracts"])),
        list(get_path(data, &["validation_strategy"]))
    )
}

fn validate_split(data: &Value) -> CflowResult<()> {
    require_field(get_path(data, &["stories"]), "stories")?;
    let Some(stories) = data["stories"].as_array() else {
        return Err("Invalid split input: 'stories' must be an array".to_string());
    };
    for (idx, story) in stories.iter().enumerate() {
        require_field(story.get("id"), &format!("stories[{}].id", idx))?;
        require_field(story.get("title"), &format!("stories[{}].title", idx))?;
        require_field(story.get("description"), &format!("stories[{}].description", idx))?;
        require_field(story.get("acceptance_criteria"), &format!("stories[{}].acceptance_criteria", idx))?;
    }
    Ok(())
}

fn render_stories_index(stories: &Value) -> String {
    let mut index = "# Stories\n\n".to_string();
    if let Some(arr) = stories.as_array() {
        for item in arr {
            let id = option_to_js_string(item.get("id"));
            let title = option_to_js_string(item.get("title"));
            index.push_str(&format!("- [{id} - {title}](stories/{id}/STORY.md)\n"));
        }
    } else {
        index.push_str("- _None_\n");
    }
    index
}

fn render_story_md(story: &Value) -> String {
    format!(
        "# Story: {}\n\n## Description\n\n{}\n\n## Acceptance Criteria\n\n{}\n\n## Files to Change\n\n{}\n",
        option_to_js_string(story.get("title")),
        option_to_js_string(story.get("description")),
        list(story.get("acceptance_criteria")),
        list(story.get("files_to_change"))
    )
}

fn validate_packet_verify(data: &Value) -> CflowResult<()> {
    require_field(get_path(data, &["status"]), "status")?;
    assert_allowed(get_path(data, &["status"]), &["passed", "failed"], "status")?;
    Ok(())
}

fn render_packet_verify(data: &Value) -> String {
    format!(
        "# Packet Verify\n\n## Status\n\n{}\n\n## Goal Achieved\n\n{}\n\n## Regressions Checked\n\n{}\n\n## Findings\n\n{}\n",
        option_to_js_string(get_path(data, &["status"])),
        option_to_js_string(get_path(data, &["goal_achieved"])),
        option_to_js_string(get_path(data, &["regressions_checked"])),
        list(get_path(data, &["findings"]))
    )
}

fn render_packet_ship(data: &Value) -> String {
    format!(
        "# Packet Ship\n\n## Status\n\nshipped\n\n## Changelog\n\n{}\n\n## Commit Message\n\n```text\n{}\n```\n",
        list(get_path(data, &["changelog"])),
        option_to_js_string(get_path(data, &["commit_message"]))
    )
}

fn validate_packet_ship(data: &Value) -> CflowResult<()> {
    require_field(get_path(data, &["commit_message"]), "commit_message")?;
    Ok(())
}

fn command_packet_new(args: &[String]) -> CflowResult<()> {
    let title = args.first().cloned().unwrap_or_default();
    if title.is_empty() {
        return Err("Missing packet title. Usage: cflow packet new \"<title>\"".to_string());
    }

    let now = chrono::Local::now();
    let timestamp = now.format("%Y%m%d-%H%M%S").to_string();
    let slug = slugify(&title);
    let packet_id = if slug.is_empty() {
        timestamp.clone()
    } else {
        format!("{}-{}", timestamp, slug)
    };

    let packet_path = format!("packets/{}", packet_id);
    let full_path = format!(".coding/{}", packet_path);

    ensure_dir(&format!("{}/.placeholder", full_path))?;

    let mut state = load_state();
    state["current_packet_id"] = Value::String(packet_id.clone());
    state["current_story_id"] = Value::Null;
    state["current_task_id"] = Value::Null;
    
    let mut packet_meta = serde_json::Map::new();
    packet_meta.insert("title".to_string(), Value::String(title.clone()));
    packet_meta.insert("status".to_string(), Value::String("in_progress".to_string()));
    packet_meta.insert("phase".to_string(), Value::String("new".to_string()));
    packet_meta.insert("lane".to_string(), Value::String("none".to_string()));
    packet_meta.insert("split_required".to_string(), Value::Bool(false));
    packet_meta.insert("risk_flags".to_string(), Value::Array(Vec::new()));
    packet_meta.insert("hard_gates".to_string(), Value::Array(Vec::new()));
    packet_meta.insert("next_action".to_string(), Value::String("intake".to_string()));
    let now_str = now.to_rfc3339();
    packet_meta.insert("created_at".to_string(), Value::String(now_str.clone()));
    packet_meta.insert("updated_at".to_string(), Value::String(now_str));
    
    let mut artifacts = serde_json::Map::new();
    artifacts.insert("request".to_string(), Value::Bool(false));
    artifacts.insert("intake".to_string(), Value::Bool(false));
    artifacts.insert("packet".to_string(), Value::Bool(false));
    artifacts.insert("stories".to_string(), Value::Bool(false));
    artifacts.insert("packet_verify".to_string(), Value::Bool(false));
    artifacts.insert("packet_ship".to_string(), Value::Bool(false));
    packet_meta.insert("artifacts".to_string(), Value::Object(artifacts));
    
    packet_meta.insert("stories".to_string(), Value::Object(serde_json::Map::new()));

    state["packets"][&packet_id] = Value::Object(packet_meta);
    save_state(&state)?;

    write_text(".coding/current", &packet_path)?;

    println!("Packet created: {}", packet_id);
    println!("Path: {}", full_path);

    Ok(())
}

fn command_packet_intake(args: &[String]) -> CflowResult<()> {
    let packet_path = resolve_packet(args)?;
    if !Path::new(&packet_path).exists() {
        return Err(format!("Packet folder does not exist: {}", packet_path));
    }
    let output = format!("{}/INTAKE.md", packet_path);
    let mut data = read_json_input(args)?;
    classify_intake(&mut data);
    validate_intake(&data)?;
    write_text(&output, &render_intake(&data))?;
    println!("created {}", output);

    let packet_id = extract_task_id(&packet_path);
    let mut state = load_state();
    
    if let Some(packet_meta) = state["packets"].get_mut(&packet_id) {
        packet_meta["lane"] = data["lane"].clone();
        packet_meta["risk_flags"] = data["risk_flags"].clone();
        packet_meta["hard_gates"] = data["hard_gates"].clone();
        packet_meta["split_required"] = data["split_required"].clone();
        packet_meta["next_action"] = data["next_action"].clone();
        packet_meta["artifacts"]["intake"] = Value::Bool(true);
        packet_meta["phase"] = Value::String("intake_done".to_string());
        packet_meta["updated_at"] = Value::String(chrono::Local::now().to_rfc3339());
        let has_req = Path::new(&format!("{}/REQUEST.md", packet_path)).exists();
        packet_meta["artifacts"]["request"] = Value::Bool(has_req);
    }
    
    save_state(&state)?;
    Ok(())
}

fn command_packet_brief(args: &[String]) -> CflowResult<()> {
    let packet_path = resolve_packet(args)?;
    if !Path::new(&packet_path).exists() {
        return Err(format!("Packet folder does not exist: {}", packet_path));
    }
    let output = format!("{}/PACKET.md", packet_path);
    let data = read_json_input(args)?;
    validate_packet_brief(&data)?;
    write_text(&output, &render_packet_brief(&data))?;
    println!("created {}", output);

    let packet_id = extract_task_id(&packet_path);
    let mut state = load_state();
    if let Some(packet_meta) = state["packets"].get_mut(&packet_id) {
        packet_meta["artifacts"]["packet"] = Value::Bool(true);
        packet_meta["phase"] = Value::String("packet_briefed".to_string());
        packet_meta["updated_at"] = Value::String(chrono::Local::now().to_rfc3339());
    }
    save_state(&state)?;
    Ok(())
}

fn command_packet_split(args: &[String]) -> CflowResult<()> {
    let packet_path = resolve_packet(args)?;
    if !Path::new(&packet_path).exists() {
        return Err(format!("Packet folder does not exist: {}", packet_path));
    }
    
    let data = read_json_input(args)?;
    validate_split(&data)?;
    
    let packet_id = extract_task_id(&packet_path);
    let mut state = load_state();
    
    let stories = data["stories"].as_array().unwrap();
    
    let index_output = format!("{}/STORIES.md", packet_path);
    write_text(&index_output, &render_stories_index(&data["stories"]))?;
    
    let mut state_stories = serde_json::Map::new();
    for story in stories {
        let id = option_to_js_string(story.get("id"));
        let title = option_to_js_string(story.get("title"));
        
        let story_dir = format!("{}/stories/{}", packet_path, id);
        let story_file = format!("{}/STORY.md", story_dir);
        
        ensure_dir(&format!("{}/.placeholder", story_dir))?;
        write_text(&story_file, &render_story_md(story))?;
        
        let mut story_meta = serde_json::Map::new();
        story_meta.insert("title".to_string(), Value::String(title));
        story_meta.insert("status".to_string(), Value::String("todo".to_string()));
        story_meta.insert("phase".to_string(), Value::String("new".to_string()));
        story_meta.insert("findings_count".to_string(), Value::Number(0.into()));
        state_stories.insert(id, Value::Object(story_meta));
    }
    
    if let Some(packet_meta) = state["packets"].get_mut(&packet_id) {
        packet_meta["artifacts"]["stories"] = Value::Bool(true);
        packet_meta["phase"] = Value::String("split_done".to_string());
        packet_meta["stories"] = Value::Object(state_stories);
        packet_meta["updated_at"] = Value::String(chrono::Local::now().to_rfc3339());
    }
    
    save_state(&state)?;
    println!("created {}/STORIES.md and story subfolders", packet_path);
    Ok(())
}

fn command_packet_status() {
    let state = load_state();
    let Some(packet_id) = state["current_packet_id"].as_str() else {
        println!("No current packet.");
        return;
    };
    let Some(packet) = state["packets"].get(packet_id) else {
        println!("Warning: Packet metadata not found in state.json.");
        return;
    };
    
    println!("Packet: {}", packet_id);
    println!("Title: {}", option_to_js_string(packet.get("title")));
    println!("Lane: {}", option_to_js_string(packet.get("lane")));
    println!("Phase: {}", option_to_js_string(packet.get("phase")));
    println!("\nStories:");
    if let Some(stories) = packet.get("stories").and_then(|s| s.as_object()) {
        if stories.is_empty() {
            println!("- No stories defined yet.");
        } else {
            let mut sorted_stories: Vec<_> = stories.iter().collect();
            sorted_stories.sort_by_key(|(id, _)| *id);
            for (id, meta) in sorted_stories {
                println!(
                    "- {id}: {} ({})",
                    option_to_js_string(meta.get("status")),
                    option_to_js_string(meta.get("phase"))
                );
            }
        }
    } else {
        println!("- No stories defined yet.");
    }
}

fn command_packet_verify(args: &[String]) -> CflowResult<()> {
    let packet_path = resolve_packet(args)?;
    if !Path::new(&packet_path).exists() {
        return Err(format!("Packet folder does not exist: {}", packet_path));
    }
    let output = format!("{}/PACKET_VERIFY.md", packet_path);
    let data = read_json_input(args)?;
    validate_packet_verify(&data)?;
    write_text(&output, &render_packet_verify(&data))?;
    println!("created {}", output);

    let packet_id = extract_task_id(&packet_path);
    let mut state = load_state();
    let status = option_to_js_string(get_path(&data, &["status"]));
    let phase = if status == "passed" {
        "packet_verify_passed"
    } else {
        "packet_verify_failed"
    };
    if let Some(packet_meta) = state["packets"].get_mut(&packet_id) {
        packet_meta["artifacts"]["packet_verify"] = Value::Bool(true);
        packet_meta["phase"] = Value::String(phase.to_string());
        packet_meta["updated_at"] = Value::String(chrono::Local::now().to_rfc3339());
    }
    save_state(&state)?;
    Ok(())
}

fn command_packet_ship(args: &[String]) -> CflowResult<()> {
    let packet_path = resolve_packet(args)?;
    if !Path::new(&packet_path).exists() {
        return Err(format!("Packet folder does not exist: {}", packet_path));
    }
    
    let verify_path = format!("{}/PACKET_VERIFY.md", packet_path);
    if !Path::new(&verify_path).exists() {
        return Err("Ship rejected: PACKET_VERIFY.md is missing".to_string());
    }
    let verify_content = fs::read_to_string(&verify_path).map_err(|e| e.to_string())?;
    let verify_status = first_non_empty_section_line(&verify_content, "Status")
        .ok_or_else(|| "Ship rejected: PACKET_VERIFY.md status is missing".to_string())?;
    if verify_status != "passed" {
        return Err(format!("Ship rejected: PACKET_VERIFY.md status must be passed (found {verify_status})"));
    }

    let data = read_json_input(args)?;
    validate_packet_ship(&data)?;

    let output = format!("{}/PACKET_SHIP.md", packet_path);
    write_text(&output, &render_packet_ship(&data))?;
    println!("created {}", output);

    let packet_id = extract_task_id(&packet_path);
    let mut state = load_state();
    
    if let Some(packet_meta) = state["packets"].get_mut(&packet_id) {
        packet_meta["artifacts"]["packet_ship"] = Value::Bool(true);
        packet_meta["phase"] = Value::String("committed".to_string());
        packet_meta["status"] = Value::String("done".to_string());
        packet_meta["updated_at"] = Value::String(chrono::Local::now().to_rfc3339());
    }
    
    save_state(&state)?;
    
    let commit_mode = has_flag(args, "--commit");
    if commit_mode {
        let commit_message = option_to_js_string(get_path(&data, &["commit_message"]));
        let output_git = Command::new("git")
            .args(["commit", "-a", "-m", &commit_message])
            .output()
            .map_err(|e| e.to_string())?;
        if !output_git.status.success() {
            return Err(format!(
                "Git commit failed:\n{}",
                String::from_utf8_lossy(&output_git.stderr)
            ));
        }
        println!("Git commit successful!");
    } else {
        println!("Dry-run mode. Run with --commit to apply git changes.");
    }
    Ok(())
}

fn command_packet(args: &[String]) -> CflowResult<()> {
    if args.is_empty() {
        return Err("Usage: cflow packet new|intake|brief|split|status|verify|ship".to_string());
    }
    match args[0].as_str() {
        "new" => command_packet_new(&args[1..]),
        "intake" => command_packet_intake(&args[1..]),
        "brief" => command_packet_brief(&args[1..]),
        "split" => command_packet_split(&args[1..]),
        "status" => {
            command_packet_status();
            Ok(())
        }
        "verify" => command_packet_verify(&args[1..]),
        "ship" => command_packet_ship(&args[1..]),
        _ => Err(format!("Unknown packet command: {}", args[0])),
    }
}

fn command_story_list() -> CflowResult<()> {
    let state = load_state();
    let Some(packet_id) = state["current_packet_id"].as_str() else {
        println!("No current packet.");
        return Ok(());
    };
    let Some(packet) = state["packets"].get(packet_id) else {
        println!("Warning: Packet metadata not found in state.json.");
        return Ok(());
    };
    
    println!("Stories for packet {}:", packet_id);
    if let Some(stories) = packet.get("stories").and_then(|s| s.as_object()) {
        if stories.is_empty() {
            println!("No stories defined yet.");
        } else {
            let mut sorted_stories: Vec<_> = stories.iter().collect();
            sorted_stories.sort_by_key(|(id, _)| *id);
            for (id, meta) in sorted_stories {
                let current_marker = if state["current_story_id"].as_str() == Some(id.as_str()) {
                    "*"
                } else {
                    " "
                };
                println!(
                    "{} {id}: {} ({})",
                    current_marker,
                    option_to_js_string(meta.get("status")),
                    option_to_js_string(meta.get("phase"))
                );
            }
        }
    }
    Ok(())
}

fn command_story_switch(args: &[String]) -> CflowResult<()> {
    let target = args.first().cloned().unwrap_or_default();
    if target.is_empty() {
        return Err("Usage: cflow story switch <story-id>".to_string());
    }
    
    let mut state = load_state();
    let packet_id = match state["current_packet_id"].as_str() {
        Some(id) => id.to_string(),
        None => return Err("No current packet selected.".to_string()),
    };
    
    let matched_id = {
        let packet = state["packets"].get(&packet_id).ok_or("Current packet not found in state.")?;
        let stories = packet["stories"].as_object().ok_or("No stories in current packet.")?;
        
        let mut found = None;
        for id in stories.keys() {
            if id == &target || id.starts_with(&format!("{}-", target)) {
                found = Some(id.clone());
                break;
            }
        }
        found.ok_or_else(|| format!("Story '{}' not found.", target))?
    };
    
    state["current_story_id"] = Value::String(matched_id.clone());
    save_state(&state)?;
    
    let story_dir = format!("packets/{}/stories/{}", packet_id, matched_id);
    write_text(".coding/current", &story_dir)?;
    
    println!("Switched to story: {}", matched_id);
    Ok(())
}


fn command_story_status() -> CflowResult<()> {
    let state = load_state();
    let Some(packet_id) = state["current_packet_id"].as_str() else {
        println!("No current packet.");
        return Ok(());
    };
    let Some(story_id) = state["current_story_id"].as_str() else {
        println!("No current story selected.");
        return Ok(());
    };
    
    let packet = &state["packets"][packet_id];
    let story = &packet["stories"][story_id];
    
    println!("Current story: {}", story_id);
    println!("Title: {}", option_to_js_string(story.get("title")));
    println!("Status: {}", option_to_js_string(story.get("status")));
    println!("Phase: {}", option_to_js_string(story.get("phase")));
    println!("Findings: {}", option_to_js_string(story.get("findings_count")));
    
    Ok(())
}

fn command_story_plan(args: &[String]) -> CflowResult<()> {
    let mut new_args = args.to_vec();
    new_args.push("--story".to_string());
    new_args.push("current".to_string());
    command_plan(&new_args)
}

fn command_story_coding(args: &[String]) -> CflowResult<()> {
    let mut new_args = args.to_vec();
    new_args.push("--story".to_string());
    new_args.push("current".to_string());
    command_coding(&new_args)
}

fn command_story_verify(args: &[String]) -> CflowResult<()> {
    let mut new_args = args.to_vec();
    new_args.push("--story".to_string());
    new_args.push("current".to_string());
    command_verify(&new_args)
}

fn command_story_ship(args: &[String]) -> CflowResult<()> {
    let mut new_args = args.to_vec();
    new_args.push("--story".to_string());
    new_args.push("current".to_string());
    command_ship(&new_args)
}

fn command_story(args: &[String]) -> CflowResult<()> {
    if args.is_empty() {
        return Err("Usage: cflow story list|switch|status|plan|coding|verify|ship|agent".to_string());
    }
    match args[0].as_str() {
        "list" => command_story_list(),
        "switch" => command_story_switch(&args[1..]),
        "status" => command_story_status(),
        "plan" => command_story_plan(&args[1..]),
        "coding" => command_story_coding(&args[1..]),
        "verify" => command_story_verify(&args[1..]),
        "ship" => command_story_ship(&args[1..]),
        "agent" => {
            if args.len() < 2 {
                return Err("Usage: cflow story agent plan|coding ...".to_string());
            }
            match args[1].as_str() {
                "plan" => {
                    let mut new_args = args[2..].to_vec();
                    new_args.push("--story".to_string());
                    new_args.push("current".to_string());
                    command_agent_plan(&new_args)
                }
                "coding" => {
                    let mut new_args = args[2..].to_vec();
                    new_args.push("--story".to_string());
                    new_args.push("current".to_string());
                    command_agent_coding(&new_args)
                }
                _ => Err(format!("Unknown story agent command: {}", args[1])),
            }
        }
        _ => Err(format!("Unknown story command: {}", args[0])),
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
    let task_id = if slug.is_empty() {
        timestamp.clone()
    } else {
        format!("{}-{}", timestamp, slug)
    };

    let task_path = format!("tasks/{}", task_id);
    let full_path = format!(".coding/{}", task_path);

    ensure_dir(&format!("{}/.placeholder", full_path))?;

    let mut state = load_state();
    state["current_task_id"] = Value::String(task_id.clone());
    save_state(&state)?;

    update_task_state(&task_id, Some("new"), Some(&name))?;

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

    let task_id = extract_task_id(&task_path);
    update_task_state(&task_id, Some("requested"), None)?;

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

    let task_id = extract_task_id(&task_path);
    update_task_state(&task_id, Some("planned"), None)?;

    Ok(())
}

fn command_coding(args: &[String]) -> CflowResult<()> {
    let task_path = resolve_task(args)?;
    if !Path::new(&task_path).exists() {
        return Err(format!("Task folder does not exist: {}", task_path));
    }
    let output = format!("{}/CODING.md", task_path);
    let data = read_json_input(args)?;
    validate_coding_for_task(&data, &task_path)?;
    write_text(&output, &render_coding(&data))?;
    println!("created {}", output);

    let task_id = extract_task_id(&task_path);
    update_task_state(&task_id, Some("coding_done"), None)?;

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

    let task_id = extract_task_id(&task_path);
    let verify_status = option_to_js_string(get_path(&data, &["status"]));
    let phase = if verify_status == "passed" {
        "verify_passed"
    } else {
        "verify_failed"
    };
    update_task_state(&task_id, Some(phase), None)?;

    println!(
        "status={}",
        option_to_js_string(get_path(&data, &["status"]))
    );
    Ok(())
}

fn ensure_verify_passed_for_ship(task_path: &str) -> CflowResult<()> {
    let verify_path = format!("{}/VERIFY.md", task_path);
    if !Path::new(&verify_path).exists() {
        return Err("Ship rejected: VERIFY.md is missing".to_string());
    }

    let content = fs::read_to_string(&verify_path).map_err(|error| error.to_string())?;
    let status = first_non_empty_section_line(&content, "Status")
        .ok_or_else(|| "Ship rejected: VERIFY.md status is missing".to_string())?;

    if status != "passed" {
        return Err(format!(
            "Ship rejected: VERIFY.md status must be passed (found {status})"
        ));
    }

    if verify_findings_count(&content) > 0 {
        return Err("Ship rejected: VERIFY.md has findings".to_string());
    }

    Ok(())
}

fn command_ship(args: &[String]) -> CflowResult<()> {
    let task_path = resolve_task(args)?;
    if !Path::new(&task_path).exists() {
        return Err(format!("Task folder does not exist: {}", task_path));
    }

    let dry_run = has_flag(args, "--dry-run");
    let commit = has_flag(args, "--commit");
    let output = format!("{}/SHIP.md", task_path);
    let existing_ship = Path::new(&output).exists();
    let data = read_optional_json_input(args)?;

    if !dry_run && !commit && data.is_none() {
        if existing_ship {
            println!("SHIP.md already exists.");
            println!("Next options:");
            println!("- cflow ship --task current --dry-run");
        } else {
            println!("SHIP.md does not exist.");
            println!("Provide ship JSON via stdin or --input to create it.");
        }
        return Ok(());
    }

    ensure_verify_passed_for_ship(&task_path)?;

    let coding_path = format!("{}/CODING.md", task_path);
    if !Path::new(&coding_path).exists() {
        println!("Warning: CODING.md is missing.");
    }

    if let Some(data) = data {
        validate_ship(&data)?;
        write_text(&output, &render_ship(&data))?;
        println!("created {}", output);
    } else if !existing_ship {
        return Err(
            "Ship rejected: SHIP.md is missing; provide ship JSON via stdin or --input".to_string(),
        );
    } else {
        println!("using existing {}", output);
    }

    if dry_run {
        if is_git_repo() {
            git(["status", "--short"])?;
        } else {
            println!("not a git repository; skipped git status");
        }
    }

    if commit {
        if !is_git_repo() {
            return Err("Ship rejected: not a git repository".to_string());
        }

        let ship_content = fs::read_to_string(&output).map_err(|error| error.to_string())?;
        let subject = parse_ship_commit_message(&ship_content)
            .ok_or_else(|| "Ship rejected: SHIP.md commit message is missing".to_string())?;

        git(["add", "."])?;
        git(["commit", "-m", subject.as_str()])?;
    }

    let task_id = extract_task_id(&task_path);
    let mut phase = "commit_pending";
    let mut committed = false;
    let mut commit_sha = None;

    if commit {
        phase = "committed";
        committed = true;
        commit_sha = get_git_commit_sha();
    } else if dry_run {
        phase = "commit_pending";
    }

    let mut state = load_state();
    if let Some(task) = state["tasks"].get_mut(&task_id) {
        task["committed"] = Value::Bool(committed);
        if let Some(sha) = commit_sha {
            task["commit_sha"] = Value::String(sha);
        } else {
            task["commit_sha"] = Value::Null;
        }
    }
    save_state(&state)?;

    update_task_state(&task_id, Some(phase), None)?;

    if !dry_run && !commit {
        println!("ship artifact created. use --dry-run for git status.");
    }

    Ok(())
}

fn command_status() {
    let state = load_state();
    
    if let Some(packet_id) = state["current_packet_id"].as_str() {
        if !packet_id.is_empty() {
            if let Some(packet) = state["packets"].get(packet_id) {
                println!("Current packet: {}", packet_id);
                println!("Title: {}", option_to_js_string(packet.get("title")));
                println!("Lane: {}", option_to_js_string(packet.get("lane")));
                println!("Phase: {}", option_to_js_string(packet.get("phase")));
                let split_required = packet.get("split_required").and_then(|v| v.as_bool()).unwrap_or(false);
                println!("Split required: {}", split_required);
                
                println!();
                if let Some(story_id) = state["current_story_id"].as_str() {
                    if !story_id.is_empty() {
                        if let Some(story) = packet["stories"].get(story_id) {
                            println!("Current story: {}", story_id);
                            println!("Story phase: {}", option_to_js_string(story.get("phase")));
                            println!("Findings: {}", option_to_js_string(story.get("findings_count")));
                        }
                    } else {
                        println!("No current story selected.");
                    }
                } else {
                    println!("No current story selected.");
                }
                
                let packet_path = format!(".coding/packets/{}", packet_id);
                let next = determine_next_action(&packet_path);
                println!();
                println!("Next:\n{}", next.command);
                return;
            }
        }
    }

    let current_task_id = state["current_task_id"].as_str().unwrap_or("");

    if current_task_id.is_empty() {
        if Path::new(".coding/current").exists() {
            println!("Warning: .coding/current exists but state.json is missing or lacks current_task_id.");
            println!("Suggest running `cflow state repair`.");
        } else {
            println!("No current task. Run `cflow new \"<task-name>\"` first.");
        }
        return;
    }

    println!("Current task: {}", current_task_id);
    let task_path = format!(".coding/tasks/{}", current_task_id);
    println!("Path: {}", task_path);

    let task_meta = match state["tasks"].get(current_task_id) {
        Some(m) => m,
        None => {
            println!(
                "Warning: Task metadata for '{}' not found in state.json.",
                current_task_id
            );
            println!("Suggest running `cflow state repair`.");
            return;
        }
    };

    println!("Title: {}", option_to_js_string(task_meta.get("title")));
    println!("Phase: {}", option_to_js_string(task_meta.get("phase")));
    println!("Status: {}", option_to_js_string(task_meta.get("status")));
    println!(
        "Next Action: {}",
        option_to_js_string(task_meta.get("next_action"))
    );

    println!();
    println!("Artifacts:");
    let files = ["REQUEST.md", "PLAN.md", "CODING.md", "VERIFY.md", "SHIP.md"];
    let mut disagree = false;

    for file in files {
        let file_path = format!("{}/{}", task_path, file);
        let fs_exists = Path::new(&file_path).exists();
        let state_exists = task_meta["artifact_presence"]
            .get(file)
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if fs_exists != state_exists {
            disagree = true;
        }

        let status_str = if fs_exists { "exists" } else { "missing" };
        println!("- {}: {}", file, status_str);
    }

    let verify_path = format!("{}/VERIFY.md", task_path);
    if Path::new(&verify_path).exists() {
        let content = fs::read_to_string(&verify_path).unwrap_or_default();
        let fs_status = first_non_empty_section_line(&content, "Status")
            .unwrap_or_else(|| "unknown".to_string());
        let fs_findings = verify_findings_count(&content);

        let state_status = task_meta
            .get("verify_status")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        let state_findings = task_meta
            .get("findings_count")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize;

        if fs_status != state_status || fs_findings != state_findings {
            disagree = true;
        }

        println!();
        println!("Verify (from filesystem):");
        println!("- Status: {}", fs_status);
        println!("- Findings: {}", fs_findings);
    } else {
        let state_has_verify = task_meta["artifact_presence"]
            .get("VERIFY.md")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        if state_has_verify {
            disagree = true;
        }
    }

    if task_meta["committed"].as_bool().unwrap_or(false) {
        println!();
        println!("Ship:");
        println!("- Committed: true");
        if let Some(sha) = task_meta["commit_sha"].as_str() {
            println!("- Commit SHA: {}", sha);
        }
    }

    let next = determine_next_action(&task_path);
    println!();
    println!("Next (determined from filesystem):");
    println!("- {}", next.command);

    if disagree {
        println!();
        println!(
            "Warning: State file and filesystem disagree. Suggest running `cflow state repair`."
        );
    }
}

fn command_tasks() -> CflowResult<()> {
    let state = load_state();
    let current_task_id = state["current_task_id"].as_str().unwrap_or("");
    let tasks = match state["tasks"].as_object() {
        Some(t) => t,
        None => {
            println!("No tasks found in state.");
            return Ok(());
        }
    };

    if tasks.is_empty() {
        println!("No tasks found.");
        return Ok(());
    }

    println!("Known tasks:");
    for (task_id, meta) in tasks {
        let is_current = if task_id == current_task_id {
            "* "
        } else {
            "  "
        };
        let title = meta.get("title").and_then(|v| v.as_str()).unwrap_or("");
        let phase = meta.get("phase").and_then(|v| v.as_str()).unwrap_or("");
        let status = meta.get("status").and_then(|v| v.as_str()).unwrap_or("");
        let updated_at = meta
            .get("updated_at")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        println!(
            "{}{} - {} (phase: {}, status: {}, updated: {})",
            is_current, task_id, title, phase, status, updated_at
        );
    }
    Ok(())
}

fn command_switch(args: &[String]) -> CflowResult<()> {
    let task_id = args.first().cloned().unwrap_or_default();
    if task_id.is_empty() {
        return Err("Missing task ID. Usage: cflow switch <task-id>".to_string());
    }

    let mut state = load_state();
    let task_exists = state["tasks"].get(&task_id).is_some();
    if !task_exists {
        let task_folder = format!(".coding/tasks/{}", task_id);
        if Path::new(&task_folder).exists() {
            println!("Task not in state.json but folder exists. Initializing state...");
            update_task_state(&task_id, None, None)?;
            state = load_state();
        } else {
            return Err(format!(
                "Task '{}' not found in state or filesystem.",
                task_id
            ));
        }
    }

    state["current_task_id"] = Value::String(task_id.clone());
    save_state(&state)?;

    let _ = write_text(".coding/current", &format!("tasks/{}", task_id));

    println!("Switched to task: {}", task_id);
    Ok(())
}

fn command_state_repair() -> CflowResult<()> {
    println!("Repairing state.json from .coding/tasks/* ...");

    let tasks_dir = ".coding/tasks";
    if !Path::new(tasks_dir).exists() {
        println!("No tasks folder found at {}", tasks_dir);
        return Ok(());
    }

    let entries = fs::read_dir(tasks_dir).map_err(|e| e.to_string())?;
    let mut task_ids = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    task_ids.push(name.to_string());
                }
            }
        }
    }

    task_ids.sort();

    for task_id in &task_ids {
        println!("- Syncing state for: {}", task_id);
        let title = if task_id.len() > 16 {
            task_id[16..].to_string()
        } else {
            task_id.clone()
        };
        update_task_state(task_id, None, Some(&title))?;
    }

    let mut state = load_state();
    let current_task_id = state["current_task_id"].as_str().map(String::from);

    if current_task_id.is_none() || !task_ids.contains(current_task_id.as_ref().unwrap()) {
        let mut fallback_id = None;
        if Path::new(".coding/current").exists() {
            if let Ok(current_content) = fs::read_to_string(".coding/current") {
                let current_content = current_content.trim();
                let id = current_content
                    .strip_prefix("tasks/")
                    .unwrap_or(current_content);
                if task_ids.contains(&id.to_string()) {
                    fallback_id = Some(id.to_string());
                }
            }
        }

        if fallback_id.is_none() {
            fallback_id = task_ids.last().cloned();
        }

        if let Some(id) = fallback_id {
            println!("Setting current_task_id to: {}", id);
            state["current_task_id"] = Value::String(id);
            save_state(&state)?;
        }
    }

    println!("State repair complete.");
    Ok(())
}

fn get_verify_status(task_path: &str) -> Option<String> {
    let verify_path = format!("{}/VERIFY.md", task_path);
    let content = fs::read_to_string(&verify_path).ok()?;
    first_non_empty_section_line(&content, "Status")
}

fn get_verify_findings_count(task_path: &str) -> Option<usize> {
    let verify_path = format!("{}/VERIFY.md", task_path);
    let content = fs::read_to_string(&verify_path).ok()?;
    Some(verify_findings_count(&content))
}

struct NextAction {
    command: String,
    reason: String,
}

fn determine_next_action(task_path: &str) -> NextAction {
    let state = load_state();
    
    if let Some(packet_id) = state["current_packet_id"].as_str() {
        if !packet_id.is_empty() {
            let p_path = format!(".coding/packets/{}", packet_id);
            let req = Path::new(&format!("{}/REQUEST.md", p_path)).exists();
            let intake = Path::new(&format!("{}/INTAKE.md", p_path)).exists();
            
            if !req {
                return NextAction {
                    command: "packet intake or request creation".to_string(),
                    reason: "REQUEST.md is missing".to_string(),
                };
            }
            if !intake {
                return NextAction {
                    command: "cflow packet intake --packet current".to_string(),
                    reason: "INTAKE.md is missing".to_string(),
                };
            }
            
            let packet_meta = &state["packets"][packet_id];
            let lane = option_to_js_string(packet_meta.get("lane"));
            let split_required = packet_meta.get("split_required").and_then(|v| v.as_bool()).unwrap_or(false);
            
            if lane == "tiny" {
                let plan = Path::new(&format!("{}/PLAN.md", p_path)).exists();
                let coding = Path::new(&format!("{}/CODING.md", p_path)).exists();
                let verify = Path::new(&format!("{}/VERIFY.md", p_path)).exists();
                let ship = Path::new(&format!("{}/SHIP.md", p_path)).exists();
                
                if !plan {
                    return NextAction {
                        command: "cflow agent plan --packet current".to_string(),
                        reason: "PLAN.md is missing in tiny lane".to_string(),
                    };
                }
                if !coding {
                    return NextAction {
                        command: "cflow agent coding --packet current".to_string(),
                        reason: "CODING.md is missing in tiny lane".to_string(),
                    };
                }
                if !verify {
                    return NextAction {
                        command: "cflow verify --task current".to_string(),
                        reason: "VERIFY.md is missing in tiny lane".to_string(),
                    };
                }
                let status = get_verify_status(&p_path).unwrap_or_default();
                if status == "failed" || status == "partial" {
                    return NextAction {
                        command: "cflow agent coding --task current --fix".to_string(),
                        reason: format!("VERIFY.md status is {}", status),
                    };
                }
                let findings_count = get_verify_findings_count(&p_path).unwrap_or(0);
                if findings_count > 0 {
                    return NextAction {
                        command: "cflow agent coding --task current --fix".to_string(),
                        reason: format!("VERIFY.md has {} findings", findings_count),
                    };
                }
                if !ship {
                    return NextAction {
                        command: "cflow ship --task current --dry-run".to_string(),
                        reason: "VERIFY.md passed, SHIP.md is missing".to_string(),
                    };
                }
                return NextAction {
                    command: "done or commit pending".to_string(),
                    reason: "SHIP.md exists".to_string(),
                };
            }
            
            if split_required {
                let packet_brief = Path::new(&format!("{}/PACKET.md", p_path)).exists();
                let stories_idx = Path::new(&format!("{}/STORIES.md", p_path)).exists();
                
                if !packet_brief {
                    return NextAction {
                        command: "cflow packet brief --packet current".to_string(),
                        reason: "PACKET.md is missing".to_string(),
                    };
                }
                if !stories_idx {
                    return NextAction {
                        command: "cflow packet split --packet current".to_string(),
                        reason: "STORIES.md is missing".to_string(),
                    };
                }
            }
            
            let current_story_id = state["current_story_id"].as_str().unwrap_or("");
            if current_story_id.is_empty() {
                return NextAction {
                    command: "cflow story list / cflow story switch <story-id>".to_string(),
                    reason: "current story is missing".to_string(),
                };
            }
            
            let story_path = format!("{}/stories/{}", p_path, current_story_id);
            let story_plan = Path::new(&format!("{}/PLAN.md", story_path)).exists();
            let story_coding = Path::new(&format!("{}/CODING.md", story_path)).exists();
            let story_verify = Path::new(&format!("{}/VERIFY.md", story_path)).exists();
            
            if !story_plan {
                return NextAction {
                    command: "cflow story agent plan --story current".to_string(),
                    reason: "current story PLAN.md is missing".to_string(),
                };
            }
            if !story_coding {
                return NextAction {
                    command: "cflow story agent coding --story current".to_string(),
                    reason: "current story CODING.md is missing".to_string(),
                };
            }
            if !story_verify {
                return NextAction {
                    command: "cflow story verify --story current".to_string(),
                    reason: "current story VERIFY.md is missing".to_string(),
                };
            }
            
            let story_verify_status = get_verify_status(&story_path).unwrap_or_default();
            if story_verify_status == "failed" || story_verify_status == "partial" {
                return NextAction {
                    command: "cflow story agent coding --story current --fix".to_string(),
                    reason: format!("story VERIFY.md status is {}", story_verify_status),
                };
            }
            let story_findings = get_verify_findings_count(&story_path).unwrap_or(0);
            if story_findings > 0 {
                return NextAction {
                    command: "cflow story agent coding --story current --fix".to_string(),
                    reason: format!("story VERIFY.md has {} findings", story_findings),
                };
            }
            
            let mut all_stories_done = true;
            if let Some(stories_map) = packet_meta.get("stories").and_then(|s| s.as_object()) {
                for (id, _) in stories_map {
                    let s_path = format!("{}/stories/{}", p_path, id);
                    let s_verify = Path::new(&format!("{}/VERIFY.md", s_path)).exists();
                    let s_verify_status = get_verify_status(&s_path).unwrap_or_default();
                    let s_findings = get_verify_findings_count(&s_path).unwrap_or(0);
                    let s_ship = Path::new(&format!("{}/SHIP.md", s_path)).exists();
                    
                    let is_done = s_ship || (s_verify && s_verify_status == "passed" && s_findings == 0);
                    if !is_done {
                        all_stories_done = false;
                        break;
                    }
                }
            } else {
                all_stories_done = false;
            }
            
            if !all_stories_done {
                return NextAction {
                    command: "cflow story list / cflow story switch <story-id>".to_string(),
                    reason: "current story is done but other stories are pending".to_string(),
                };
            }
            
            let packet_verify = Path::new(&format!("{}/PACKET_VERIFY.md", p_path)).exists();
            if !packet_verify {
                return NextAction {
                    command: "cflow packet verify --packet current".to_string(),
                    reason: "all stories verified/shipped".to_string(),
                };
            }
            
            let p_verify_content = fs::read_to_string(&format!("{}/PACKET_VERIFY.md", p_path)).unwrap_or_default();
            let p_status = first_non_empty_section_line(&p_verify_content, "Status").unwrap_or_default();
            
            if p_status == "passed" {
                let packet_ship = Path::new(&format!("{}/PACKET_SHIP.md", p_path)).exists();
                if !packet_ship {
                    return NextAction {
                        command: "cflow packet ship --packet current --dry-run".to_string(),
                        reason: "PACKET_VERIFY.md passed and PACKET_SHIP.md missing".to_string(),
                    };
                }
                return NextAction {
                    command: "done or commit pending".to_string(),
                    reason: "PACKET_SHIP.md exists".to_string(),
                };
            } else {
                return NextAction {
                    command: "cflow packet verify --packet current".to_string(),
                    reason: format!("PACKET_VERIFY.md status is {}", p_status),
                };
            }
        }
    }

    let req = Path::new(&format!("{}/REQUEST.md", task_path)).exists();
    let plan = Path::new(&format!("{}/PLAN.md", task_path)).exists();
    let coding = Path::new(&format!("{}/CODING.md", task_path)).exists();
    let verify = Path::new(&format!("{}/VERIFY.md", task_path)).exists();
    let ship = Path::new(&format!("{}/SHIP.md", task_path)).exists();

    if !req {
        NextAction {
            command: "cflow request --task current".to_string(),
            reason: "REQUEST.md is missing".to_string(),
        }
    } else if !plan {
        NextAction {
            command: "cflow agent plan --task current".to_string(),
            reason: "PLAN.md is missing".to_string(),
        }
    } else if !coding {
        NextAction {
            command: "cflow agent coding --task current".to_string(),
            reason: "CODING.md is missing".to_string(),
        }
    } else if !verify {
        NextAction {
            command: "cflow verify --task current".to_string(),
            reason: "VERIFY.md is missing".to_string(),
        }
    } else {
        let status = get_verify_status(task_path).unwrap_or_default();
        match status.as_str() {
            "failed" | "partial" => NextAction {
                command: "cflow agent coding --task current --fix".to_string(),
                reason: format!("VERIFY.md status is {}", status),
            },
            "skipped" => NextAction {
                command: "cflow verify --task current".to_string(),
                reason: "VERIFY.md status is skipped".to_string(),
            },
            "passed" => {
                let findings_count = get_verify_findings_count(task_path).unwrap_or(0);
                if findings_count > 0 {
                    NextAction {
                        command: "cflow agent coding --task current --fix".to_string(),
                        reason: format!("VERIFY.md has {} findings", findings_count),
                    }
                } else if !ship {
                    NextAction {
                        command: "cflow ship --task current --dry-run".to_string(),
                        reason: "VERIFY.md exists and status is passed and SHIP.md missing"
                            .to_string(),
                    }
                } else {
                    NextAction {
                        command: "done or commit pending".to_string(),
                        reason: "SHIP.md exists".to_string(),
                    }
                }
            }
            _ => NextAction {
                command: "cflow verify --task current".to_string(),
                reason: format!("VERIFY.md status is {}", status),
            },
        }
    }
}

fn command_next(args: &[String]) -> CflowResult<()> {
    let task_path = resolve_task(args)?;
    let next = determine_next_action(&task_path);
    println!("Next: {}", next.command);
    println!("Reason: {}", next.reason);
    Ok(())
}

fn command_run(args: &[String]) -> CflowResult<()> {
    let task_path = resolve_task(args)?;

    loop {
        let next = determine_next_action(&task_path);

        match next.command.as_str() {
            "cflow agent plan --task current" => {
                println!("Next: {}", next.command);
                println!("Reason: {}", next.reason);
                println!("--- Running: {} ---", next.command);
                let current_args = vec![
                    "plan".to_string(),
                    "--task".to_string(),
                    "current".to_string(),
                ];
                command_agent(&current_args)?;
            }
            "cflow agent coding --task current" => {
                println!("Next: {}", next.command);
                println!("Reason: {}", next.reason);
                println!("--- Running: {} ---", next.command);
                let current_args = vec![
                    "coding".to_string(),
                    "--task".to_string(),
                    "current".to_string(),
                ];
                command_agent(&current_args)?;
            }
            "cflow agent coding --task current --fix" => {
                println!("Next: {}", next.command);
                println!("Reason: {}", next.reason);
                println!("--- Running: {} ---", next.command);
                let current_args = vec![
                    "coding".to_string(),
                    "--task".to_string(),
                    "current".to_string(),
                    "--fix".to_string(),
                ];
                command_agent(&current_args)?;
                break;
            }
            "cflow ship --task current --dry-run" => {
                println!("Next: {}", next.command);
                println!("Reason: {}", next.reason);
                println!("--- Running: {} ---", next.command);
                let mut cmd = Command::new(env::current_exe().unwrap_or_else(|_| "cflow".into()));
                cmd.args(["ship", "--dry-run", "--task", "current"]);
                cmd.status().map_err(|e| e.to_string())?;
                break;
            }
            _ => {
                println!("Next: {}", next.command);
                println!("Reason: {}", next.reason);
                println!("Stopping because human input is needed or task is complete.");
                break;
            }
        }
    }

    Ok(())
}

fn print_usage() {
    println!(
        "Usage:
  cflow new \"<task-name>\"
  cflow request [--task current] [--input file]
  cflow plan    [--task current] [--input file]
  cflow coding  [--task current] [--input file]
  cflow agent plan   [--task current] [--provider name] [--verbose]
  cflow agent coding [--task current] [--provider name] [--fix] [--verbose]
  cflow agent providers
  cflow agent doctor [--provider name]
  cflow verify  [--task current] [--input file]
  cflow ship    [--task current] [--input file] [--dry-run|--commit]
  cflow problem add|list|open|resolved|cancelled|show|resolve|cancel|update
  cflow status
  cflow tasks
  cflow switch <task-id>
  cflow state repair
  cflow next    [--task current]
  cflow run     [--task current]
  cflow packet new|intake|brief|split|status|verify|ship
  cflow story list|switch|status|plan|coding|verify|ship
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
        Some("coding") => command_coding(&raw_args),
        Some("agent") => command_agent(&raw_args),
        Some("verify") => command_verify(&raw_args),
        Some("ship") => command_ship(&raw_args),
        Some("problem") => command_problem(&raw_args),
        Some("packet") => command_packet(&raw_args),
        Some("story") => command_story(&raw_args),
        Some("status") => {
            command_status();
            Ok(())
        }
        Some("tasks") => command_tasks(),
        Some("switch") => command_switch(&raw_args),
        Some("state") => {
            if raw_args.first().map(|s| s.as_str()) == Some("repair") {
                command_state_repair()
            } else {
                Err("Unknown state command. Did you mean `cflow state repair`?".to_string())
            }
        }
        Some("next") => command_next(&raw_args),
        Some("run") => command_run(&raw_args),
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

    fn test_task_dir(name: &str) -> String {
        let path = env::temp_dir().join(format!(
            "cflow-test-{}-{}",
            name,
            chrono::Local::now()
                .timestamp_nanos_opt()
                .unwrap_or_default()
        ));
        fs::create_dir_all(&path).expect("test dir should be created");
        path.to_string_lossy().into_owned()
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

    #[test]
    fn next_failed_verify_routes_to_agent_coding_fix() {
        let task_path = test_task_dir("next-failed-verify");
        write_text(&format!("{}/REQUEST.md", task_path), "# Request\n").unwrap();
        write_text(&format!("{}/PLAN.md", task_path), "# Plan\n").unwrap();
        write_text(&format!("{}/CODING.md", task_path), "# Coding\n").unwrap();
        write_text(
            &format!("{}/VERIFY.md", task_path),
            "# Verify\n\n## Status\n\nfailed\n\n## Findings\n\n- [F001] copy_mismatch\n  - Severity: high\n",
        )
        .unwrap();

        let next = determine_next_action(&task_path);

        assert_eq!(next.command, "cflow agent coding --task current --fix");
    }

    #[test]
    fn ship_gate_rejects_passed_verify_with_findings() {
        let task_path = test_task_dir("ship-findings");
        write_text(
            &format!("{}/VERIFY.md", task_path),
            "# Verify\n\n## Status\n\npassed\n\n## Findings\n\n- [F001] copy_mismatch\n  - Severity: high\n",
        )
        .unwrap();

        assert_eq!(
            ensure_verify_passed_for_ship(&task_path)
                .expect_err("ship should reject findings in VERIFY.md"),
            "Ship rejected: VERIFY.md has findings"
        );
    }

    #[test]
    fn parses_commit_message_from_ship_markdown_fence() {
        let rendered = render_ship(&parse_json(include_str!("../examples/ship.json")));

        assert_eq!(
            parse_ship_commit_message(&rendered).as_deref(),
            Some("feat(docs): add copy button to code blocks")
        );
    }

    #[test]
    fn counts_structured_verify_findings() {
        let verify = "# Verify\n\n## Status\n\nfailed\n\n## Findings\n\n- [F001] copy_mismatch\n  - Severity: high\n";

        assert_eq!(verify_findings_count(verify), 1);
    }

    #[test]
    fn verify_none_findings_section_has_no_findings() {
        let verify = "# Verify\n\n## Status\n\npassed\n\n## Findings\n\n- _None_\n";

        assert_eq!(verify_findings_count(verify), 0);
    }

    #[test]
    fn cli_provider_overrides_config_default() {
        let config = r#"default_provider = "claude""#
            .parse::<toml::Value>()
            .expect("config should parse");
        let args = vec!["--provider".to_string(), "gemini".to_string()];

        assert_eq!(
            resolve_agent_provider(&args, Some(&config)).expect("provider should resolve"),
            "gemini"
        );
    }

    #[test]
    fn reads_custom_provider_phase_config() {
        let config = r#"
[providers.custom.plan]
cmd = "my-agent"
args = ["--json"]
prompt_mode = "stdin"
"#
        .parse::<toml::Value>()
        .expect("config should parse");

        let command = config_agent_command(Some(&config), "custom", AgentPhase::Plan)
            .expect("config lookup should succeed")
            .expect("command should exist");

        assert_eq!(command.cmd, "my-agent");
        assert_eq!(command.args, vec!["--json"]);
        assert_eq!(command.prompt_mode, PromptMode::Stdin);
    }

    #[test]
    fn extracts_json_from_claude_style_wrapper_text() {
        let stdout = r#"{
  "type": "message",
  "content": [
    {
      "type": "text",
      "text": "{\"mode\":\"initial\",\"status\":\"ready_for_verify\",\"summary\":[],\"fixed_findings\":[],\"changed_files\":[],\"notes\":[],\"next\":\"verify\"}"
    }
  ]
}"#;

        let payload = agent_json_payload(stdout).expect("payload should be extracted");
        let data: Value = serde_json::from_str(&payload).expect("payload should parse");

        assert_eq!(data["mode"], "initial");
    }

    #[test]
    fn extracts_last_json_object_from_stdout() {
        let stdout = r#"progress {"ignored":true}
{"mode":"initial","status":"ready_for_verify","summary":[],"fixed_findings":[],"changed_files":[],"notes":[],"next":"verify"}"#;

        let payload = agent_json_payload(stdout).expect("payload should be extracted");
        let data: Value = serde_json::from_str(&payload).expect("payload should parse");

        assert_eq!(data["mode"], "initial");
    }

    #[test]
    fn built_in_codex_coding_uses_schema_and_arg_prompt_mode() {
        let command = builtin_agent_command("codex", AgentPhase::Coding)
            .expect("builtin should resolve")
            .expect("command should exist");

        assert_eq!(command.cmd, "codex");
        assert_eq!(command.prompt_mode, PromptMode::Arg);
        assert!(command
            .args
            .contains(&"schemas/coding.schema.json".to_string()));
        assert_eq!(command.args.last().map(String::as_str), Some("--"));
        assert!(display_resolved_agent_command(&command).ends_with("-- \"<PROMPT>\""));
    }

    #[test]
    fn built_in_codex_plan_uses_separator_before_prompt() {
        let command = builtin_agent_command("codex", AgentPhase::Plan)
            .expect("builtin should resolve")
            .expect("command should exist");

        assert_eq!(command.cmd, "codex");
        assert_eq!(command.prompt_mode, PromptMode::Arg);
        assert!(command
            .args
            .contains(&"schemas/plan.schema.json".to_string()));
        assert_eq!(command.args.last().map(String::as_str), Some("--"));
        assert!(display_resolved_agent_command(&command).ends_with("-- \"<PROMPT>\""));
    }

    #[test]
    fn built_in_antigravity_uses_agy_prompt_arg_for_plan_and_coding() {
        for phase in [AgentPhase::Plan, AgentPhase::Coding] {
            let command = builtin_agent_command("antigravity", phase)
                .expect("builtin should resolve")
                .expect("command should exist");

            assert_eq!(command.cmd, "agy");
            assert_eq!(command.args, vec!["--prompt"]);
            assert_eq!(command.prompt_mode, PromptMode::Arg);
            assert_eq!(
                display_resolved_agent_command(&command),
                "agy --prompt \"<PROMPT>\""
            );
        }
    }
}
