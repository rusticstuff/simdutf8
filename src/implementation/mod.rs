#[macro_use]
mod macros;

mod utf8check;

#[allow(dead_code)]
pub(crate) mod avx2;

#[allow(dead_code)]
pub(crate) mod sse42;
