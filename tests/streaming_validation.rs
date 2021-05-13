#[cfg(all(feature = "public_imp", target_feature = "avx2"))]
use simdutf8::basic::imp::x86::avx2::Utf8Validator;

#[test]
#[cfg(all(feature = "public_imp", target_feature = "avx2"))]
fn streaming_64_invalid() {
    // TODO: use repeat_x
    let mut input = b"a".repeat(63);
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
    // TODO: use repeat_x
    let mut input = b"a".repeat(64);
    unsafe {
        let mut validator = Utf8Validator::new();
        validator.update(&input);
        assert!(validator.finish().is_ok())
    }
}
