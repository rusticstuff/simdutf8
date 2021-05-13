mod common;
use common::BStrExt;

#[cfg(all(feature = "public_imp", target_feature = "avx2"))]
use simdutf8::basic::imp::x86::avx2::Utf8Validator;

#[test]
#[cfg(all(feature = "public_imp", target_feature = "avx2"))]
fn streaming_64_invalid() {
    let mut input = b"a".repeat_x(63);
    input.push(b'\xff');
    unsafe {
        let mut validator = Utf8Validator::new();
        validator.update(&input);
        assert!(validator.finish().is_err());
    }
}

#[test]
#[cfg(all(feature = "public_imp", target_feature = "avx2"))]
fn streaming_64_valid() {
    let mut input = b"a".repeat_x(64);
    unsafe {
        let mut validator = Utf8Validator::new();
        validator.update(&input);
        assert!(validator.finish().is_ok())
    }
}
