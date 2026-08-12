use std::collections::{BTreeMap, BTreeSet, HashMap};

use anyhow::{Context, Result, bail, ensure};

use crate::{data::baseline::TimelineEvent, data::encode_browser_name, utils::save_bin_compressed};

const FORMAT_VERSION: u8 = 2;

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

/// The processed timeline: the (sorted) browser-id set and the deduplicated
/// `(date, snapshot)` events, each snapshot listing every browser in id order.
pub struct Events {
    browser_ids: Vec<u8>,
    events: Vec<(u32, Vec<(u8, String)>)>,
}

impl Events {
    /// Every version string the Baseline blob will reference, for the canonical version table.
    pub fn versions(&self) -> BTreeSet<String> {
        version_set(&self.events)
    }
}

fn version_set(events: &[(u32, Vec<(u8, String)>)]) -> BTreeSet<String> {
    events
        .iter()
        .flat_map(|(_, snapshot)| snapshot.iter().map(|(_, version)| version.clone()))
        .collect()
}

pub fn build_events(timeline: &[TimelineEvent]) -> Result<Events> {
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
    for event in timeline {
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

    Ok(Events { browser_ids: expected_browsers.into_iter().collect(), events })
}

/// Format: schema, browser count, event count, version count (u16 LE), browser IDs, the
/// version-id table (canonical version-table indices as u16 LE — Baseline version strings must
/// stay byte-identical to what `bbmTransform` emits, so they resolve through the shared table
/// like everything else), then fixed-size events: a u32 LE date key plus one version-id-table
/// index per browser, one byte wide while the table has at most 256 entries and two (u16 LE)
/// after that — the runtime derives the width from the version count. Fixed-size records let
/// the runtime binary-search by date without deserializing anything.
pub fn build_baseline(events: &Events, canonical: &HashMap<String, u16>) -> Result<()> {
    let Events { browser_ids, events } = events;
    let versions = version_set(events);

    let event_count = u16::try_from(events.len()).context("too many Baseline events")?;
    let browser_count = u8::try_from(browser_ids.len()).context("too many Baseline browsers")?;
    let version_count = u16::try_from(versions.len()).context("too many Baseline versions")?;
    let version_index: HashMap<&String, u16> =
        versions.iter().enumerate().map(|(i, v)| (v, i as u16)).collect();

    let mut bytes = Vec::new();
    bytes.push(FORMAT_VERSION);
    bytes.push(browser_count);
    bytes.extend_from_slice(&event_count.to_le_bytes());
    bytes.extend_from_slice(&version_count.to_le_bytes());
    bytes.extend_from_slice(browser_ids);
    for version in &versions {
        let index = *canonical
            .get(version.as_str())
            .with_context(|| format!("Baseline version {version} missing from canonical table"))?;
        bytes.extend_from_slice(&index.to_le_bytes());
    }
    for (date, snapshot) in events {
        bytes.extend_from_slice(&date.to_le_bytes());
        ensure!(
            snapshot.iter().map(|(id, _)| *id).eq(browser_ids.iter().copied()),
            "Baseline snapshot browser order changed"
        );
        for (_, version) in snapshot {
            let index = version_index[version];
            if versions.len() <= 256 {
                bytes.push(index as u8);
            } else {
                bytes.extend_from_slice(&index.to_le_bytes());
            }
        }
    }
    save_bin_compressed("baseline.bin", &bytes);
    Ok(())
}
