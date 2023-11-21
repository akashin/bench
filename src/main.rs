fn main() {
    let mut a = 0i64;
    let mut b = 1i64;

    for _ in 0..10000 {
        let c = a.wrapping_add(b);
        a = b;
        b = c;
    }
    assert_eq!(a, -2872092127636481573);
}
