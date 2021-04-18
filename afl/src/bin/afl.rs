#[macro_use]
extern crate afl;

fn main() {
    fuzz!(|data: &[u8]| {
        if let Ok(s) = std::str::from_utf8(data) {
            let simd_res = simdutf8::compat::from_utf8(data);
            let res = std::str::from_utf8(data);
            match (simd_res, res) {
                (Ok(_), Ok(_)) => {}
                (Ok(_), Err(_)) => {
                    panic!("simd: Ok, std: Err")
                }
                (Err(_), Ok(_)) => {
                    panic!("simd: Err, std: Ok")
                }
                (Err(simd_err), Err(std_err)) => {
                    assert_eq!(simd_err.valid_up_to(), std_err.valid_up_to());
                    assert_eq!(simd_err.error_len(), std_err.error_len());
                }
            }
        }
    });
}
