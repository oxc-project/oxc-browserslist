use std::collections::{BTreeSet, HashMap};
use std::fs;

use postcard::to_allocvec;
use quote::{format_ident, quote};
use zopfli::{Format, Options};

use super::paths::root;

pub fn save_bin_compressed(file: &str, bytes: &[u8]) {
    // Compress with Zopfli, a (deliberately slow, build-time-only) deflate-compatible encoder that
    // squeezes a few percent more out of every blob than a normal deflate encoder. The output is a
    // standard raw-deflate stream prefixed with the decompressed length as a little-endian u32, so
    // the runtime decoder (`decompress_blob`) can allocate an exact-size buffer and inflate in
    // a single pass instead of pulling in miniz_oxide's buffer-growing `decompress_to_vec`.
    let mut compressed = u32::try_from(bytes.len()).unwrap().to_le_bytes().to_vec();
    zopfli::compress(Options::default(), Format::Deflate, bytes, &mut compressed).unwrap();
    let file = format!("{}.deflate", file);
    fs::write(root().join("src/generated").join(file), compressed).unwrap();
}

pub fn generate_file(file: &str, token_stream: proc_macro2::TokenStream) {
    let syntax_tree = syn::parse2(token_stream).unwrap();
    let code = prettyplease::unparse(&syntax_tree);
    fs::write(root().join("src/generated").join(file), code).unwrap();
}

/// Append one LEB128 varint (postcard's wire format, so the runtime's shared `read_varint`
/// decodes it) to `out`.
pub fn push_varint(out: &mut Vec<u8>, mut value: u64) {
    loop {
        let byte = (value & 0x7f) as u8;
        value >>= 7;
        if value == 0 {
            out.push(byte);
            return;
        }
        out.push(byte | 0x80);
    }
}

/// Zigzag-encode a signed value for varint storage (small magnitudes of either sign stay small).
pub fn zigzag(value: i64) -> u64 {
    ((value << 1) ^ (value >> 63)) as u64
}

pub fn create_range_vec(v: &Vec<Vec<u8>>) -> Vec<u32> {
    let mut offset = 0;
    // [start0, start1, ..., startN, endN]
    let mut ranges = vec![];
    for values in v {
        ranges.push(offset as u32);
        offset += values.len();
    }
    ranges.push(offset as u32);
    ranges
}

/// Deduplicate and lexicographically sort `values` into a string intern table, save it as a
/// compressed blob, and return the table alongside a value -> `u16` index map for remapping data.
/// Both the feature and region generators intern their version strings this way.
pub fn intern_table(
    blob: &str,
    values: impl IntoIterator<Item = String>,
) -> (Vec<String>, HashMap<String, u16>) {
    let table: Vec<String> = values.into_iter().collect::<BTreeSet<_>>().into_iter().collect();
    save_bin_compressed(blob, &to_allocvec(&table).unwrap());
    let index = table.iter().enumerate().map(|(i, v)| (v.clone(), i as u16)).collect();
    (table, index)
}

/// Save a lookup-key table and emit the matching `get_*` accessor module. The feature and region
/// matchers share this shape: binary-search a compressed key table for `name`, then build
/// `<module>::<value_type>` from the surrounding `(start, end)` slice of `RANGES`.
pub fn generate_keyed_lookup(
    out_rs: &str,
    keys_blob: &str,
    keys: &[String],
    ranges: &[u32],
    module: &str,
    value_type: &str,
    fn_name: &str,
) {
    save_bin_compressed(keys_blob, &to_allocvec(keys).unwrap());
    let keys_file = format!("{keys_blob}.deflate");
    let module = format_ident!("{module}");
    let value_type = format_ident!("{value_type}");
    let fn_name = format_ident!("{fn_name}");
    // Emit the narrowest offset type that fits: today both blobs' offsets fit u16, which halves
    // the table; if the data ever outgrows u16 the next regeneration silently falls back to u32.
    let max = *ranges.last().unwrap();
    let ranges_static = if max <= u32::from(u16::MAX) {
        let ranges = ranges.iter().map(|&r| r as u16).collect::<Vec<_>>();
        quote! { static RANGES: &[u16] = &[#(#ranges),*]; }
    } else {
        quote! { static RANGES: &[u32] = &[#(#ranges),*]; }
    };
    let output = quote! {
        use crate::data::caniuse::{compression::LazyData, #module::#value_type};

        static KEYS: LazyData<Vec<String>> = LazyData::new(include_bytes!(#keys_file));
        #ranges_static

        pub fn #fn_name(name: &str) -> Option<#value_type> {
            let index = KEYS.get().binary_search_by(|key| key.as_str().cmp(name)).ok()?;
            Some(#value_type::new(RANGES[index].into(), RANGES[index + 1].into()))
        }
    };
    generate_file(out_rs, output);
}
