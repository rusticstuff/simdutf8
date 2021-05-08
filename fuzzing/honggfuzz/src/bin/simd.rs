use common::test_utf8;
use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            test_utf8(data);
        });
    }
}
