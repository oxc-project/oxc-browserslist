use crate::data::{BrowserName, caniuse, caniuse::compression::LazyBlob, decode_browser_name};

const FORMAT_VERSION: u8 = 2;
const HEADER_LEN: usize = 5;

static DATA: LazyBlob = LazyBlob::new(include_bytes!("../generated/baseline.bin.deflate"));

struct Header {
    browser_count: usize,
    event_count: usize,
    browser_ids_start: usize,
    version_table_start: usize,
    events_start: usize,
    event_len: usize,
}

fn read_u16(data: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(data[offset..offset + 2].try_into().unwrap())
}

fn read_u32(data: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap())
}

fn data() -> (&'static [u8], Header) {
    let data = DATA.get();
    assert_eq!(data[0], FORMAT_VERSION, "unsupported Baseline data format");
    let browser_count = usize::from(data[1]);
    let event_count = usize::from(read_u16(data, 2));
    let version_count = usize::from(data[4]);
    let browser_ids_start = HEADER_LEN;
    // Per-blob version ids resolve through the shared canonical version table (u16 LE indices).
    let version_table_start = browser_ids_start + browser_count;
    let events_start = version_table_start + version_count * 2;
    let event_len = 4 + browser_count;
    assert_eq!(data.len(), events_start + event_count * event_len, "invalid Baseline data");
    (
        data,
        Header {
            browser_count,
            event_count,
            browser_ids_start,
            version_table_start,
            events_start,
            event_len,
        },
    )
}

pub fn browsers() -> impl Iterator<Item = BrowserName> {
    let (data, header) = data();
    data[header.browser_ids_start..header.version_table_start]
        .iter()
        .map(|&id| decode_browser_name(id))
}

pub fn min_versions_on(cutoff: u64) -> Option<Versions> {
    let (data, header) = data();
    let mut left = 0;
    let mut right = header.event_count;
    while left < right {
        let middle = left + (right - left) / 2;
        let offset = header.events_start + middle * header.event_len;
        if u64::from(read_u32(data, offset)) <= cutoff {
            left = middle + 1;
        } else {
            right = middle;
        }
    }
    let index = left.checked_sub(1)?;
    let event_start = header.events_start + index * header.event_len + 4;
    Some(Versions {
        browser_ids: &data[header.browser_ids_start..header.version_table_start],
        version_table: &data[header.version_table_start..header.events_start],
        entries: &data[event_start..event_start + header.browser_count],
        index: 0,
    })
}

pub struct Versions {
    browser_ids: &'static [u8],
    version_table: &'static [u8],
    entries: &'static [u8],
    index: usize,
}

impl Iterator for Versions {
    type Item = (BrowserName, &'static str);

    fn next(&mut self) -> Option<Self::Item> {
        let id = *self.browser_ids.get(self.index)?;
        let local = usize::from(self.entries[self.index]);
        let canonical = usize::from(read_u16(self.version_table, local * 2));
        self.index += 1;
        Some((decode_browser_name(id), caniuse::version_table()[canonical].as_str()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.browser_ids.len() - self.index;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for Versions {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_events() {
        assert!(min_versions_on(20_150_728).is_none());
        let mut versions = min_versions_on(20_150_729).unwrap();
        assert_eq!(versions.len(), 14);
        assert!(versions.any(|(browser, version)| browser == "edge" && version == "12"));
        assert_eq!(min_versions_on(u64::MAX).unwrap().len(), 14);
    }
}
