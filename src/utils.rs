pub fn percentage(numerator: u64, denominator: u64) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64 * 100.0
    }
}

pub fn format_int(value: u64) -> String {
    let s = value.to_string();
    let mut result = String::new();

    for (i, ch) in s.chars().rev().enumerate() {
        if i != 0 && i % 3 == 0 {
            result.push('.');
        }
        result.push(ch);
    }

    result.chars().rev().collect()
}

pub fn format_pct(value: f64) -> String {
    format!("{:.4}%", value)
}

pub fn truncate(text: &str, max_len: usize) -> String {
    let len = text.chars().count();

    if len <= max_len {
        text.to_string()
    } else {
        let shortened: String = text.chars().take(max_len).collect();
        format!("{}...", shortened)
    }
}

pub fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}
