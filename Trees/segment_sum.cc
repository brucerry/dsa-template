/*
 * A sum segment tree supports point assignments and inclusive range sums.
 * Build from a nonempty array. Updates and queries use zero-based indices
 * and take O(log n) time.
 *
 * The range_sum example prints 9 for indices [1, 3], then 16 after changing index 2 to 10.
 */

#include <cstddef>
#include <iterator>
#include <numeric>
#include <print>
#include <vector>

class SegmentTree {
private:
    std::vector<int> tree;
    int n;

    void build(std::vector<int>& arr, std::size_t node, int start, int end) {
        if (start == end) {
            tree[node] = arr[static_cast<std::size_t>(start)];
        } else {
            int mid = std::midpoint(start, end);
            build(arr, 2 * node + 1, start, mid);
            build(arr, 2 * node + 2, mid + 1, end);
            tree[node] = tree[2 * node + 1] + tree[2 * node + 2];
        }
    }

    void update(std::size_t node, int start, int end, int idx, int value) {
        if (start == end) {
            tree[node] = value;
        } else {
            int mid = std::midpoint(start, end);
            if (idx <= mid) {
                update(2 * node + 1, start, mid, idx, value);
            } else {
                update(2 * node + 2, mid + 1, end, idx, value);
            }
            tree[node] = tree[2 * node + 1] + tree[2 * node + 2];
        }
    }

    int query(std::size_t node, int start, int end, int left, int right) {
        if (right < start or end < left) {
            return 0;
        }
        if (left <= start and end <= right) {
            return tree[node];
        }
        int mid = std::midpoint(start, end);
        int leftQuery = query(2 * node + 1, start, mid, left, right);
        int rightQuery = query(2 * node + 2, mid + 1, end, left, right);
        return leftQuery + rightQuery;
    }

public:
    SegmentTree(std::vector<int>& arr) {
        n = static_cast<int>(std::size(arr));
        tree.resize(4 * std::size(arr), 0);
        build(arr, 0, 0, n - 1);
    }

    void update(int idx, int value) {
        update(0, 0, n - 1, idx, value);
    }

    int query(int left, int right) {
        return query(0, 0, n - 1, left, right);
    }
};

int range_sum() {
    std::vector<int> arr = {1, 2, 3, 4, 5};
    SegmentTree segTree(arr);
    
    std::println("Sum of range [1,3]: {}", segTree.query(1, 3));
    segTree.update(2, 10);
    std::println("Sum of range [1,3] after update: {}", segTree.query(1, 3));
    
    return 0;
}
