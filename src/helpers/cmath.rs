use std::mem::swap;

pub fn gcd(mut n: usize, mut m: usize) -> usize {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}
