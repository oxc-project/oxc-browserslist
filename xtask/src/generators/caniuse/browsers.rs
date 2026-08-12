use std::collections::HashMap;

use anyhow::{Context, Result, ensure};
use indexmap::IndexMap;

use crate::{
    data::{Agent, Caniuse, encode_browser_name},
    utils::{push_varint, save_bin_compressed, zigzag},
};

const FORMAT_VERSION: u8 = 2;

/// The root caniuse dataset: every browser's version list with per-version global usage and
/// release date, plus the global-usage query order. One hand-decoded stream replaces what used
/// to be a nested-postcard browsers blob, a separate global-usage `.rs` table with its own
/// string pool, and (via the shared canonical table) two duplicate version-string tables.
///
/// Layout (all varints are LEB128, the wire format the runtime's shared `read_varint` decodes;
/// the runtime reader is `decode_browsers` in `src/data/caniuse.rs`, section for section):
///
/// ```text
/// u8   FORMAT_VERSION
/// u32  date unit (LE) — gcd of every release date; 86400 while upstream stays midnight-UTC.
///      A non-midnight date upstream degrades the unit (worst case 1s) with no code change.
/// u8   usage-table length, then that many u32 (LE) f32 bit patterns, value-descending —
///      the distinct nonzero per-version global-usage values (~101 for 681 versions)
/// u8   browser count, then that many browser ids (caniuse agents order)
/// per browser: varint version count
/// per browser: varint released count — versions[..count] have release dates, the rest are
///              unreleased (upstream keeps the dateless entries trailing; asserted below)
/// per browser, per version: varint canonical version-table index (version_list order)
/// per browser, per version: varint usage index — 0 = zero usage, else usage_table[i - 1]
/// per browser, per released version: varint zigzag day delta (first entry absolute);
///              days are `release_date / unit`, deltas are signed (releases are not monotonic)
/// varint global-usage entry count, then per entry: varint flat version position — the
///              usage-descending order `> N%`/`cover` queries iterate, expressed as positions
///              into the version stream above. Stored (not re-derived at runtime) because
///              equal-usage ties fix which versions a `cover` boundary selects.
/// ```
pub fn build_caniuse_browsers(data: &Caniuse, canonical: &HashMap<String, u16>) -> Result<()> {
    let agents = &data.agents;
    let date_unit = date_unit(agents)?;
    let usage_table = usage_table(agents)?;
    let usage_index: HashMap<u32, u8> =
        usage_table.iter().enumerate().map(|(i, u)| (u.to_bits(), (i + 1) as u8)).collect();

    let mut bytes = Vec::new();

    // Header: format version and date unit.
    bytes.push(FORMAT_VERSION);
    bytes.extend_from_slice(
        &u32::try_from(date_unit).context("date unit overflows u32")?.to_le_bytes(),
    );

    // Usage intern table.
    bytes.push(usage_table.len() as u8);
    for usage in &usage_table {
        bytes.extend_from_slice(&usage.to_bits().to_le_bytes());
    }

    // Browser ids.
    ensure!(agents.len() <= 255, "browser count overflows u8");
    bytes.push(agents.len() as u8);
    for name in agents.keys() {
        bytes.push(encode_browser_name(name));
    }

    // Per-browser version counts, then released counts.
    for agent in agents.values() {
        push_varint(&mut bytes, agent.version_list.len() as u64);
    }
    for (name, agent) in agents {
        push_varint(&mut bytes, released_count(name, agent)? as u64);
    }

    // Per-version canonical version-table indices.
    for (name, agent) in agents {
        for version in &agent.version_list {
            let index = *canonical.get(&version.version).with_context(|| {
                format!("{name} {} missing from canonical table", version.version)
            })?;
            push_varint(&mut bytes, u64::from(index));
        }
    }

    // Per-version usage indices.
    for agent in agents.values() {
        for version in &agent.version_list {
            let index = if version.global_usage == 0.0 {
                0
            } else {
                usage_index[&version.global_usage.to_bits()]
            };
            push_varint(&mut bytes, u64::from(index));
        }
    }

    // Per-browser release-date day deltas.
    for agent in agents.values() {
        let mut previous_day = 0i64;
        for version in &agent.version_list {
            let Some(date) = version.release_date else { break };
            let day = date / date_unit;
            push_varint(&mut bytes, zigzag(day - previous_day));
            previous_day = day;
        }
    }

    // Global-usage order, as flat positions into the version stream.
    let positions = global_usage_positions(agents)?;
    push_varint(&mut bytes, positions.len() as u64);
    for position in positions {
        push_varint(&mut bytes, position);
    }

    save_bin_compressed("caniuse_browsers.bin", &bytes);
    Ok(())
}

