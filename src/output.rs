use colored::Colorize;
use serde_json::Value;
use std::io::{self, Write};

fn format_value(v: &Value) -> String {
    match v {
        Value::Null => "-".to_string(),
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        _ => v.to_string(),
    }
}

fn colorize_status(val: &str) -> String {
    match val.to_lowercase().as_str() {
        "running" => val.green().to_string(),
        "stopped" => val.red().to_string(),
        "paused" | "suspended" => val.yellow().to_string(),
        "online" => val.green().to_string(),
        "offline" => val.red().to_string(),
        "ok" => val.green().to_string(),
        _ => val.to_string(),
    }
}

fn is_status_column(header: &str) -> bool {
    matches!(
        header.to_uppercase().as_str(),
        "STATUS" | "STATE" | "EXIT STATUS"
    )
}

pub fn print_list(data: &Value, json: bool, columns: &[(&str, &str)]) {
    if json {
        println!("{}", serde_json::to_string_pretty(data).unwrap_or_default());
        return;
    }

    let items = match data.as_array() {
        Some(arr) => arr,
        None => {
            println!("No data");
            return;
        }
    };

    if items.is_empty() {
        println!("No items found.");
        return;
    }

    let mut widths: Vec<usize> = columns.iter().map(|(_, h)| h.len()).collect();
    for item in items {
        for (i, (key, _)) in columns.iter().enumerate() {
            let val = format_value(&item[*key]);
            widths[i] = widths[i].max(val.len());
        }
    }

    let header: String = columns
        .iter()
        .zip(&widths)
        .map(|((_, h), w)| format!("{:<width$}", h.bold(), width = *w))
        .collect::<Vec<_>>()
        .join("  ");
    println!("{}", header);
    println!(
        "{}",
        widths
            .iter()
            .map(|w| "-".repeat(*w))
            .collect::<Vec<_>>()
            .join("  ")
    );

    for item in items {
        let row: String = columns
            .iter()
            .zip(&widths)
            .map(|((key, hdr), w)| {
                let val = format_value(&item[*key]);
                if is_status_column(hdr) {
                    format!("{:<width$}", colorize_status(&val), width = *w)
                } else {
                    format!("{:<width$}", val, width = *w)
                }
            })
            .collect::<Vec<_>>()
            .join("  ");
        println!("{}", row);
    }
}

pub fn print_item(data: &Value, json: bool, fields: &[(&str, &str)]) {
    if json {
        println!("{}", serde_json::to_string_pretty(data).unwrap_or_default());
        return;
    }

    let max_label = fields.iter().map(|(_, l)| l.len()).max().unwrap_or(0);
    for (key, label) in fields {
        let val = format_value(&data[*key]);
        let display_val = if is_status_column(label) {
            colorize_status(&val)
        } else {
            val
        };
        println!(
            "{:>width$} : {}",
            label.bold(),
            display_val,
            width = max_label
        );
    }
}

pub fn print_raw(data: &Value, json: bool) {
    if json {
        println!("{}", serde_json::to_string_pretty(data).unwrap_or_default());
        return;
    }

    if let Some(obj) = data.as_object() {
        let max_key = obj.keys().map(|k| k.len()).max().unwrap_or(0);
        let mut keys: Vec<&String> = obj.keys().collect();
        keys.sort();
        for key in keys {
            println!(
                "{:>width$} : {}",
                key.bold(),
                format_value(&obj[key]),
                width = max_key
            );
        }
    } else {
        println!("{}", serde_json::to_string_pretty(data).unwrap_or_default());
    }
}

pub fn confirm(message: &str) -> bool {
    eprint!("{} [y/N] ", message);
    io::stderr().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    input.trim().eq_ignore_ascii_case("y")
}
