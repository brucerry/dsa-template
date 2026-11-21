//! The Sieve of Eratosthenes returns a primality flag for each index from 0 to n.

pub fn build_sieve(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    for flag in is_prime.iter_mut().take(2) {
        *flag = false;
    }
    let mut factor = 2;
    while factor <= n / factor {
        if is_prime[factor] {
            for multiple in (factor * factor..=n).step_by(factor) {
                is_prime[multiple] = false;
            }
        }
        factor += 1;
    }
    is_prime
}