/// The gcd of all release dates. Upstream dates are midnight-UTC seconds today, so this is
/// 86400 and day values fit ~2 varint bytes.
fn date_unit(agents: &IndexMap<String, Agent>) -> Result<i64> {
    let mut unit: i64 = 0;
    for agent in agents.values() {
        for version in &agent.version_list {
            if let Some(date) = version.release_date {
                ensure!(date > 0, "release date must be positive (0 is the None niche)");
                unit = gcd(unit, date);
            }
        }
    }
    Ok(if unit == 0 { 1 } else { unit })
}

const fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// The distinct nonzero usage values, value-descending. Only ~101 distinct values exist across
/// 681 versions, so each version stores a 1-byte table index instead of 4 high-entropy mantissa
/// bytes. Exact by construction — the table holds the very bits.
fn usage_table(agents: &IndexMap<String, Agent>) -> Result<Vec<f32>> {
    let mut values: Vec<f32> = agents
        .values()
        .flat_map(|agent| agent.version_list.iter().map(|version| version.global_usage))
        .filter(|usage| *usage != 0.0)
        .collect();
    values.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
    values.dedup();
    ensure!(values.len() <= 255, "usage intern table overflows u8");
    Ok(values)
}

/// How many leading versions have a release date. The dateless (unreleased) versions must all
/// trail, so one count per browser replaces a presence flag per version.
fn released_count(name: &str, agent: &Agent) -> Result<usize> {
    let released =
        agent.version_list.iter().take_while(|version| version.release_date.is_some()).count();
    ensure!(
        agent.version_list[released..].iter().all(|version| version.release_date.is_none()),
        "browser {name} has a dateless version before a dated one"
    );
    Ok(released)
}

/// The usage-descending `(browser, version)` order for `> N%`/`cover` queries, as positions
/// into the flat version stream. Built EXACTLY as the deleted global_usage.rs generator did
/// (same input order, same filter, same unstable sort) so the committed order — including the
/// outcome of equal-usage ties, which decide `cover` boundary selection — is preserved
/// verbatim. Do not "canonicalize" the tie order; that would change query results.
fn global_usage_positions(agents: &IndexMap<String, Agent>) -> Result<Vec<u64>> {
    let mut global_usage: Vec<(u8, String, f32)> = agents
        .iter()
        .flat_map(|(name, agent)| {
            let browser_id = encode_browser_name(name);
            agent
                .usage_global
                .iter()
                .filter(|(_, usage)| **usage > 0.0f32)
                .map(move |(version, usage)| (browser_id, version.clone(), *usage))
        })
        .collect();
    global_usage.sort_unstable_by(|(_, _, a), (_, _, b)| b.partial_cmp(a).unwrap());

    // Flat position (and stored usage bits) of every (browser, version) in the version stream.
    let mut flat_position: HashMap<(u8, &str), (u64, u32)> = HashMap::new();
    let mut flat = 0u64;
    for (name, agent) in agents {
        let id = encode_browser_name(name);
        for version in &agent.version_list {
            flat_position
                .insert((id, version.version.as_str()), (flat, version.global_usage.to_bits()));
            flat += 1;
        }
    }

    global_usage
        .iter()
        .map(|(id, version, usage)| {
            // The stream must reproduce this entry bit-for-bit from the per-version usage data.
            let (position, stored_bits) =
                *flat_position.get(&(*id, version.as_str())).with_context(|| {
                    format!("usage entry {version} not in browser {id}'s version_list")
                })?;
            ensure!(
                stored_bits == usage.to_bits(),
                "usage_global and version_list disagree for browser id {id} version {version}"
            );
            Ok(position)
        })
        .collect()
}
