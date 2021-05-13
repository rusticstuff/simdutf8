pub trait BStrExt {
    fn repeat_x(&self, count: usize) -> Vec<u8>;
}

/// b"a".repeat() is not implemented for Rust 1.38.0 (MSRV)
impl<T> BStrExt for T
where
    T: AsRef<[u8]>,
{
    fn repeat_x(&self, count: usize) -> Vec<u8> {
        use std::io::Write;

        let x = self.as_ref();
        let mut res = Vec::with_capacity(x.len() * count);
        for _ in 0..count {
            #[allow(clippy::unwrap_used)]
            res.write_all(x).unwrap();
        }
        res
    }
}
