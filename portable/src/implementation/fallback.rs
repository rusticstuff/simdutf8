/// Fallback implementation using the standard library.
///
/// # Errors
/// Returns the zero-sized [`basic::Utf8Error`] on failure.
#[inline]
pub const fn validate_utf8_basic(input: &[u8]) -> Result<(), crate::basic::Utf8Error> {
    match core::str::from_utf8(input) {
        Ok(_) => Ok(()),
        Err(_) => Err(crate::basic::Utf8Error {}),
    }
}

/// Fallback implementation using the standard library.
///
/// # Errors
/// Returns [`compat::Utf8Error`] with detailed error information on failure.
#[inline]
pub fn validate_utf8_compat(input: &[u8]) -> Result<(), crate::compat::Utf8Error> {
    super::validate_utf8_at_offset(input, 0)
}

/// Low-level implementation of the [`basic::imp::Utf8Validator`] trait.
///
/// This is implementation requires CPU SIMD features specified by the module it resides in.
/// It is undefined behavior to call it if the required CPU features are not
/// available.
#[cfg(feature = "public_imp")]
pub struct Utf8ValidatorImp {
    expected_cont_bytes: u8,
    err: bool,
}

use core::panic;

#[cfg(feature = "public_imp")]
pub use Utf8ValidatorImp as ChunkedUtf8ValidatorImp;

#[cfg(feature = "public_imp")]
impl Utf8ValidatorImp {
    #[inline]
    fn update(&mut self, mut input: &[u8]) {
        if self.err {
            return;
        }
        if self.expected_cont_bytes > 0 {
            let to_check = (self.expected_cont_bytes as usize).min(input.len());
            for b in &input[..to_check] {
                if b & 0b1100_0000 != 0b1000_0000 {
                    // not a continuation byte
                    self.err = true;
                    return;
                }
                self.expected_cont_bytes -= 1;
            }
            if self.expected_cont_bytes > 0 {
                // not enough continuation bytes
                return;
            }
            input = &input[to_check..];
        }
        if let Err(e) = core::str::from_utf8(input) {
            // cannot wrap, since there is at least one byte left which is not valid UTF-8
            // by itself
            self.expected_cont_bytes = match input[e.valid_up_to()] {
                0b1100_0000..0b1110_0000 => 1,
                0b1110_0000..0b1111_0000 => 2,
                0b1111_0000..0b1111_1000 => 3,
                _ => {
                    // invalid byte for starting sequence
                    self.err = true;
                    return;
                }
            };
            let rem_input = input.len() - e.valid_up_to() - 1;
            if rem_input >= self.expected_cont_bytes as usize {
                // too many continuation bytes so they are not valid
                self.err = true;
                return;
            }
            for i in 0..rem_input {
                if input[e.valid_up_to() + i + 1] & 0b1100_0000 != 0b1000_0000 {
                    // not a continuation byte
                    self.err = true;
                    return;
                }
                self.expected_cont_bytes -= 1;
            }
        }
    }

    #[inline]
    const fn finalize(self) -> core::result::Result<(), crate::basic::Utf8Error> {
        if self.err || self.expected_cont_bytes > 0 {
            Err(crate::basic::Utf8Error {})
        } else {
            Ok(())
        }
    }
}

#[cfg(feature = "public_imp")]
impl crate::basic::imp::Utf8Validator for Utf8ValidatorImp {
    #[inline]
    #[must_use]
    fn new() -> Self {
        Self {
            expected_cont_bytes: 0,
            err: false,
        }
    }

    #[inline]
    fn update(&mut self, input: &[u8]) {
        if input.is_empty() {
            return;
        }
        self.update(input);
    }

    #[inline]
    fn finalize(self) -> core::result::Result<(), crate::basic::Utf8Error> {
        self.finalize()
    }
}

#[cfg(feature = "public_imp")]
impl crate::basic::imp::ChunkedUtf8Validator for Utf8ValidatorImp {
    #[inline]
    #[must_use]
    fn new() -> Self {
        Self {
            expected_cont_bytes: 0,
            err: false,
        }
    }

    #[inline]
    fn update_from_chunks(&mut self, input: &[u8]) {
        self.update(input);
    }

    #[inline]
    fn finalize(
        mut self,
        remaining_input: core::option::Option<&[u8]>,
    ) -> core::result::Result<(), crate::basic::Utf8Error> {
        if let Some(remaining_input) = remaining_input {
            self.update(remaining_input);
        }
        self.finalize()
    }
}
