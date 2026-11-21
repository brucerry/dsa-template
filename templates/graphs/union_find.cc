/*
 * Union-Find tracks disjoint sets of elements numbered 0 through n - 1.
 * Each element starts in its own set, and group counts the remaining sets.
 * find returns a set's root with path compression. unite returns true when sets merge.
 */

#include <algorithm>
#include <cstddef>
#include <numeric>
#include <vector>

struct UnionFind {
    int group;
    std::vector<int> parent;

    UnionFind(int n) : group(n), parent(static_cast<std::size_t>(n)) {
        std::ranges::iota(parent, 0);
    }

    bool unite(int node1, int node2) {
        int par1 = find(node1);
        int par2 = find(node2);
        if (par1 == par2)
            return false;
        parent[static_cast<std::size_t>(par1)] = par2;
        --group;
        return true;
    }

    int find(int node) {
        int& next = parent[static_cast<std::size_t>(node)];
        return next == node ? node : next = find(next);
    }
};
