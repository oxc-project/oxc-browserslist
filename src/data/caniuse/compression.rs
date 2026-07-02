use std::sync::OnceLock;

use miniz_oxide::inflate::{
    TINFLStatus,
    core::{
        DecompressorOxide, decompress, inflate_flags::TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF,
    },
};
use serde::de::DeserializeOwned;

/// Decompress a bundled blob: the decompressed length as a little-endian u32, followed by a raw
/// deflate stream (see `save_bin_compressed` in xtask). Knowing the exact output size up front
/// lets us inflate in a single pass into an exact-size buffer, so only miniz_oxide's core
/// decompressor is linked in, not its buffer-growing `decompress_to_vec` wrapper.
pub fn decompress_blob(blob: &[u8]) -> Vec<u8> {
    let (len, compressed) = blob.split_at(4);
    let len = u32::from_le_bytes(len.try_into().unwrap()) as usize;
    let mut data = vec![0u8; len];
    let (status, _, written) = decompress(
        &mut DecompressorOxide::default(),
        compressed,
        &mut data,
        0,
        TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF,
    );
    assert!(status == TINFLStatus::Done && written == len, "Failed to decompress data");
    data
}

/// Decompress a bundled blob and postcard-deserialize it in one step. Used directly by the few
/// call sites that transform the decoded value further (and reused by [`LazyData`]).
pub fn load<T: DeserializeOwned>(compressed: &[u8]) -> T {
    postcard::from_bytes(&decompress_blob(compressed)).unwrap()
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
        self.cell.get_or_init(|| decompress_blob(self.compressed))
    }
}

#[cfg(test)]
mod tests {
    use super::decompress_blob;

    /// The codegen <-> runtime blob contract: every bundled blob's length header must match its
    /// deflate stream, and the single-pass decoder must produce exactly what miniz_oxide's
    /// reference `decompress_to_vec` produces. Enumerates the directory `save_bin_compressed`
    /// writes into, so blobs added later are covered automatically. Guards the nightly data
    /// regeneration.
    #[test]
    fn bundled_blobs_match_reference_decoder() {
        let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated");
        let mut checked = 0;
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_none_or(|ext| ext != "deflate") {
                continue;
            }
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            let blob = std::fs::read(&path).unwrap();
            let reference = miniz_oxide::inflate::decompress_to_vec(&blob[4..])
                .unwrap_or_else(|_| panic!("{name}: reference decode failed"));
            let len = u32::from_le_bytes(blob[..4].try_into().unwrap()) as usize;
            assert_eq!(len, reference.len(), "{name}: length header mismatch");
            assert_eq!(decompress_blob(&blob), reference, "{name}: decoded bytes differ");
            checked += 1;
        }
        assert!(checked > 0, "no .deflate blobs found in {dir}");
    }

    /// Exercise stored, fixed, and dynamic deflate blocks through the single-pass decoder by
    /// round-tripping various payload shapes at every compression level.
    #[test]
    fn roundtrip_synthetic_streams() {
        let mut lcg: u32 = 0x2545_f491;
        let noise: Vec<u8> = (0..4096)
            .map(|_| {
                lcg = lcg.wrapping_mul(1_103_515_245).wrapping_add(12345);
                (lcg >> 16) as u8
            })
            .collect();
        let shapes: &[Vec<u8>] = &[
            Vec::new(),
            b"hello deflate".to_vec(),
            vec![0u8; 10_000],
            b"abcdefgh".repeat(1000),
            noise,
        ];
        for data in shapes {
            for level in 0..=10 {
                let mut blob = u32::try_from(data.len()).unwrap().to_le_bytes().to_vec();
                blob.extend_from_slice(&miniz_oxide::deflate::compress_to_vec(data, level));
                assert_eq!(&decompress_blob(&blob), data, "shape len {} level {level}", data.len());
            }
        }
    }
}
