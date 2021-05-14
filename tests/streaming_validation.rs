#![cfg(feature = "public_imp")]

mod common;

#[allow(unused_imports)]
use common::BStrExt;

#[allow(unused_imports)]
use simdutf8::basic::imp::Utf8Validator;

#[cfg(target_feature = "avx2")]
use simdutf8::basic::imp::x86::avx2::Utf8ValidatorImp;

#[test]
#[cfg(target_feature = "avx2")]
fn streaming_64_invalid() {
    let mut input = b"a".repeat_x(63);
    input.push(b'\xff');
    unsafe {
        let mut validator = Utf8ValidatorImp::new();
        validator.update(&input);
        assert!(validator.finalize().is_err());
    }
}

#[test]
#[cfg(target_feature = "avx2")]
fn streaming_64_valid() {
    let mut input = b"a".repeat_x(64);
    unsafe {
        let mut validator = Utf8ValidatorImp::new();
        validator.update(&input);
        assert!(validator.finalize().is_ok())
    }
}
