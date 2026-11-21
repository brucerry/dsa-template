/*
 * A Fenwick tree supports point assignments and inclusive range sums.
 * Public indices are zero-based. Updates and queries take O(log n) time.
 */

#include <algorithm>
#include <cstddef>
#include <iterator>
#include <ranges>
#include <vector>

struct Fenwick {
    Fenwick(std::vector<int>& values) : nums(values), fenwick(std::size(values) + 1) {
        for (std::size_t i : std::views::iota(std::size_t{1}, std::size(fenwick))) {
            init_fenwick(i);
        }
    }

    void update(int index, int val) {
        std::size_t position = static_cast<std::size_t>(index);
        int diff = val - nums[position];
        nums[position] = val;
        for (std::size_t i = position + 1; i < std::size(fenwick); i += lowbit(i)) {
            fenwick[i] += diff;
        }
    }

    int sumRange(int left, int right) {
        return sum(right + 1) - sum(left);
    }

private:
    std::vector<int> nums, fenwick;

    std::size_t lowbit(std::size_t x) {
        return x & (~x + 1);
    }

    void init_fenwick(std::size_t f) {
        int val = nums[f - 1];
        while (f < std::size(fenwick)) {
            fenwick[f] += val;
            f += lowbit(f);
        }
    }

    int sum(int count) {
        int res = 0;
        for (std::size_t i = static_cast<std::size_t>(std::max(count, 0)); i; i -= lowbit(i)) {
            res += fenwick[i];
        }
        return res;
    }
};
