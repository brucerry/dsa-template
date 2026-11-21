/*
 * Rabin–Karp counts occurrences of a nonempty pattern using rolling hashes.
 * A random base and two moduli reduce the chance of collisions.
 *
 * Adapted from https://github.com/ncduy0303/Competitive-Programming/blob/master/String%20Processing/String%20Hashing%20(Rabin-Karp).cpp
 * Example problem at https://cses.fi/problemset/task/1753
 */

#include <array>
#include <chrono>
#include <cstddef>
#include <iterator>
#include <random>
#include <ranges>
#include <string_view>
#include <vector>

std::mt19937 rng(static_cast<std::mt19937::result_type>(
    std::chrono::steady_clock::now().time_since_epoch().count()));
long long rand(long long l, long long r) {
    return std::uniform_int_distribution(l, r)(rng);
}

static long long BASE = rand(300, 1e9);
constexpr int NUM_MOD = 2;
constexpr std::array<long long, NUM_MOD> MODS{1'000'002'277, 1'000'005'277};

struct Hash {
    std::vector<long long> H[NUM_MOD], P[NUM_MOD];
    Hash(std::string_view s) {
        for (auto [hashes, powers, mod] : std::views::zip(H, P, MODS)) {
            powers.push_back(1);
            hashes.push_back(0);
            for (char c : s) {
                powers.push_back(powers.back() * BASE % mod);
                hashes.push_back((hashes.back() * BASE + c) % mod);
            }
        }
    }
    std::array<long long, NUM_MOD> operator()(int l, int r) {
        std::size_t start = static_cast<std::size_t>(l);
        std::size_t end = static_cast<std::size_t>(r + 1);
        std::array<long long, NUM_MOD> res;
        for (auto [value, hashes, powers, mod] : std::views::zip(res, H, P, MODS)) {
            value = (hashes[end] - hashes[start] * powers[end - start]) % mod;
            if (value < 0)
                value += mod;
        }
        return res;
    }
};

int count_occurs(std::string_view s, std::string_view pattern) {
    int n = static_cast<int>(std::size(s));
    int offset = static_cast<int>(std::size(pattern)) - 1;
    if (offset >= n) {
        return 0;
    }

    Hash hs(s), ht(pattern);
    std::array<long long, NUM_MOD> target = ht(0, offset);
    int res = 0;
    for (int end : std::views::iota(offset, n)) {
        res += hs(end - offset, end) == target;
    }
    return res;
}
