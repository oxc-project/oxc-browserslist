#![no_main]

use use browserslist::{Distrib, Opts, resolve};

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        if s.chars().all(|c| !c.is_control()) {
            let _ = resolve([s], &Opts::default());
        }
    }
});
