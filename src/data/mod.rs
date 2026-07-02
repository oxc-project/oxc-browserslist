pub mod caniuse;
pub mod electron;
pub mod node;

use std::borrow::Cow;

pub(crate) type BrowserName = Cow<'static, str>;

/// Resolve a version string from a concatenated pool by offset and length. Callers extract
/// these from their table's own packed-u32 layout.
///
/// Storing version strings this way keeps the generated tables free of `&str` fat pointers,
/// each of which would otherwise cost 16 bytes plus a load-time relocation entry in the binary.
#[inline]
pub(crate) fn unpack_str(pool: &'static str, offset: u32, len: u32) -> &'static str {
    &pool[offset as usize..(offset + len) as usize]
}

pub(crate) fn decode_browser_name(id: u8) -> BrowserName {
    match id {
        1 => Cow::Borrowed("ie"),
        2 => Cow::Borrowed("edge"),
        3 => Cow::Borrowed("firefox"),
        4 => Cow::Borrowed("chrome"),
        5 => Cow::Borrowed("safari"),
        6 => Cow::Borrowed("opera"),
        7 => Cow::Borrowed("ios_saf"),
        8 => Cow::Borrowed("op_mini"),
        9 => Cow::Borrowed("android"),
        10 => Cow::Borrowed("bb"),
        11 => Cow::Borrowed("op_mob"),
        12 => Cow::Borrowed("and_chr"),
        13 => Cow::Borrowed("and_ff"),
        14 => Cow::Borrowed("ie_mob"),
        15 => Cow::Borrowed("and_uc"),
        16 => Cow::Borrowed("samsung"),
        17 => Cow::Borrowed("and_qq"),
        18 => Cow::Borrowed("baidu"),
        19 => Cow::Borrowed("kaios"),
        _ => unreachable!("cannot recognize browser id"),
    }
}
