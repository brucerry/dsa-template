//! An LRU cache with expected O(1) get and put operations.
//! A hash map locates entries. A doubly linked list of vector indices tracks recency
//! and reuses the oldest entry's slot when the cache is full.

use std::collections::HashMap;

struct Entry {
    key: i32,
    value: i32,
    prev: Option<usize>,
    next: Option<usize>,
}

pub struct LruCache {
    capacity: usize,
    indices: HashMap<i32, usize>,
    entries: Vec<Entry>,
    oldest: Option<usize>,
    newest: Option<usize>,
}

impl LruCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            indices: HashMap::new(),
            entries: Vec::new(),
            oldest: None,
            newest: None,
        }
    }

    pub fn get(&mut self, key: i32) -> Option<i32> {
        let &index = self.indices.get(&key)?;
        self.move_to_newest(index);
        Some(self.entries[index].value)
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&index) = self.indices.get(&key) {
            self.entries[index].value = value;
            self.move_to_newest(index);
            return;
        }
        if self.capacity == 0 {
            return;
        }

        let entry = Entry {
            key,
            value,
            prev: None,
            next: None,
        };
        let index = if self.entries.len() < self.capacity {
            let index = self.entries.len();
            self.entries.push(entry);
            index
        } else {
            let index = self.oldest.expect("a full cache has an oldest entry");
            self.indices.remove(&self.entries[index].key);
            self.detach(index);
            self.entries[index] = entry;
            index
        };
        self.indices.insert(key, index);
        self.append(index);
    }

    fn move_to_newest(&mut self, index: usize) {
        if self.newest != Some(index) {
            self.detach(index);
            self.append(index);
        }
    }

    fn detach(&mut self, index: usize) {
        let prev = self.entries[index].prev;
        let next = self.entries[index].next;
        if let Some(prev) = prev {
            self.entries[prev].next = next;
        } else {
            self.oldest = next;
        }
        if let Some(next) = next {
            self.entries[next].prev = prev;
        } else {
            self.newest = prev;
        }
    }

    fn append(&mut self, index: usize) {
        self.entries[index].prev = self.newest;
        self.entries[index].next = None;
        if let Some(newest) = self.newest {
            self.entries[newest].next = Some(index);
        } else {
            self.oldest = Some(index);
        }
        self.newest = Some(index);
    }
}
