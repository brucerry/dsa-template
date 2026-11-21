/*
 * The Z-function counts occurrences of a nonempty pattern in linear time.
 * Keep the separator '#' out of both the text and the pattern.
 * z_func gives the matching prefix length at each position and sets the first entry to 0.
 *
 * Adapted from https://github.com/ncduy0303/Competitive-Programming/blob/master/String%20Processing/Z-algo%20(Z%20function).cpp
 * Example problem at https://cses.fi/problemset/task/1753/
 */

#include <algorithm>
#include <cstddef>
#include <iterator>
#include <ranges>
#include <string>
#include <string_view>
#include <vector>
 
std::vector<int> z_func(std::string& s) {
    std::size_t n = std::size(s);
    std::vector<int> f(n);
    std::size_t l = 0, r = 0;
    for (std::size_t i : std::views::iota(std::size_t{0}, n) | std::views::drop(1)) {
        std::size_t match = 0;
        if (i <= r)
            match = std::min(r - i + 1, static_cast<std::size_t>(f[i - l]));
        while (i + match < n and s[match] == s[i + match]) {
            ++match;
        }
        f[i] = static_cast<int>(match);
        if (std::size_t end = i + match - 1; end > r) {
            l = i;
            r = end;
        }
    }
    return f;
}
 
int count_occurs(std::string_view s, std::string_view pattern) {
    std::string ps;
    ps.reserve(std::size(pattern) + 1 + std::size(s));
    ps += pattern;
    ps += '#';
    ps += s;
    std::ptrdiff_t n = std::ssize(pattern);
    std::vector<int> f = z_func(ps);
    return static_cast<int>(std::ranges::count(f | std::views::drop(n + 1), n));
}
