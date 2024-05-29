use ahash::AHashMap;
use chrono::{NaiveDate, NaiveDateTime};
use once_cell::sync::Lazy;
pub static RELEASE_SCHEDULE: Lazy<AHashMap<&'static str, (NaiveDateTime, NaiveDateTime)>> =
    Lazy::new(|| {
        let date_format = "%Y-%m-%d";
        let mut map = ahash::AHashMap::with_capacity(24usize);
        map.insert("0.8", ("2012-06-25", "2014-07-31"));
        map.insert("0.10", ("2013-03-11", "2016-10-31"));
        map.insert("0.12", ("2015-02-06", "2016-12-31"));
        map.insert("4", ("2015-09-08", "2018-04-30"));
        map.insert("5", ("2015-10-29", "2016-06-30"));
        map.insert("6", ("2016-04-26", "2019-04-30"));
        map.insert("7", ("2016-10-25", "2017-06-30"));
        map.insert("8", ("2017-05-30", "2019-12-31"));
        map.insert("9", ("2017-10-01", "2018-06-30"));
        map.insert("10", ("2018-04-24", "2021-04-30"));
        map.insert("11", ("2018-10-23", "2019-06-01"));
        map.insert("12", ("2019-04-23", "2022-04-30"));
        map.insert("13", ("2019-10-22", "2020-06-01"));
        map.insert("14", ("2020-04-21", "2023-04-30"));
        map.insert("15", ("2020-10-20", "2021-06-01"));
        map.insert("16", ("2021-04-20", "2023-09-11"));
        map.insert("17", ("2021-10-19", "2022-06-01"));
        map.insert("18", ("2022-04-19", "2025-04-30"));
        map.insert("19", ("2022-10-18", "2023-06-01"));
        map.insert("20", ("2023-04-18", "2026-04-30"));
        map.insert("21", ("2023-10-17", "2024-06-01"));
        map.insert("22", ("2024-04-23", "2027-04-30"));
        map.insert("23", ("2024-10-15", "2025-06-01"));
        map.insert("24", ("2025-04-22", "2028-04-30"));
        map.into_iter()
            .map(|(version, (start, end))| {
                (
                    version,
                    (
                        NaiveDate::parse_from_str(start, date_format)
                            .unwrap()
                            .and_hms_opt(0, 0, 0)
                            .unwrap(),
                        NaiveDate::parse_from_str(end, date_format)
                            .unwrap()
                            .and_hms_opt(0, 0, 0)
                            .unwrap(),
                    ),
                )
            })
            .collect()
    });
