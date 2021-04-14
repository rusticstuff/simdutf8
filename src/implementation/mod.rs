#[macro_use]
mod macros;

#[allow(dead_code)]
pub(crate) mod avx2;

#[allow(dead_code)]
pub(crate) mod sse42;

struct Utf8CheckingState<T> {
    pub prev: T,
    pub incomplete: T,
    pub error: T,
}
