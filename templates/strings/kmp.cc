/*
 * Knuth–Morris–Pratt finds the first occurrence of needle in haystack.
 * strStr returns a zero-based index, -1 for no match, or 0 for an empty needle.
 * When calling kmp directly, pass a zero-filled table with one entry per needle character.
 */

#include <cstddef>
#include <iterator>
#include <ranges>
#include <string_view>
#include <vector>

void kmp(std::string_view needle, std::vector<int>& preproc) {
    std::size_t l = 0;
    for (std::size_t r : std::views::iota(std::size_t{0}, std::size(needle)) | std::views::drop(1)) {
        while (l and needle[l] != needle[r]) {
            l = static_cast<std::size_t>(preproc[l - 1]);
        }
        if (needle[l] == needle[r]) {
            preproc[r] = static_cast<int>(++l);
        }
    }
}

int strStr(std::string_view haystack, std::string_view needle) {
    std::size_t n = std::size(needle);
    if (!n) {
        return 0;
    }

    std::vector<int> preproc(n);
    kmp(needle, preproc);

    std::size_t j = 0;
    for (std::size_t i : std::views::iota(std::size_t{0}, std::size(haystack))) {
        while (j and haystack[i] != needle[j]) {
            j = static_cast<std::size_t>(preproc[j - 1]);
        }
        if (haystack[i] == needle[j] and ++j == n) {
            return static_cast<int>(i + 1 - n);
        }
    }
    return -1;
}
