#![no_main]
#![no_std]

use panic_halt as _;

use sha2::{Digest, Sha256};

extern "C" {
    fn assert_eq_i64(actual: u64, expected: u64);
}
// fn assert_eq_i64(actual: u64, expected: u64) {
//     assert_eq!(actual, expected);
// }

fn assert_eq_array(expected: &[u8], actual: &[u8]) {
    for i in 0..32 {
        unsafe {
            assert_eq_i64(expected[i] as u64, actual[i] as u64);
        }
    }
}

#[no_mangle]
pub fn main() {
    let hash = Sha256::digest(b"X");
    let expected = [
        75, 104, 171, 56, 71, 254, 218, 125, 108, 98, 193, 251, 203, 238, 191, 163, 94, 171, 115,
        81, 237, 94, 120, 244, 221, 173, 234, 93, 246, 75, 128, 21,
    ];

    assert_eq_array(&expected, &hash)
}
