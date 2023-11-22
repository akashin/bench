#![no_main]
#![no_std]

use panic_halt as _;

extern "C" {
    fn assert_eq(actual: i64, expected: i64);
}

#[no_mangle]
pub fn main() {
    let mut a = 0i64;
    let mut b = 1i64;

    for _ in 0..10000 {
        let c = a.wrapping_add(b);
        a = b;
        b = c;
    }
    unsafe {
        assert_eq(a, -2872092127636481573);
    }
}
