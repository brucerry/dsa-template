/*
 * Finds the longest path in an undirected tree, measured in edges.
 * Each edge contains two endpoints. With n - 1 edges, nodes are numbered 0 through n - 1.
 */

#include <algorithm>
#include <cstddef>
#include <iterator>
#include <queue>
#include <ranges>
#include <vector>

int diameter(std::vector<std::vector<int>>& edges) {
    std::size_t n = std::size(edges) + 1;
    std::vector<std::vector<int>> al(n);
    std::vector<int> cnt(n);
    for (auto& e : edges) {
        std::size_t a = static_cast<std::size_t>(e[0]);
        std::size_t b = static_cast<std::size_t>(e[1]);
        al[a].push_back(e[1]);
        al[b].push_back(e[0]);
        ++cnt[a];
        ++cnt[b];
    }
    std::queue<std::size_t> que;
    for (std::size_t i : std::views::iota(std::size_t{0}, n)) {
        if (cnt[i] == 1)
            que.push(i);
    }
    std::vector<int> depth(n), seen(n);
    int res = 0;
    while (!que.empty()) {
        std::size_t cur = que.front();
        que.pop();
        seen[cur] = 1;
        for (int neighbor : al[cur]) {
            std::size_t next = static_cast<std::size_t>(neighbor);
            if (--cnt[next] == 1)
                que.push(next);
            if (!seen[next]) {
                res = std::max(res, depth[cur] + depth[next] + 1);
                depth[next] = std::max(depth[next], depth[cur] + 1);
            }
        }
    }
    return res;
}
