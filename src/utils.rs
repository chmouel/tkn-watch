/// parse datetime as human duration.
pub fn parse_dt_as_duration(dt: &str) -> String {
    let start_time = chrono::NaiveDateTime::parse_from_str(dt, "%Y-%m-%dT%H:%M:%SZ")
        .expect("Failed to parse start_time");
    // format duration since now
    let duration = start_time - chrono::Utc::now().naive_local();
    chrono_humanize::HumanTime::from(duration).to_string()
}

// color status string
pub fn colorit(color: &str, string: &str) -> String {
    format!(
        "\x1b[{}m{}\x1b[0m",
        match color {
            "yellow" => String::from("33"),
            "blue" => String::from("34"),
            "green" => String::from("32"),
            "red" => String::from("31"),
            "cyan" => String::from("36"),
            "magenta" => String::from("35"),
            "underline" => String::from("4"),
            "italic" => String::from("3"),
            "bold" => String::from("1"),
            _ => String::new(),
        },
        string
    )
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
