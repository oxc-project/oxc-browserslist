use flate2::read::DeflateDecoder;
use std::io::Read;

/// Decompress deflate-compressed data.
pub fn decompress_deflate(compressed_data: &[u8]) -> Vec<u8> {
    let mut decoder = DeflateDecoder::new(compressed_data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).expect("Failed to decompress data");
    decompressed
}
