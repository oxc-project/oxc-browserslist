use crate::semver::Version;
use chrono::{NaiveDate, NaiveDateTime};
use once_cell::sync::Lazy;
pub static RELEASE_SCHEDULE: Lazy<Vec<(Version, NaiveDateTime, NaiveDateTime)>> = Lazy::new(|| {
    let date_format = "%Y-%m-%d";
    [
        (Version(0u32, 8u32, 0u32), "2012-06-25", "2014-07-31"),
        (Version(0u32, 10u32, 0u32), "2013-03-11", "2016-10-31"),
        (Version(0u32, 12u32, 0u32), "2015-02-06", "2016-12-31"),
        (Version(4u32, 0u32, 0u32), "2015-09-08", "2018-04-30"),
        (Version(5u32, 0u32, 0u32), "2015-10-29", "2016-06-30"),
        (Version(6u32, 0u32, 0u32), "2016-04-26", "2019-04-30"),
        (Version(7u32, 0u32, 0u32), "2016-10-25", "2017-06-30"),
        (Version(8u32, 0u32, 0u32), "2017-05-30", "2019-12-31"),
        (Version(9u32, 0u32, 0u32), "2017-10-01", "2018-06-30"),
        (Version(10u32, 0u32, 0u32), "2018-04-24", "2021-04-30"),
        (Version(11u32, 0u32, 0u32), "2018-10-23", "2019-06-01"),
        (Version(12u32, 0u32, 0u32), "2019-04-23", "2022-04-30"),
        (Version(13u32, 0u32, 0u32), "2019-10-22", "2020-06-01"),
        (Version(14u32, 0u32, 0u32), "2020-04-21", "2023-04-30"),
        (Version(15u32, 0u32, 0u32), "2020-10-20", "2021-06-01"),
        (Version(16u32, 0u32, 0u32), "2021-04-20", "2023-09-11"),
        (Version(17u32, 0u32, 0u32), "2021-10-19", "2022-06-01"),
        (Version(18u32, 0u32, 0u32), "2022-04-19", "2025-04-30"),
        (Version(19u32, 0u32, 0u32), "2022-10-18", "2023-06-01"),
        (Version(20u32, 0u32, 0u32), "2023-04-18", "2026-04-30"),
        (Version(21u32, 0u32, 0u32), "2023-10-17", "2024-06-01"),
        (Version(22u32, 0u32, 0u32), "2024-04-23", "2027-04-30"),
        (Version(23u32, 0u32, 0u32), "2024-10-15", "2025-06-01"),
        (Version(24u32, 0u32, 0u32), "2025-04-22", "2028-04-30"),
    ]
    .into_iter()
    .map(|(version, start, end)| {
        (
            version,
            NaiveDate::parse_from_str(start, date_format)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            NaiveDate::parse_from_str(end, date_format)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
    })
    .collect::<Vec<_>>()
});
