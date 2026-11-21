#[path = "../templates/trees/fenwick_sum.rs"]
mod fenwick;
#[path = "../templates/arrays/kadane.rs"]
mod kadane;
#[path = "../templates/strings/kmp.rs"]
mod kmp;
#[path = "../templates/maths/prime_sieve.rs"]
mod prime_sieve;
#[path = "../templates/trees/segment_sum.rs"]
mod segment_sum;
#[path = "../templates/graphs/union_find.rs"]
mod union_find;

use std::{hint::black_box, process::ExitCode, time::Instant};

struct Random(u32);

impl Random {
    fn new() -> Self {
        Self(240513)
    }

    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        self.0
    }
}

fn measure<T>(run: impl FnOnce() -> T) -> (T, u128) {
    let start = Instant::now();
    let result = black_box(run());
    (result, start.elapsed().as_nanos())
}

fn run_kadane(size: usize) -> (i64, u128) {
    let mut random = Random::new();
    let values: Vec<i64> = (0..size)
        .map(|_| i64::from(random.next() % 101) - 50)
        .collect();
    measure(|| kadane::kadane(&values).unwrap())
}

fn run_union_find(size: usize) -> (i64, u128) {
    let mut random = Random::new();
    let pairs: Vec<_> = (0..4 * size)
        .map(|_| (random.next() as usize % size, random.next() as usize % size))
        .collect();
    let mut sets = union_find::UnionFind::new(size);
    let (checksum, elapsed) = measure(|| {
        let (mut merged, mut connected) = (0_i64, 0_i64);
        for (index, &(a, b)) in pairs.iter().enumerate() {
            if index.is_multiple_of(2) {
                merged += i64::from(sets.unite(a, b));
            } else {
                connected += i64::from(sets.find(a) == sets.find(b));
            }
        }
        (merged << 32) | connected
    });
    assert_eq!(sets.groups(), size - (checksum >> 32) as usize);
    (checksum, elapsed)
}

fn run_sieve(size: usize) -> (i64, u128) {
    let (flags, elapsed) = measure(|| prime_sieve::build_sieve(size));
    (flags.into_iter().map(i64::from).sum(), elapsed)
}

fn run_kmp(size: usize) -> (i64, u128) {
    let mut text = vec![b'a'; size];
    let mut pattern = vec![b'a'; (size / 1024).clamp(1, size)];
    *text.last_mut().unwrap() = b'b';
    *pattern.last_mut().unwrap() = b'b';
    measure(|| kmp::str_str(&text, &pattern).map_or(-1, |index| index as i64))
}

fn run_range_workload<T>(
    size: usize,
    make_tree: impl FnOnce(&[i64]) -> T,
    update: impl Fn(&mut T, usize, i64),
    query: impl Fn(&T, usize, usize) -> i64,
) -> (i64, u128) {
    let mut random = Random::new();
    let values: Vec<i64> = (0..size)
        .map(|_| i64::from(random.next() % 1_000))
        .collect();
    let operations: Vec<_> = (0..4 * size)
        .map(|_| {
            let index = random.next() as usize % size;
            let value = i64::from(random.next() % 1_000);
            let a = random.next() as usize % size;
            let b = random.next() as usize % size;
            (index, value, a.min(b), a.max(b))
        })
        .collect();
    let mut tree = make_tree(&values);
    measure(|| {
        let mut checksum = 0;
        for &(index, value, left, right) in &operations {
            update(&mut tree, index, value);
            checksum += query(&tree, left, right);
        }
        checksum
    })
}

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let (Some(workload), Some(size), None) = (args.next(), args.next(), args.next()) else {
        return ExitCode::from(2);
    };
    let Ok(size) = size.parse::<usize>() else {
        return ExitCode::from(2);
    };
    if size == 0 || size > i32::MAX as usize {
        return ExitCode::from(2);
    }
    let (checksum, elapsed) = match workload.as_str() {
        "kadane" => run_kadane(size),
        "union-find" => run_union_find(size),
        "sieve" => run_sieve(size),
        "kmp" => run_kmp(size),
        "fenwick" if size <= 2_000_000 => run_range_workload(
            size,
            fenwick::Fenwick::new,
            fenwick::Fenwick::update,
            fenwick::Fenwick::sum_range,
        ),
        "segment-tree" if size <= 2_000_000 => run_range_workload(
            size,
            segment_sum::SegmentTree::new,
            segment_sum::SegmentTree::update,
            segment_sum::SegmentTree::query,
        ),
        _ => return ExitCode::from(2),
    };
    println!("{elapsed} {checksum}");
    ExitCode::SUCCESS
}
