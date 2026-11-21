//! A binary trie stores unsigned 32-bit keys from most to least significant bit.
//! `find_possible` returns a stored key that minimizes XOR with the target.
//! An empty trie returns `None`.

#[derive(Default)]
pub struct Trie {
    children: [Option<Box<Trie>>; 2],
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, key: u32) {
        let mut node = self;
        for shift in (0..u32::BITS).rev() {
            let bit = usize::from(key & (1 << shift) != 0);
            node = node.children[bit].get_or_insert_default();
        }
    }

    pub fn find_possible(&self, target: u32) -> Option<u32> {
        let mut node = self;
        let mut result = 0;
        for shift in (0..u32::BITS).rev() {
            let mask = 1 << shift;
            let preferred = usize::from(target & mask != 0);
            let bit = if node.children[preferred].is_some() {
                preferred
            } else {
                preferred ^ 1
            };
            node = node.children[bit].as_deref()?;
            if bit != 0 {
                result |= mask;
            }
        }
        Some(result)
    }
}
