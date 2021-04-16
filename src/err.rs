/// UTF-8 validation error
#[derive(Debug)]
pub struct Utf8Error {}

/// Exact UTF-8 validation error
#[derive(Debug)]
pub struct Utf8ErrorExact {
    pub(crate) valid_up_to: usize,
    pub(crate) error_len: Option<u8>,
}

impl Utf8ErrorExact {
    /// Returns the index in the given string up to which valid UTF-8 was
    /// verified.
    #[inline]
    #[must_use]
    pub fn valid_up_to(&self) -> usize {
        self.valid_up_to
    }

    /// Provides more information about the failure.
    #[inline]
    #[must_use]
    pub fn error_len(&self) -> Option<usize> {
        self.error_len.map(|len| len as usize)
    }
}
