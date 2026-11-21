/*
 * A binary trie stores keys as bit paths, from most significant to least significant.
 * Keep keys nonnegative and below 2^31 to avoid setting the sign bit of the int
 * used to assemble the result.
 * find_possible returns a key that minimizes XOR with the target.
 * Insert at least one key before querying.
 */

#include <ranges>

constexpr int NUM_BIT = 32;

struct Trie {
    Trie* children[2] = {};

    Trie() = default;

    void insert(long num) {
        Trie* cur = this;
        for (int i : std::views::iota(0, NUM_BIT) | std::views::reverse) {
            int bit = (num >> i) & 1;
            if (!cur->children[bit])
                cur->children[bit] = new Trie();
            cur = cur->children[bit];
        }
    }

    long find_possible(long target) {
        long res = 0;
        Trie* cur = this;
        for (int i : std::views::iota(0, NUM_BIT) | std::views::reverse) {
            int bit = (target >> i) & 1;
            if (!cur->children[bit]) {
                cur = cur->children[bit^1];
                res |= (bit ^ 1) << i;
            }
            else {
                cur = cur->children[bit];
                res |= bit << i;
            }
        }
        return res;
    }
};
