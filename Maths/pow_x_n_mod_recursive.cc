/*
 * Recursive binary exponentiation halves the exponent at each step.
 * Runtime and stack space are both logarithmic in the exponent.
 * Use a nonnegative base and exponent. Intermediate products must fit in long.
 * Use the default modulus because the recursive call omits the mod argument.
 */

long pow(long x, long n, long mod = 1'000'000'007L) {
    if (n == 0)
        return 1;
    long p = pow(x, n / 2);
    return p * p % mod * (n % 2 ? x : 1) % mod;
}
