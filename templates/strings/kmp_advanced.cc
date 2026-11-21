/*
 * Knuth–Morris–Pratt finds all occurrences of sub in s, including overlaps.
 * Returns zero-based start indices in ascending order.
 * Use a nonempty pattern and keep the separator '#' out of both inputs.
 * Time and space are linear in the combined length of the text and pattern.
 */

#include <cstddef>
#include <iterator>
#include <ranges>
#include <string>
#include <vector>

std::vector<int> kmp(std::string& s, std::string& sub) {
    std::string tmp = sub + '#' + s;
    std::vector<int> dp(std::size(tmp));
    for (std::size_t i : std::views::iota(std::size_t{1}, std::size(tmp))) {
        std::size_t cur = static_cast<std::size_t>(dp[i - 1]);
        while (cur and tmp[i] != tmp[cur]) {
            cur = static_cast<std::size_t>(dp[cur - 1]);
        }
        dp[i] = static_cast<int>(cur + (tmp[i] == tmp[cur]));
    }
    std::vector<int> k;
    std::ptrdiff_t pattern_length = std::ssize(sub);
    for (auto [i, match_length] : std::views::zip(std::views::iota(std::ptrdiff_t{0}), dp)) {
        if (match_length >= pattern_length) {
            k.push_back(static_cast<int>(i - 2 * pattern_length));
        }
    }
    return k;
}
