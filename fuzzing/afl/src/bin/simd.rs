#[macro_use]
extern crate afl;

use common::test_utf8;

fn main() {
    fuzz!(|data: &[u8]| {
        test_utf8(data);
    });
}
