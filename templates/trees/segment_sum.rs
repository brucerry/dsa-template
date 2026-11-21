//! A sum segment tree supports point assignments and inclusive range sums.
//! Use zero-based indices within the input and sums that fit in `i64`.
//! Updates and queries take O(log n) time.

pub struct SegmentTree {
    len: usize,
    width: usize,
    tree: Vec<i64>,
}

impl SegmentTree {
    pub fn new(values: &[i64]) -> Self {
        let width = values.len().next_power_of_two();
        let mut tree = vec![0; 2 * width];
        tree[width..width + values.len()].copy_from_slice(values);
        for node in (1..width).rev() {
            tree[node] = tree[2 * node] + tree[2 * node + 1];
        }
        Self {
            len: values.len(),
            width,
            tree,
        }
    }

    pub fn update(&mut self, index: usize, value: i64) {
        assert!(index < self.len);
        let mut node = index + self.width;
        self.tree[node] = value;
        while node > 1 {
            node /= 2;
            self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
        }
    }

    pub fn query(&self, left: usize, right: usize) -> i64 {
        assert!(left <= right && right < self.len);
        let mut start = left + self.width;
        let mut end = right + self.width + 1;
        let mut sum = 0;
        while start < end {
            if !start.is_multiple_of(2) {
                sum += self.tree[start];
                start += 1;
            }
            if !end.is_multiple_of(2) {
                end -= 1;
                sum += self.tree[end];
            }
            start /= 2;
            end /= 2;
        }
        sum
    }
}
