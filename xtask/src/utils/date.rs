/// Convert a calendar date to Julian Day Number.
#[allow(clippy::cast_possible_truncation)]
pub const fn date_to_julian_day(year: i32, month: u32, day: u32) -> i32 {
    let a = (14 - month as i32) / 12;
    let y = year + 4800 - a;
    let m = month as i32 + 12 * a - 3;
    (day as i32) + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045
}

/// Parse ISO 8601 date string (YYYY-MM-DD) to Julian Day Number.
pub fn parse_date(s: &str) -> i32 {
    let parts: Vec<&str> = s.split('-').collect();
    assert_eq!(parts.len(), 3, "Invalid date format: {s}");
    let year: i32 = parts[0].parse().unwrap();
    let month: u32 = parts[1].parse().unwrap();
    let day: u32 = parts[2].parse().unwrap();
    date_to_julian_day(year, month, day)
}
