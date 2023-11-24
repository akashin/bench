#![no_main]
#![no_std]

use panic_halt as _;

use sha2::{Sha256, Digest};

extern "C" {
    fn assert_eq(actual: i64, expected: i64);
}

fn subslice_to_i64(slice: &[u8], begin: usize, end: usize) -> i64 {
    let mut result = 0i64;
    for i in begin..end {
        result <<= 8;
        result += slice[i] as i64;
    }
    result
}

#[no_mangle]
pub fn main() {
    let mut hasher = Sha256::new();
    hasher.update(b"hello world");
    let result = hasher.finalize();

    unsafe {
        assert_eq(subslice_to_i64(&result, 0, 8), 1);
        assert_eq(subslice_to_i64(&result, 8, 16), 2);
        assert_eq(subslice_to_i64(&result, 16, 24), 3);
        assert_eq(subslice_to_i64(&result, 24, 32), 4);
    }
}
