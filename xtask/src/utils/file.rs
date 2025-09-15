use std::{fs, io::Write};

use flate2::{Compression, write::DeflateEncoder};

use super::paths::root;

pub fn save_bin_compressed(file: &str, bytes: &[u8]) {
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(bytes).unwrap();
    let compressed = encoder.finish().unwrap();
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
