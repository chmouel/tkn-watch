/// parse datetime as human duration.
pub fn parse_dt_as_duration(dt: &str) -> String {
    let start_time = chrono::NaiveDateTime::parse_from_str(dt, "%Y-%m-%dT%H:%M:%SZ")
        .expect("Failed to parse start_time");
    // format duration since now
    let duration = start_time - chrono::Utc::now().naive_local();
    chrono_humanize::HumanTime::from(duration).to_string()
}

// color status string
pub fn colorit(status: &str, s: &str) -> String {
    let color = match status {
        "yellow" => "\x1b[33m".to_string(),
        "blue" => "\x1b[34m".to_string(),
        "green" => "\x1b[32m".to_string(),
        "red" => "\x1b[31m".to_string(),
        "cyan" => "\x1b[36m".to_string(),
        "magenta" => "\x1b[35m".to_string(),
        _ => "".to_string(),
    };
    format!("{}{}\x1b[0m", color, s)
}

pub fn get_running_char(status: &str) -> String {
    match status {
        "pending" => colorit("yellow", "*"),
        "running" => colorit("yellow", "*"),
        "succeeded" => colorit("green", "✓"),
        "completed" => colorit("green", "✓"),
        "cancelled" => colorit("red", "-"),
        "error" => colorit("red", "✕"),
        "failed" => colorit("red", "✕"),
        "couldntgettask" => colorit("red", "✕"),
        _ => format!("• {} ", status),
    }
}
