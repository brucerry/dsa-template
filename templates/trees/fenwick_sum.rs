//! A Fenwick tree supports point assignments and inclusive range sums.
//! Construction takes O(n) time. Updates and queries take O(log n) time.
//! Use zero-based indices and values whose sums and update differences fit in `i64`.

pub struct Fenwick {
    values: Vec<i64>,
    tree: Vec<i64>,
}

impl Fenwick {
    pub fn new(values: &[i64]) -> Self {
        let mut tree = vec![0; values.len() + 1];
        for (i, &value) in values.iter().enumerate() {
            let index = i + 1;
            tree[index] += value;
            let parent = index + index.isolate_lowest_one();
            if parent < tree.len() {
                tree[parent] += tree[index];
            }
        }
        Self {
            values: values.to_vec(),
            tree,
        }
    }

    pub fn update(&mut self, index: usize, value: i64) {
        let difference = value - self.values[index];
        self.values[index] = value;
        let mut index = index + 1;
        while index < self.tree.len() {
            self.tree[index] += difference;
            index += index.isolate_lowest_one();
        }
    }

    pub fn sum_range(&self, left: usize, right: usize) -> i64 {
        assert!(left <= right && right < self.values.len());
        self.prefix_sum(right + 1) - self.prefix_sum(left)
    }

    fn prefix_sum(&self, mut count: usize) -> i64 {
        let mut sum = 0;
        while count != 0 {
            sum += self.tree[count];
            count -= count.isolate_lowest_one();
        }
        sum
    }
}
