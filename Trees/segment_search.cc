/*
 * A max segment tree finds the first index whose value meets or exceeds a threshold.
 * Build from a nonempty array. Queries and point updates take O(log n) time.
 * query returns a zero-based index or -1 when no element qualifies.
 *
 * The search example reserves each matched index by setting its tree value to -1.
 * Use nonnegative thresholds in that example.
 */

#include <algorithm>
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
            tree[node] = std::max(tree[2 * node + 1], tree[2 * node + 2]);
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
            tree[node] = std::max(tree[2 * node + 1], tree[2 * node + 2]);
        }
    }

    int query(std::size_t node, int start, int end, int value) {
        if (start == end) {
            return (tree[node] >= value) ? start : -1;
        }
        int mid = std::midpoint(start, end);
        if (tree[2 * node + 1] >= value) {
            return query(2 * node + 1, start, mid, value);
        }
        return query(2 * node + 2, mid + 1, end, value);
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

    int query(int value) {
        return query(0, 0, n - 1, value);
    }
};

int search(std::vector<int>& nums1, std::vector<int>& nums2) {
    SegmentTree segTree(nums2);

    for (int num : nums1) {
        int index = segTree.query(num);
        if (index == -1) {
            std::println("Cannot find an unoccupied index j from nums2 where {} <= nums2[j]", num);
        } else {
            int value = nums2[static_cast<std::size_t>(index)];
            std::print("Index {} of nums2 = {} >= {}, mark it as occupied", index, value, num);
            segTree.update(index, -1);
        }
    }

    return 0;
}
