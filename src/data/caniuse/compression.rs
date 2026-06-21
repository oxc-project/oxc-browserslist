use std::sync::OnceLock;

use serde::de::DeserializeOwned;

/// Decompress raw-deflate-compressed data (RFC 1951).
///
/// The bundled blobs are produced at build time by the `xtask` Zopfli encoder, which emits a
/// standard raw-deflate stream. They are trusted, compile-time inputs, and decompression is cached
/// behind `OnceLock`, so this decoder favours small code size over speed (a bit-at-a-time canonical
/// Huffman decoder, modelled on zlib's `puff.c`). Correctness is pinned by a differential test
/// against `miniz_oxide` over every bundled blob (see the `tests` module below).
pub fn decompress_deflate(compressed_data: &[u8]) -> Vec<u8> {
    inflate::decompress(compressed_data)
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

/// Minimal raw-DEFLATE (RFC 1951) decoder. Safe-Rust port of the algorithm in zlib's `puff.c`.
mod inflate {
    const MAX_BITS: usize = 15; // longest Huffman code length
    const MAX_L_CODES: usize = 288; // max literal/length codes (incl. fixed-Huffman set)
    const ALL_CODES: usize = MAX_L_CODES + 32; // length-array sizing (litlen + distance)

    // Base length / extra bits for length symbols 257..=285 (indexed by symbol - 257).
    const LEN_BASE: [u16; 29] = [
        3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115,
        131, 163, 195, 227, 258,
    ];
    const LEN_EXTRA: [u8; 29] =
        [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0];
    // Base distance / extra bits for distance symbols 0..=29.
    const DIST_BASE: [u16; 30] = [
        1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
        2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
    ];
    const DIST_EXTRA: [u8; 30] = [
        0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12,
        13, 13,
    ];
    // Order in which code-length-code lengths are stored in a dynamic block header.
    const CL_ORDER: [usize; 19] =
        [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];

    /// LSB-first bit reader over a byte slice.
    struct BitReader<'a> {
        data: &'a [u8],
        pos: usize,
        bitbuf: u32,
        bitcnt: u32,
    }

    impl<'a> BitReader<'a> {
        fn new(data: &'a [u8]) -> Self {
            Self { data, pos: 0, bitbuf: 0, bitcnt: 0 }
        }

        /// Read `need` (0..=15) bits, least-significant bit first. Reads whole bytes only as
        /// required, so `bitcnt` stays < 8 afterwards (relied on by `align_to_byte`).
        fn bits(&mut self, need: u32) -> u32 {
            while self.bitcnt < need {
                self.bitbuf |= u32::from(self.data[self.pos]) << self.bitcnt;
                self.pos += 1;
                self.bitcnt += 8;
            }
            let val = self.bitbuf & ((1u32 << need) - 1);
            self.bitbuf >>= need;
            self.bitcnt -= need;
            val
        }

        /// Discard buffered bits to realign to the next byte boundary (for stored blocks).
        fn align_to_byte(&mut self) {
            self.bitbuf = 0;
            self.bitcnt = 0;
        }
    }

    /// Canonical Huffman decoding table: `count[n]` codes of length `n`, `symbol` sorted by code.
    struct Huffman {
        count: [u16; MAX_BITS + 1],
        symbol: [u16; MAX_L_CODES],
    }

    impl Huffman {
        fn new() -> Self {
            Self { count: [0; MAX_BITS + 1], symbol: [0; MAX_L_CODES] }
        }

        /// Build the decoding table from per-symbol code lengths (0 = symbol unused).
        fn construct(&mut self, lengths: &[u16]) {
            self.count = [0; MAX_BITS + 1];
            for &len in lengths {
                self.count[len as usize] += 1;
            }
            // Starting offset of each code length's run within `symbol`.
            let mut offs = [0u16; MAX_BITS + 1];
            for len in 1..MAX_BITS {
                offs[len + 1] = offs[len] + self.count[len];
            }
            for (sym, &len) in lengths.iter().enumerate() {
                if len != 0 {
                    self.symbol[offs[len as usize] as usize] = sym as u16;
                    offs[len as usize] += 1;
                }
            }
        }

        /// Decode one symbol by walking the canonical code one bit at a time.
        fn decode(&self, reader: &mut BitReader) -> u16 {
            let mut code: i32 = 0;
            let mut first: i32 = 0;
            let mut index: i32 = 0;
            for len in 1..=MAX_BITS {
                code |= reader.bits(1) as i32;
                let count = i32::from(self.count[len]);
                if code - count < first {
                    return self.symbol[(index + code - first) as usize];
                }
                index += count;
                first = (first + count) << 1;
                code <<= 1;
            }
            unreachable!("invalid Huffman code in trusted bundled data")
        }
    }

    /// Emit symbols for one compressed block using the given literal/length and distance tables.
    fn inflate_block(reader: &mut BitReader, out: &mut Vec<u8>, litlen: &Huffman, dist: &Huffman) {
        loop {
            let sym = litlen.decode(reader);
            match sym {
                0..=255 => out.push(sym as u8),
                256 => break, // end of block
                _ => {
                    let s = usize::from(sym - 257);
                    let length =
                        usize::from(LEN_BASE[s]) + reader.bits(u32::from(LEN_EXTRA[s])) as usize;
                    let d = usize::from(dist.decode(reader));
                    let distance =
                        usize::from(DIST_BASE[d]) + reader.bits(u32::from(DIST_EXTRA[d])) as usize;
                    let start = out.len() - distance;
                    // Byte-by-byte copy so overlapping back-references (distance < length) work.
                    for i in 0..length {
                        out.push(out[start + i]);
                    }
                }
            }
        }
    }

    /// Literal/length and distance tables for fixed-Huffman blocks (RFC 1951 §3.2.6).
    fn fixed_tables() -> (Huffman, Huffman) {
        let mut lengths = [0u16; MAX_L_CODES];
        lengths[0..144].fill(8);
        lengths[144..256].fill(9);
        lengths[256..280].fill(7);
        lengths[280..288].fill(8);
        let mut litlen = Huffman::new();
        litlen.construct(&lengths);
        let mut dist = Huffman::new();
        dist.construct(&[5u16; 30]);
        (litlen, dist)
    }

    /// Read a dynamic-block header, build its tables, and inflate the block (RFC 1951 §3.2.7).
    fn dynamic_block(reader: &mut BitReader, out: &mut Vec<u8>) {
        let nlen = reader.bits(5) as usize + 257;
        let ndist = reader.bits(5) as usize + 1;
        let ncode = reader.bits(4) as usize + 4;

        // Code-length code lengths, then the table that decodes the litlen+dist lengths.
        let mut cl_lengths = [0u16; 19];
        for &idx in CL_ORDER.iter().take(ncode) {
            cl_lengths[idx] = reader.bits(3) as u16;
        }
        let mut code_length = Huffman::new();
        code_length.construct(&cl_lengths);

        // Decode the combined literal/length + distance code lengths (with run-length repeats).
        let mut lengths = [0u16; ALL_CODES];
        let mut index = 0;
        while index < nlen + ndist {
            match code_length.decode(reader) {
                sym @ 0..=15 => {
                    lengths[index] = sym;
                    index += 1;
                }
                16 => {
                    let prev = lengths[index - 1];
                    for _ in 0..3 + reader.bits(2) {
                        lengths[index] = prev;
                        index += 1;
                    }
                }
                17 => {
                    for _ in 0..3 + reader.bits(3) {
                        lengths[index] = 0;
                        index += 1;
                    }
                }
                _ => {
                    for _ in 0..11 + reader.bits(7) {
                        lengths[index] = 0;
                        index += 1;
                    }
                }
            }
        }

        let mut litlen = Huffman::new();
        litlen.construct(&lengths[..nlen]);
        let mut dist = Huffman::new();
        dist.construct(&lengths[nlen..nlen + ndist]);
        inflate_block(reader, out, &litlen, &dist);
    }

    /// Copy a stored (uncompressed) block verbatim (RFC 1951 §3.2.4).
    fn stored_block(reader: &mut BitReader, out: &mut Vec<u8>) {
        reader.align_to_byte();
        let len =
            usize::from(reader.data[reader.pos]) | (usize::from(reader.data[reader.pos + 1]) << 8);
        reader.pos += 4; // skip LEN (2 bytes) + NLEN (2 bytes)
        out.extend_from_slice(&reader.data[reader.pos..reader.pos + len]);
        reader.pos += len;
    }

    pub fn decompress(data: &[u8]) -> Vec<u8> {
        let mut reader = BitReader::new(data);
        let mut out = Vec::with_capacity(data.len() * 4);
        loop {
            let last = reader.bits(1);
            match reader.bits(2) {
                0 => stored_block(&mut reader, &mut out),
                1 => {
                    let (litlen, dist) = fixed_tables();
                    inflate_block(&mut reader, &mut out, &litlen, &dist);
                }
                2 => dynamic_block(&mut reader, &mut out),
                _ => unreachable!("invalid block type in trusted bundled data"),
            }
            if last == 1 {
                return out;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::decompress_deflate;

    /// Every bundled blob must decode byte-for-byte identically to the reference `miniz_oxide`
    /// decoder. This pins the hand-rolled inflate to the encoder used by `xtask` and protects
    /// future caniuse-lite data updates from a silent divergence.
    #[test]
    fn matches_miniz_oxide_on_every_bundled_blob() {
        macro_rules! blobs {
            ($($p:literal),* $(,)?) => {
                [$( (&include_bytes!(concat!("../../generated/", $p))[..], $p) ),*]
            };
        }
        let blobs = blobs![
            "caniuse_browsers.bin.deflate",
            "caniuse_feature_browser_versions.bin.deflate",
            "caniuse_feature_keys.bin.deflate",
            "caniuse_feature_matching.bin.deflate",
            "caniuse_feature_version_table.bin.deflate",
            "caniuse_region_keys.bin.deflate",
            "caniuse_region_pair_indices.bin.deflate",
            "caniuse_region_pairs.bin.deflate",
            "caniuse_region_percentages.bin.deflate",
            "caniuse_region_version_table.bin.deflate",
            "node_versions.bin.deflate",
        ];
        for (blob, name) in blobs {
            let reference = miniz_oxide::inflate::decompress_to_vec(blob)
                .unwrap_or_else(|e| panic!("miniz_oxide failed on {name}: {e:?}"));
            assert_eq!(decompress_deflate(blob), reference, "inflate mismatch on {name}");
        }
    }

    /// Fuzz the decoder against `miniz_oxide` over many compressed streams that exercise stored,
    /// fixed-Huffman, and dynamic-Huffman blocks (varied data shapes + all compression levels).
    #[test]
    fn matches_miniz_oxide_on_fuzzed_streams() {
        // Deterministic xorshift PRNG (reproducible, no rng dependency).
        let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for case in 0..800u32 {
            let len = (next() % 5000) as usize;
            let raw: Vec<u8> = (0..len)
                .map(|i| match case % 4 {
                    0 => (next() & 0xff) as u8, // high-entropy (often stored blocks)
                    1 => (i % 7) as u8,         // short repetitive pattern (back-references)
                    2 => {
                        if next() % 5 == 0 {
                            (next() & 0xff) as u8
                        } else {
                            b'a'
                        }
                    } // text-like
                    _ => 0,                     // long zero runs
                })
                .collect();
            let level = (case % 11) as u8; // 0..=10 covers every block type
            let compressed = miniz_oxide::deflate::compress_to_vec(&raw, level);
            assert_eq!(decompress_deflate(&compressed), raw, "case {case} len {len} level {level}");
        }
    }
}
