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

/// Read one LEB128 varint (postcard's wire format; see `push_varint` in xtask) from `bytes`,
/// advancing `pos`. The writer's contract is u64, so accumulate in u64 — on 32-bit targets
/// (wasm32) a usize accumulator would silently truncate large values such as date deltas.
fn read_varint_u64(bytes: &[u8], pos: &mut usize) -> u64 {
    let mut result = 0;
    let mut shift = 0;
    loop {
        let byte = bytes[*pos];
        *pos += 1;
        result |= u64::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            return result;
        }
        shift += 7;
    }
}

/// Read one varint that the format guarantees to be a count or index (fits usize on every
/// supported target; a violation is corrupt data and panics loudly).
pub fn read_varint(bytes: &[u8], pos: &mut usize) -> usize {
    usize::try_from(read_varint_u64(bytes, pos)).unwrap()
}

/// Read one zigzag-encoded signed varint (see `zigzag` in xtask).
pub fn read_zigzag(bytes: &[u8], pos: &mut usize) -> i64 {
    let value = read_varint_u64(bytes, pos);
    ((value >> 1) as i64) ^ -((value & 1) as i64)
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
