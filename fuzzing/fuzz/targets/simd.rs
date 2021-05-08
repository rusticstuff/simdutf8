#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

use common::test_utf8;

fuzz_target!(|data: &[u8]| {
    test_utf8(data);
});
