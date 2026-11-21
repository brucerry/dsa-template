//! A prefix trie stores words made of lowercase ASCII letters a through z.
//! Words with a common prefix share the same path. Operations take O(word length).

#[derive(Default)]
pub struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        for byte in word.bytes() {
            let index = usize::from(byte - b'a');
            node = node.children[index].get_or_insert_default();
        }
        node.is_end = true;
    }

    pub fn search_prefix(&self, prefix: &str) -> Option<&Self> {
        let mut node = self;
        for byte in prefix.bytes() {
            let index = usize::from(byte - b'a');
            node = node.children[index].as_deref()?;
        }
        Some(node)
    }

    pub fn search(&self, word: &str) -> bool {
        self.search_prefix(word).is_some_and(|node| node.is_end)
    }
}
