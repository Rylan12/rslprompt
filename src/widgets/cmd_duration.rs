use crate::{context::Context, formatting::Color, widgets::Widget};

/// Only show the duration for commands that take at least this long.
const MIN_DURATION_MS: u64 = 2_000;

/// Display how long the last command took to run, if it was slow enough to be interesting.
pub struct CmdDuration;

impl Widget for CmdDuration {
    fn content(&self, context: &Context) -> Option<String> {
        context
            .cmd_duration_ms()
            .filter(|&ms| ms >= MIN_DURATION_MS)
            .map(format_duration)
    }

    fn color(&self, _context: &Context) -> Color {
        Color::Yellow
    }
}

/// Format a duration in milliseconds as a compact human-readable string (e.g. `1h 2m 3s`).
fn format_duration(ms: u64) -> String {
    let total_secs = ms / 1000;
    let days = total_secs / 86_400;
    let hours = (total_secs % 86_400) / 3_600;
    let mins = (total_secs % 3_600) / 60;
    let secs = total_secs % 60;

    let parts: Vec<String> = [(days, "d"), (hours, "h"), (mins, "m"), (secs, "s")]
        .into_iter()
        .skip_while(|&(value, _)| value == 0)
        .filter(|&(value, _)| value != 0)
        .map(|(value, unit)| format!("{value}{unit}"))
        .collect();

    if parts.is_empty() {
        "0s".to_string()
    } else {
        parts.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::format_duration;

    #[test]
    fn formats_durations() {
        assert_eq!(format_duration(0), "0s");
        assert_eq!(format_duration(2_500), "2s");
        assert_eq!(format_duration(59_999), "59s");
        assert_eq!(format_duration(60_000), "1m");
        assert_eq!(format_duration(65_000), "1m 5s");
        assert_eq!(format_duration(3_600_000), "1h");
        assert_eq!(format_duration(3_723_000), "1h 2m 3s");
        assert_eq!(format_duration(90_061_000), "1d 1h 1m 1s");
    }
}
