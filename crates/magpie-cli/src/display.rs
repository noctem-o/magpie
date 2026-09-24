//! Small formatting helpers for human-readable output.
//!
//! Timestamps are display only. The log is ordered, not clocked (ADR-0006), so
//! nothing here gives a recorded time any meaning beyond "when the writer said
//! it was written".

/// Render nanoseconds since the Unix epoch as a UTC timestamp.
pub(crate) fn utc(timestamp_nanos: u64) -> String {
    let seconds = timestamp_nanos / 1_000_000_000;
    let days = i64::try_from(seconds / 86_400).unwrap_or(i64::MAX);
    let second_of_day = seconds % 86_400;
    let (year, month, day) = civil_from_days(days);
    format!(
        "{year:04}-{month:02}-{day:02} {:02}:{:02}:{:02}Z",
        second_of_day / 3_600,
        (second_of_day % 3_600) / 60,
        second_of_day % 60
    )
}

/// Days since 1970-01-01 to a proleptic Gregorian date (Howard Hinnant's algorithm).
fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let z = days.saturating_add(719_468);
    let era = z.div_euclid(146_097);
    let day_of_era = z.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_index = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_index + 2) / 5 + 1;
    let month = if month_index < 10 {
        month_index + 3
    } else {
        month_index - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    (year, month, day)
}

/// The first line of `text`, cut to `max` characters, for one-line listings.
pub(crate) fn preview(text: &str, max: usize) -> String {
    let line = text.lines().next().unwrap_or("");
    let mut chars = line.chars();
    let cut: String = chars.by_ref().take(max).collect();
    if chars.next().is_some() || text.lines().nth(1).is_some() {
        format!("{cut}…")
    } else {
        cut
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utc_handles_the_epoch_and_a_leap_day() {
        assert_eq!(utc(0), "1970-01-01 00:00:00Z");
        assert_eq!(utc(951_782_400 * 1_000_000_000), "2000-02-29 00:00:00Z");
        assert_eq!(utc(1_758_710_531 * 1_000_000_000), "2025-09-24 10:42:11Z");
    }

    #[test]
    fn preview_marks_cut_text() {
        assert_eq!(preview("short", 10), "short");
        assert_eq!(preview("a longer line of text", 8), "a longer…");
        assert_eq!(preview("first\nsecond", 10), "first…");
    }
}
