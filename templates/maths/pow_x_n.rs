//! Binary exponentiation raises a floating-point base to a signed integer power.
//! Use a nonzero base for negative exponents.

pub fn pow(mut x: f64, n: i32) -> f64 {
    if n < 0 {
        x = x.recip();
    }
    let mut exponent = n.unsigned_abs();
    let mut result = 1.0;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result *= x;
        }
        exponent >>= 1;
        x *= x;
    }
    result
}
