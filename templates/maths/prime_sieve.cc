/*
 * The Sieve of Eratosthenes marks primes from 0 through n, inclusive.
 * Use n >= 1. The result stores 1 for primes and 0 for all other entries.
 */

#include <cstddef>
#include <vector>

constexpr int LARGEST_NUMBER = 1e9;

std::vector<int> build_sieve(int n = LARGEST_NUMBER) {
    std::size_t limit = static_cast<std::size_t>(n);
    std::vector<int> isprime(limit + 1, true);
    isprime[0] = isprime[1] = false;
    for (std::size_t f = 2; f * f <= limit; ++f) {
        if (isprime[f]) {
            for (std::size_t x = f * f; x <= limit; x += f) {
                isprime[x] = false;
            }
        }
    }
    return isprime;
}
