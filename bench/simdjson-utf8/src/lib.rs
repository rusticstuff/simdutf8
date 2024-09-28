#![expect(non_upper_case_globals)]
#![expect(non_camel_case_types)]
#![expect(non_snake_case)]
use std::os::raw::c_char;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[inline(always)] // only works if simdjson ist compiled with LLVM and cross-language LTO is enabled
pub fn validate(bytes: &[u8]) -> bool {
    unsafe { simdjson_validate_utf8(bytes.as_ptr() as *const c_char, bytes.len() as size_t) }
}

#[cfg(test)]
mod tests {
    use super::validate;

    #[test]
    fn valid_utf8() {
        let hello = "hello world!";
        assert!(validate(hello.as_bytes()));
    }

    #[test]
    fn invalid_utf8() {
        let hello = b"\xff";
        assert!(!validate(hello));
    }
}
