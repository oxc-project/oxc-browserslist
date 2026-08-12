use std::{collections::HashMap, fs};

use anyhow::{Context, Result, ensure};
use indexmap::IndexMap;

use crate::utils::{push_varint, root, save_bin_compressed};

/// The parsed electron-to-chromium mapping: `(major, minor, chromium version)`, in
/// versions.json order (ascending — asserted in `build_electron_to_chromium`).
pub struct ElectronVersion {
    pub major: u8,
    pub minor: u8,
    pub chromium: String,
}

pub fn load_electron_versions() -> Result<Vec<ElectronVersion>> {
    let data_path = root().join("node_modules/electron-to-chromium/versions.json");
    serde_json::from_slice::<IndexMap<String, String>>(&fs::read(data_path)?)?
        .into_iter()
        .map(|(electron_version, chromium)| {
            let split = electron_version.split('.').collect::<Vec<_>>();
            ensure!(split.len() == 2, "electron version must be in major.minor format");
            let major: u8 = split[0].parse().context("electron major overflow")?;
            let minor: u8 = split[1].parse().context("electron minor overflow")?;
            Ok(ElectronVersion { major, minor, chromium })
        })
        .collect()
}

const FORMAT_VERSION: u8 = 1;

/// Electron (major, minor) -> Chromium version, as one compressed varint stream. Each entry
/// packs `major << 24 | minor << 16 | canonical version-table index`; the Chromium strings are
/// chrome release versions, so they live in the shared canonical table and this blob carries no
/// string pool of its own. Entries are ascending (the `last N electron versions` queries walk
/// back-to-front), and major/minor sit in the top bits, so ascending versions are exactly
/// ascending packed values — stored as a first absolute value plus positive deltas.
///
/// Layout: `u8 FORMAT_VERSION`, varint entry count, then per entry a varint (first absolute,
/// then delta to the previous packed value).
pub fn build_electron_to_chromium(
    data: &[ElectronVersion],
    canonical: &HashMap<String, u16>,
) -> Result<()> {
    let packed: Vec<u32> = data
        .iter()
        .map(|version| {
            let index = *canonical.get(version.chromium.as_str()).with_context(|| {
                format!("chromium version {} missing from canonical table", version.chromium)
            })?;
            Ok(u32::from(version.major) << 24 | u32::from(version.minor) << 16 | u32::from(index))
        })
        .collect::<Result<_>>()?;
    ensure!(packed.is_sorted(), "electron versions must be ascending");

    let mut bytes = vec![FORMAT_VERSION];
    push_varint(&mut bytes, packed.len() as u64);
    let mut previous = 0u32;
    for value in packed {
        push_varint(&mut bytes, u64::from(value - previous));
        previous = value;
    }
    save_bin_compressed("electron.bin", &bytes);
    Ok(())
}
