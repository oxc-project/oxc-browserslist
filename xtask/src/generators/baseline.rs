use std::collections::{BTreeMap, BTreeSet, HashMap};

use anyhow::{Context, Result, bail, ensure};

use crate::{data::encode_browser_name, utils::save_bin_compressed};

const FORMAT_VERSION: u8 = 1;

/// Canonical mapping from `baseline-browser-mapping` to Can I Use names, ported from
/// Browserslist's `bbmTransform`.
fn caniuse_name(name: &str) -> Result<Option<&'static str>> {
    Ok(match name {
        "chrome" => Some("chrome"),
        "chrome_android" => Some("and_chr"),
        "edge" => Some("edge"),
        "firefox" => Some("firefox"),
        "firefox_android" => Some("and_ff"),
        "safari" => Some("safari"),
        "safari_ios" => Some("ios_saf"),
        "webview_android" => Some("android"),
        "samsunginternet_android" => Some("samsung"),
        "opera_android" => Some("op_mob"),
        "opera" => Some("opera"),
        "qq_android" => Some("and_qq"),
        "uc_android" => Some("and_uc"),
        "kai_os" => Some("kaios"),
        // Browserslist's mapping intentionally drops these browsers.
        "ya_android" | "facebook_android" | "instagram_android" => None,
        _ => bail!("unclassified Baseline browser: {name}"),
    })
}

fn date_key(date: &str) -> Result<u32> {
    let mut parts = date.split('-');
    let year: u32 = parts.next().context("missing Baseline year")?.parse()?;
    let month: u32 = parts.next().context("missing Baseline month")?.parse()?;
    let day: u32 = parts.next().context("missing Baseline day")?.parse()?;
    ensure!(parts.next().is_none(), "invalid Baseline date: {date}");
    Ok(year * 10_000 + month * 100 + day)
}

pub fn build_baseline() -> Result<()> {
    let timeline = crate::data::baseline::load()?;
    let expected_browsers = [
        "chrome",
        "chrome_android",
        "edge",
        "firefox",
        "firefox_android",
        "safari",
        "safari_ios",
        "webview_android",
        "samsunginternet_android",
        "opera_android",
        "opera",
        "qq_android",
        "uc_android",
        "kai_os",
    ]
    .map(|name| encode_browser_name(caniuse_name(name).unwrap().unwrap()))
    .into_iter()
    .collect::<BTreeSet<_>>();

    let mut events: Vec<(u32, Vec<(u8, String)>)> = Vec::with_capacity(timeline.len());
    for event in &timeline {
        let date = date_key(&event.date)?;
        ensure!(events.last().is_none_or(|(last, _)| *last < date), "timeline is not sorted");

        let mut snapshot = BTreeMap::new();
        for entry in &event.browsers {
            if let Some(name) = caniuse_name(&entry.browser)? {
                let id = encode_browser_name(name);
                ensure!(
                    snapshot.insert(id, entry.version.clone()).is_none(),
                    "duplicate Baseline browser {} on {}",
                    entry.browser,
                    event.date
                );
            }
        }
        ensure!(
            snapshot.keys().copied().collect::<BTreeSet<_>>() == expected_browsers,
            "incomplete Baseline snapshot on {}",
            event.date
        );
        let snapshot = snapshot.into_iter().collect::<Vec<_>>();

        // Dropping browsers unsupported by Can I Use can make adjacent snapshots identical.
        if events.last().is_none_or(|(_, last)| *last != snapshot) {
            events.push((date, snapshot));
        }
    }

    let browser_ids = expected_browsers.into_iter().collect::<Vec<_>>();
    let versions = events
        .iter()
        .flat_map(|(_, snapshot)| snapshot.iter().map(|(_, version)| version.clone()))
        .collect::<BTreeSet<_>>();
    let mut pool = String::new();
    let mut version_ranges = HashMap::with_capacity(versions.len());
    for version in versions {
        let offset = u16::try_from(pool.len()).context("Baseline string pool overflow")?;
        let len = u8::try_from(version.len()).context("Baseline version string too long")?;
        pool.push_str(&version);
        version_ranges.insert(version, (offset, len));
    }
    let pool_len = u16::try_from(pool.len()).context("Baseline string pool overflow")?;
    let event_count = u16::try_from(events.len()).context("too many Baseline events")?;
    let browser_count = u8::try_from(browser_ids.len()).context("too many Baseline browsers")?;

    // Format: schema, browser count, event count, pool length, browser IDs, string pool, then
    // fixed-size events. Each event is a date followed by one `(pool offset, string length)` entry
    // per browser. Fixed-size records let the runtime binary-search without deserializing tables.
    let mut bytes = Vec::new();
    bytes.push(FORMAT_VERSION);
    bytes.push(browser_count);
    bytes.extend_from_slice(&event_count.to_le_bytes());
    bytes.extend_from_slice(&pool_len.to_le_bytes());
    bytes.extend_from_slice(&browser_ids);
    bytes.extend_from_slice(pool.as_bytes());
    for (date, snapshot) in events {
        bytes.extend_from_slice(&date.to_le_bytes());
        ensure!(
            snapshot.iter().map(|(id, _)| *id).eq(browser_ids.iter().copied()),
            "Baseline snapshot browser order changed"
        );
        for (_, version) in snapshot {
            let (offset, len) = version_ranges[&version];
            bytes.extend_from_slice(&offset.to_le_bytes());
            bytes.push(len);
        }
    }
    save_bin_compressed("baseline.bin", &bytes);
    Ok(())
}
