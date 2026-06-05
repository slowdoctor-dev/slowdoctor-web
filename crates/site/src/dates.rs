//! Shared calendar-date and date-prefixed filename rules.

use time::{Date, Month};

/// Parse `YYYY-MM-DD`, rejecting malformed and invalid calendar dates.
pub fn parse_date_only(date: &str, context: &str) -> Result<Date, String> {
    let parts: Vec<&str> = date.split('-').collect();
    let ok_shape = parts.len() == 3
        && parts[0].len() == 4
        && parts[1].len() == 2
        && parts[2].len() == 2
        && parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()));
    if !ok_shape {
        return Err(format!(
            "Invalid date in {context}: expected YYYY-MM-DD, received \"{date}\""
        ));
    }
    let year: i32 = parts[0]
        .parse()
        .map_err(|_| format!("Invalid date in {context}"))?;
    let month_n: u8 = parts[1]
        .parse()
        .map_err(|_| format!("Invalid date in {context}"))?;
    let day: u8 = parts[2]
        .parse()
        .map_err(|_| format!("Invalid date in {context}"))?;
    let month = Month::try_from(month_n)
        .map_err(|_| format!("Invalid calendar date in {context}: \"{date}\""))?;
    Date::from_calendar_date(year, month, day)
        .map_err(|_| format!("Invalid calendar date in {context}: \"{date}\""))
}

pub fn is_date_only(date: &str) -> bool {
    parse_date_only(date, "date").is_ok()
}

pub fn has_date_prefix(stem: &str) -> bool {
    let bytes = stem.as_bytes();
    bytes.len() > 11
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[10] == b'-'
        && bytes[..4].iter().all(|b| b.is_ascii_digit())
        && bytes[5..7].iter().all(|b| b.is_ascii_digit())
        && bytes[8..10].iter().all(|b| b.is_ascii_digit())
}

pub fn strip_date_prefix(stem: &str) -> String {
    if has_date_prefix(stem) {
        stem[11..].to_string()
    } else {
        stem.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_only_valid_iso_calendar_dates() {
        assert_eq!(parse_date_only("2024-02-29", "post.md").unwrap().day(), 29);
        assert!(parse_date_only("2023-02-29", "post.md").is_err());
        assert!(parse_date_only("2024-2-09", "post.md").is_err());
        assert!(parse_date_only("not-a-date", "post.md").is_err());
    }

    #[test]
    fn strips_only_the_filename_date_prefix_shape() {
        assert_eq!(strip_date_prefix("2026-04-07-hello-world"), "hello-world");
        assert_eq!(strip_date_prefix("hello-world"), "hello-world");
        assert_eq!(
            strip_date_prefix("2026-4-07-hello-world"),
            "2026-4-07-hello-world"
        );
    }
}
