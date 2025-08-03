use std::io::Read;

use bincode::BorrowDecode;
use flate2::read::DeflateDecoder;

/// Decompress gzip-compressed data
pub fn decompress_deflate(compressed_data: &[u8]) -> Vec<u8> {
    let mut decoder = DeflateDecoder::new(compressed_data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).expect("Failed to decompress data");
    decompressed
}

pub fn decode<'a, T: BorrowDecode<'a, ()>>(data: &'a [u8], start: u32, end: u32) -> Vec<T> {
    bincode::borrow_decode_from_slice(
        &data[start as usize..end as usize],
        bincode::config::standard(),
    )
    .unwrap()
    .0
}
