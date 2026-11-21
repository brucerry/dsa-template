//! Union-Find tracks sets of elements numbered from 0 to n - 1.
//! Path halving and union by size give nearly constant amortized operations.

pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    groups: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            groups: n,
        }
    }

    pub fn groups(&self) -> usize {
        self.groups
    }

    pub fn find(&mut self, mut node: usize) -> usize {
        while self.parent[node] != node {
            let grandparent = self.parent[self.parent[node]];
            self.parent[node] = grandparent;
            node = grandparent;
        }
        node
    }

    pub fn unite(&mut self, a: usize, b: usize) -> bool {
        let mut root_a = self.find(a);
        let mut root_b = self.find(b);
        if root_a == root_b {
            return false;
        }
        if self.size[root_a] < self.size[root_b] {
            std::mem::swap(&mut root_a, &mut root_b);
        }
        self.parent[root_b] = root_a;
        self.size[root_a] += self.size[root_b];
        self.groups -= 1;
        true
    }
}
