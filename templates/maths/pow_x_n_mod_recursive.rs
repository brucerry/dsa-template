//! Recursive modular exponentiation in logarithmic time and stack space.
//! A nonzero modulus is used at every level. Products are evaluated in `u128`.

pub fn pow(x: u64, n: u64, modulus: u64) -> u64 {
    assert!(modulus > 0);
    let modulus = u128::from(modulus);
    power(u128::from(x) % modulus, n, modulus) as u64
}

fn power(base: u128, n: u64, modulus: u128) -> u128 {
    if n == 0 {
        return 1 % modulus;
    }
    let half = power(base, n / 2, modulus);
    let square = half * half % modulus;
    if n & 1 == 0 {
        square
    } else {
        square * base % modulus
    }
}
