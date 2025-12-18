use std::time::Duration;

pub fn format_duration(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

pub fn format_mem_size(bytes: usize) -> String {
    const KB: usize = 1024;
    const MB: usize = KB * 1024;
    const GB: usize = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}
