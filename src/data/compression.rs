use flate2::read::GzDecoder;
use std::io::Read;

/// Decompress gzip-compressed data
pub fn decompress_gzip(compressed_data: &[u8]) -> Vec<u8> {
    let mut decoder = GzDecoder::new(compressed_data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).expect("Failed to decompress data");
    decompressed
}