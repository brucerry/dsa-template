//! Rabin–Karp counts matches using two rolling hashes with a shared random base.
//! Hash equality can have collisions. Substring ranges use zero-based byte offsets.
//! Example problem at <https://cses.fi/problemset/task/1753>.

use std::collections::hash_map::RandomState;
use std::hash::BuildHasher;
use std::ops::Range;
use std::sync::LazyLock;

const MODS: [u64; 2] = [1_000_000_007, 1_000_000_009];
static BASE: LazyLock<u64> =
    LazyLock::new(|| 257 + RandomState::new().hash_one(0_u8) % (MODS[0] - 257));

pub struct Hash {
    prefix: Vec<[u64; 2]>,
    powers: Vec<[u64; 2]>,
}

impl Hash {
    pub fn new(text: &[u8]) -> Self {
        let base = *BASE;
        let mut prefix = vec![[0; 2]; text.len() + 1];
        let mut powers = vec![[1; 2]; text.len() + 1];
        for (i, &byte) in text.iter().enumerate() {
            for (j, modulus) in MODS.into_iter().enumerate() {
                prefix[i + 1][j] = (prefix[i][j] * base + u64::from(byte)) % modulus;
                powers[i + 1][j] = powers[i][j] * base % modulus;
            }
        }
        Self { prefix, powers }
    }

    pub fn hash(&self, range: Range<usize>) -> [u64; 2] {
        assert!(range.start <= range.end);
        let length = range.end - range.start;
        std::array::from_fn(|i| {
            let modulus = MODS[i];
            let left = self.prefix[range.start][i] * self.powers[length][i] % modulus;
            (self.prefix[range.end][i] + modulus - left) % modulus
        })
    }
}

pub fn count_occurs(text: &[u8], pattern: &[u8]) -> usize {
    let length = pattern.len();
    if length == 0 {
        return text.len() + 1;
    }
    if length > text.len() {
        return 0;
    }
    let text_hash = Hash::new(text);
    let target = Hash::new(pattern).hash(0..length);
    (0..=text.len() - length)
        .filter(|&i| text_hash.hash(i..i + length) == target)
        .count()
}
