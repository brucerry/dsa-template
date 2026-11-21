/*
 * Iterative binary exponentiation computes x raised to n modulo mod.
 * Use x >= 0, n >= 0, and mod > 1. Intermediate products must fit in long.
 * Runs in logarithmic time with constant extra space.
 */

long pow(long x, long n, long mod = 1'000'000'007L) {
    long res = 1, p = x % mod;
    while (n) {
        if (n & 1) {
            res = (res * p) % mod;
        }
        p = (p * p) % mod;
        n >>= 1;
    }
    return res;
}
