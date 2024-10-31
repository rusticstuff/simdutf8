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
    incomplete_data: [u8; 4],
    incomplete_len: u8,
    err: bool,
}

use core::panic;

#[cfg(feature = "public_imp")]
pub use Utf8ValidatorImp as ChunkedUtf8ValidatorImp;

#[cfg(feature = "public_imp")]
impl Utf8ValidatorImp {
    #[inline]
    #[expect(clippy::cast_possible_truncation)]
    fn update(&mut self, mut input: &[u8]) {
        if self.err {
            return;
        }
        if self.incomplete_len > 0 {
            let total_bytes_needed: usize = match self.incomplete_data[0] {
                0..0b1000_0000 => {
                    panic!("ASCII data should never be incomplete");
                }
                0b1000_0000..0b1100_0000 => {
                    // first byte cannot be a continuation byte
                    self.err = true;
                    return;
                }
                0b1100_0000..0b1110_0000 => 2,
                0b1110_0000..0b1111_0000 => 3,
                0b1111_0000..0b1111_1000 => 4,
                _ => {
                    // invalid byte for starting sequence
                    self.err = true;
                    return;
                }
            };
            if self.incomplete_len as usize >= total_bytes_needed {
                // actually errored on previous update
                self.err = true;
                return;
            }
            let bytes_needed = total_bytes_needed - self.incomplete_len as usize;
            let to_copy = core::cmp::min(bytes_needed, input.len());
            self.incomplete_data
                [self.incomplete_len as usize..self.incomplete_len as usize + to_copy]
                .copy_from_slice(&input[..to_copy]);
            if to_copy < bytes_needed {
                self.incomplete_len += to_copy as u8;
                return;
            }
            if core::str::from_utf8(&self.incomplete_data[..total_bytes_needed]).is_err() {
                self.err = true;
                return;
            }
            self.incomplete_len = 0;
            input = &input[to_copy..];
        }
        if let Err(e) = core::str::from_utf8(input) {
            if input.len() - e.valid_up_to() > 3 {
                self.err = true;
                return;
            }
            self.incomplete_len = (input.len() - e.valid_up_to()) as u8;
            self.incomplete_data[..self.incomplete_len as usize]
                .copy_from_slice(&input[e.valid_up_to()..]);
        }
    }

    #[inline]
    const fn finalize(self) -> core::result::Result<(), crate::basic::Utf8Error> {
        if self.err || self.incomplete_len > 0 {
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
            incomplete_data: [0; 4],
            incomplete_len: 0,
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
            incomplete_data: [0; 4],
            incomplete_len: 0,
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
