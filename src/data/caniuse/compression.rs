use std::io::Read;
use std::sync::OnceLock;

use flate2::read::DeflateDecoder;
use serde::de::DeserializeOwned;

/// Decompress deflate-compressed data.
pub fn decompress_deflate(compressed_data: &[u8]) -> Vec<u8> {
    let mut decoder = DeflateDecoder::new(compressed_data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).expect("Failed to decompress data");
    decompressed
}

/// Decompress a bundled blob and postcard-deserialize it in one step. Used directly by the few
/// call sites that transform the decoded value further (and reused by [`LazyData`]).
pub fn load<T: DeserializeOwned>(compressed: &[u8]) -> T {
    postcard::from_bytes(&decompress_deflate(compressed)).unwrap()
}

/// A bundled blob that is decompressed and postcard-deserialized into `T` on first access, then
/// cached for the rest of the process. Use this when the deserialized value is consumed as-is; use
/// [`LazyBlob`] when the bytes are decoded by hand at the use site.
pub struct LazyData<T> {
    compressed: &'static [u8],
    cell: OnceLock<T>,
}

impl<T: DeserializeOwned> LazyData<T> {
    pub const fn new(compressed: &'static [u8]) -> Self {
        Self { compressed, cell: OnceLock::new() }
    }

    pub fn get(&self) -> &T {
        self.cell.get_or_init(|| load(self.compressed))
    }
}

/// A bundled blob that is decompressed to raw bytes on first access, then cached. Use this for data
/// decoded by hand at the use site (the byte-plane and run-length blobs), where a generic
/// deserializer would bloat the binary.
pub struct LazyBlob {
    compressed: &'static [u8],
    cell: OnceLock<Vec<u8>>,
}

impl LazyBlob {
    pub const fn new(compressed: &'static [u8]) -> Self {
        Self { compressed, cell: OnceLock::new() }
    }

    pub fn get(&self) -> &[u8] {
        self.cell.get_or_init(|| decompress_deflate(self.compressed))
    }
}
