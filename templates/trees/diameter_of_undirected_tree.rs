//! Two breadth-first searches find a tree's diameter in O(n) time and space.
//! The n - 1 edges describe a connected tree with vertices numbered from 0 to n - 1.

use std::collections::VecDeque;

pub fn diameter(edges: &[(usize, usize)]) -> usize {
    let mut adjacent = vec![Vec::new(); edges.len() + 1];
    for &(a, b) in edges {
        adjacent[a].push(b);
        adjacent[b].push(a);
    }
    let (endpoint, _) = farthest(&adjacent, 0);
    farthest(&adjacent, endpoint).1
}

fn farthest(adjacent: &[Vec<usize>], start: usize) -> (usize, usize) {
    let mut queue = VecDeque::from([(start, start, 0)]);
    let mut result = (start, 0);
    while let Some((node, parent, distance)) = queue.pop_front() {
        result = (node, distance);
        for &next in &adjacent[node] {
            if next != parent {
                queue.push_back((next, node, distance + 1));
            }
        }
    }
    result
}
