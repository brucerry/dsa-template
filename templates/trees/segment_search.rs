//! A max segment tree finds the first index whose value meets a threshold.
//! Point updates and queries take O(log n) time. A query with no match returns `None`.
//! The `search` example reserves each matched index so it cannot be reused.

pub struct SegmentTree {
    len: usize,
    width: usize,
    tree: Vec<Option<i32>>,
}

impl SegmentTree {
    pub fn new(values: &[i32]) -> Self {
        let width = values.len().next_power_of_two();
        let mut tree = vec![None; 2 * width];
        for (leaf, &value) in tree[width..].iter_mut().zip(values) {
            *leaf = Some(value);
        }
        for node in (1..width).rev() {
            tree[node] = tree[2 * node].max(tree[2 * node + 1]);
        }
        Self {
            len: values.len(),
            width,
            tree,
        }
    }

    pub fn update(&mut self, index: usize, value: i32) {
        self.set(index, Some(value));
    }

    pub fn query(&self, threshold: i32) -> Option<usize> {
        if self.tree[1]? < threshold {
            return None;
        }
        let mut node = 1;
        while node < self.width {
            node *= 2;
            if !self.tree[node].is_some_and(|value| value >= threshold) {
                node += 1;
            }
        }
        Some(node - self.width)
    }

    fn set(&mut self, index: usize, value: Option<i32>) {
        assert!(index < self.len);
        let mut node = index + self.width;
        self.tree[node] = value;
        while node > 1 {
            node /= 2;
            self.tree[node] = self.tree[2 * node].max(self.tree[2 * node + 1]);
        }
    }
}

pub fn search(thresholds: &[i32], values: &[i32]) -> Vec<Option<usize>> {
    let mut tree = SegmentTree::new(values);
    thresholds
        .iter()
        .map(|&threshold| {
            let index = tree.query(threshold)?;
            tree.set(index, None);
            Some(index)
        })
        .collect()
}
