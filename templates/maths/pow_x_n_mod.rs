//! Iterative modular exponentiation in logarithmic time and constant space.
//! Use a nonzero modulus. Products wider than `u64` are evaluated in `u128`.

pub fn pow(x: u64, mut n: u64, modulus: u64) -> u64 {
    assert!(modulus > 0);
    let modulus = u128::from(modulus);
    let mut base = u128::from(x) % modulus;
    let mut result = 1 % modulus;
    while n != 0 {
        if n & 1 != 0 {
            result = result * base % modulus;
        }
        n >>= 1;
        base = base * base % modulus;
    }
    result as u64
}
