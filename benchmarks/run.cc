#include <algorithm>
#include <atomic>
#include <charconv>
#include <chrono>
#include <cstddef>
#include <cstdint>
#include <iostream>
#include <iterator>
#include <string>
#include <string_view>
#include <system_error>
#include <utility>
#include <vector>

#include "../templates/arrays/kadane.cc"
#include "../templates/graphs/union_find.cc"
#include "../templates/maths/prime_sieve.cc"
#include "../templates/strings/kmp.cc"
#include "../templates/trees/fenwick_sum.cc"
#include "../templates/trees/segment_sum.cc"

namespace {

using Measurement = std::pair<std::int64_t, std::int64_t>;

struct Random {
    std::uint32_t state = 240513;

    std::uint32_t next() {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        return state;
    }
};

template<class Run>
auto measure(Run run) {
    using Clock = std::chrono::steady_clock;
    auto start = Clock::now();
    std::atomic_signal_fence(std::memory_order_seq_cst);
    auto result = run();
    std::atomic_signal_fence(std::memory_order_seq_cst);
    auto elapsed = std::chrono::duration_cast<std::chrono::nanoseconds>(Clock::now() - start);
    return std::pair{std::move(result), elapsed.count()};
}

Measurement run_kadane(int size) {
    Random random;
    std::vector<int> values(static_cast<std::size_t>(size));
    for (int& value : values)
        value = static_cast<int>(random.next() % 101) - 50;
    return measure([&] { return kadane(values); });
}

Measurement run_union_find(int size) {
    Random random;
    auto count = static_cast<std::uint32_t>(size);
    std::vector<std::pair<int, int>> pairs(std::size_t{4} * count);
    for (auto& [a, b] : pairs) {
        a = static_cast<int>(random.next() % count);
        b = static_cast<int>(random.next() % count);
    }
    UnionFind sets(size);
    return measure([&] {
        std::int64_t merged = 0, connected = 0;
        bool merge = true;
        for (auto [a, b] : pairs) {
            if (merge)
                merged += sets.unite(a, b);
            else
                connected += sets.find(a) == sets.find(b);
            merge = !merge;
        }
        return (merged << 32) | connected;
    });
}

Measurement run_sieve(int size) {
    auto [flags, elapsed] = measure([size] { return build_sieve(size); });
    return {std::ranges::count(flags, 1), elapsed};
}

Measurement run_kmp(int size) {
    auto length = static_cast<std::size_t>(size);
    std::string text(length, 'a'), pattern(std::clamp(length / 1024, std::size_t{1}, length), 'a');
    text.back() = pattern.back() = 'b';
    return measure([&] { return strStr(text, pattern); });
}

struct RangeOperation {
    int index, value, left, right;
};

template<class Tree, class Query>
Measurement run_range_workload(int size, Query query) {
    Random random;
    auto count = static_cast<std::uint32_t>(size);
    std::vector<int> values(count);
    for (int& value : values)
        value = static_cast<int>(random.next() % 1'000);
    std::vector<RangeOperation> operations(std::size_t{4} * count);
    for (auto& [index, value, left, right] : operations) {
        index = static_cast<int>(random.next() % count);
        value = static_cast<int>(random.next() % 1'000);
        left = static_cast<int>(random.next() % count);
        right = static_cast<int>(random.next() % count);
        if (left > right)
            std::swap(left, right);
    }
    Tree tree(values);
    return measure([&] {
        std::int64_t checksum = 0;
        for (auto [index, value, left, right] : operations) {
            tree.update(index, value);
            checksum += query(tree, left, right);
        }
        return checksum;
    });
}

}

int main(int argc, char** argv) {
    if (argc != 3)
        return 2;

    std::string_view input_size = argv[2];
    int size = 0;
    auto limit = input_size.data() + std::size(input_size);
    auto [end, error] = std::from_chars(input_size.data(), limit, size);
    if (error != std::errc{} or end != limit or size <= 0)
        return 2;

    std::string_view workload = argv[1];
    Measurement result;
    if (workload == "kadane")
        result = run_kadane(size);
    else if (workload == "union-find")
        result = run_union_find(size);
    else if (workload == "sieve")
        result = run_sieve(size);
    else if (workload == "kmp")
        result = run_kmp(size);
    else if (workload == "fenwick" and size <= 2'000'000)
        result = run_range_workload<Fenwick>(size,
            [](Fenwick& tree, int left, int right) { return tree.sumRange(left, right); });
    else if (workload == "segment-tree" and size <= 2'000'000)
        result = run_range_workload<SegmentTree>(size,
            [](SegmentTree& tree, int left, int right) { return tree.query(left, right); });
    else
        return 2;

    std::cout << result.second << ' ' << result.first << '\n';
}
