#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

fuzz_target!(|data: &[u8]| {
    assert_eq!(
        simdutf8::basic::from_utf8(data).is_ok(),
        std::str::from_utf8(data).is_ok()
    );
});
