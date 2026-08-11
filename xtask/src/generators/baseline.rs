use std::{collections::HashMap, fs};

use anyhow::{Context, Result, bail, ensure};
use quote::quote;

use crate::utils::{date_to_julian_day, generate_file, root};

/// bbm short name -> (long name, is part of the Baseline core browser set). Mirrors
/// `nameMappings` in baseline-browser-mapping's src/index.ts in declaration order; core
/// browsers are the ones declared without an `engine` there.
const NAME_MAPPINGS: &[(&str, &str, bool)] = &[
    ("c", "chrome", true),
    ("ca", "chrome_android", true),
    ("e", "edge", true),
    ("f", "firefox", true),
    ("fa", "firefox_android", true),
    ("s", "safari", true),
    ("si", "safari_ios", true),
    ("o", "opera", false),
    ("oa", "opera_android", false),
    ("sa", "samsunginternet_android", false),
    ("wva", "webview_android", false),
    ("y", "ya_android", false),
    ("u", "uc_android", false),
    ("q", "qq_android", false),
    ("k", "kai_os", false),
    ("fb", "facebook_android", false),
    ("ia", "instagram_android", false),
];

pub fn build_baseline_timeline() -> Result<()> {
    let bundle_path = root().join("node_modules/baseline-browser-mapping/dist/index.js");
    let bundle = fs::read_to_string(&bundle_path)
        .with_context(|| format!("failed to read {}", bundle_path.display()))?;

    let timeline = extract_timeline_literal(&bundle)?;
    let rows = parse_timeline(&timeline)?;
    ensure!(rows.len() >= 400, "suspiciously few timeline rows: {}", rows.len());

    let mut pool = String::new();
    let mut seen: HashMap<String, u32> = HashMap::new();
    let entries = rows
        .iter()
        .map(|&(jdn, browser, ref version)| {
            let packed = *seen.entry(version.clone()).or_insert_with(|| {
                let offset = pool.len() as u32;
                let len = version.len() as u32;
                assert!(len < (1 << 8) && offset < (1 << 24), "version pool overflow");
                pool.push_str(version);
                offset << 8 | len
            });
            quote! { (#jdn, #browser, #packed) }
        })
        .collect::<Vec<_>>();
    let browsers = NAME_MAPPINGS
        .iter()
        .map(|&(_, long_name, is_core)| quote! { (#long_name, #is_core) })
        .collect::<Vec<_>>();

    let output = quote! {
        /// baseline-browser-mapping browsers as (long name, is in the Baseline core browser
        /// set), in bbm's `nameMappings` declaration order. Non-core browsers are downstream;
        /// `kai_os` is additionally gated by `include_kaios`.
        pub static BASELINE_BROWSERS: &[(&str, bool)] = &[#(#browsers),*];
        /// Concatenated version-string pool referenced by [`BASELINE_TIMELINE`]; unpack with
        /// `data::unpack_str`.
        pub static BASELINE_VERSIONS: &str = #pool;
        /// bbm timeline rows in original file order:
        /// `(section julian day, index into BASELINE_BROWSERS, pool_offset << 8 | pool_len)`.
        /// Section dates ascend; walking in order with last-write-wins per browser reproduces
        /// bbm's `getCompatibleVersions` minimum-version scan.
        pub static BASELINE_TIMELINE: &[(i32, u8, u32)] = &[#(#entries),*];
    };

    generate_file("baseline_timeline.rs", output);

    Ok(())
}

/// Extract the embedded timeline string from bbm's minified bundle. The package ships no raw
/// data file; the timeline exists only as one big quoted JS string starting with
/// `pre_baseline\n`. Locate that marker and scan the surrounding string literal.
fn extract_timeline_literal(bundle: &str) -> Result<String> {
    let bytes = bundle.as_bytes();
    let mut search_from = 0;
    let start = loop {
        let index = bundle[search_from..]
            .find("pre_baseline")
            .map(|i| i + search_from)
            .context("timeline marker `pre_baseline` not found in bbm bundle")?;
        // The data literal is `<quote>pre_baseline\n...`; other occurrences (the parser code's
        // `startsWith("pre_baseline")` etc.) are not followed by a newline escape.
        let after = &bundle[index + "pre_baseline".len()..];
        if index > 0
            && matches!(bytes[index - 1], b'"' | b'\'' | b'`')
            && (after.starts_with("\\n") || after.starts_with('\n'))
        {
            break index;
        }
        search_from = index + 1;
    };

    let quote = bytes[start - 1];
    let mut text = String::new();
    let mut i = start;
    loop {
        match *bytes.get(i).context("unterminated timeline string literal")? {
            b'\\' => {
                let escape = *bytes.get(i + 1).context("unterminated escape sequence")?;
                match escape {
                    b'n' => text.push('\n'),
                    b'\\' | b'"' | b'\'' | b'`' => text.push(escape as char),
                    _ => bail!("unsupported escape `\\{}` in timeline literal", escape as char),
                }
                i += 2;
            }
            b if b == quote => break,
            b => {
                text.push(b as char);
                i += 1;
            }
        }
    }
    Ok(text)
}

/// Parse the timeline text the same way bbm's own loader does: a `pre_baseline` header block
/// (rows ignored by `getCompatibleVersions`), then `YYYYMMDD` section headers each followed by
/// `short,version,release_date[,engine_version]` rows, then a trailing `releases` section
/// (only used by bbm APIs browserslist never calls). Returns (julian day, browser index,
/// version) rows in file order.
fn parse_timeline(timeline: &str) -> Result<Vec<(i32, u8, String)>> {
    // None = no header seen yet; Some(None) = in the pre_baseline block; Some(Some(jdn)) = in
    // a dated section.
    let mut section: Option<Option<i32>> = None;
    let mut rows = Vec::new();
    for line in timeline.split('\n') {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "releases" {
            break;
        }
        if !line.contains(',') {
            if line == "pre_baseline" {
                ensure!(section.is_none(), "pre_baseline header must come first");
                section = Some(None);
            } else {
                // bbm's loader keys date headers on `startsWith("20")`.
                ensure!(
                    line.len() == 8
                        && line.starts_with("20")
                        && line.bytes().all(|b| b.is_ascii_digit()),
                    "unexpected timeline section header: {line}"
                );
                let year: i32 = line[0..4].parse()?;
                let month: u32 = line[4..6].parse()?;
                let day: u32 = line[6..8].parse()?;
                let jdn = date_to_julian_day(year, month, day);
                if let Some(Some(previous)) = section {
                    // The runtime lookup walks rows in order; ascending dates make that walk
                    // equivalent to bbm's date-keyed object iteration.
                    ensure!(jdn > previous, "timeline section dates must be ascending: {line}");
                }
                section = Some(Some(jdn));
            }
            continue;
        }

        let fields = line.split(',').collect::<Vec<_>>();
        ensure!(matches!(fields.len(), 3 | 4), "unexpected timeline row: {line}");
        let (short_name, version) = (fields[0], fields[1]);
        let Some(browser) = NAME_MAPPINGS.iter().position(|&(short, ..)| short == short_name)
        else {
            // A new browser upstream must be triaged by a human (name mapping, core flag),
            // so fail the codegen run loudly instead of silently dropping it.
            bail!("unknown bbm browser short name `{short_name}` in row: {line}");
        };
        ensure!(
            !version.is_empty() && version.bytes().all(|b| b.is_ascii_digit() || b == b'.'),
            "unexpected version in timeline row: {line}"
        );
        match section {
            None => bail!("timeline row before any section header: {line}"),
            Some(None) => {} // pre_baseline rows never feed minimum-version scans
            Some(Some(jdn)) => {
                rows.push((jdn, u8::try_from(browser).unwrap(), version.to_string()))
            }
        }
    }
    Ok(rows)
}
