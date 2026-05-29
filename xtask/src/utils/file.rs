use std::fs;

use zopfli::{Format, Options};

use super::paths::root;

pub fn save_bin_compressed(file: &str, bytes: &[u8]) {
    // Compress with Zopfli, a (deliberately slow, build-time-only) deflate-compatible encoder that
    // squeezes a few percent more out of every blob than a normal deflate encoder. The output is a
    // standard raw-deflate stream, so the runtime decoder (`flate2::read::DeflateDecoder`) reads it
    // unchanged and no consumer-facing dependency changes.
    let mut compressed = Vec::new();
    zopfli::compress(Options::default(), Format::Deflate, bytes, &mut compressed).unwrap();
    let file = format!("{}.deflate", file);
    fs::write(root().join("src/generated").join(file), compressed).unwrap();
}

pub fn generate_file(file: &str, token_stream: proc_macro2::TokenStream) {
    let syntax_tree = syn::parse2(token_stream).unwrap();
    let code = prettyplease::unparse(&syntax_tree);
    fs::write(root().join("src/generated").join(file), code).unwrap();
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
