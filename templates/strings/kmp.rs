//! Knuth–Morris–Pratt finds the first match in linear time.
//! Returns a byte offset, `None` for no match, or `Some(0)` for an empty pattern.

pub fn prefix_function(pattern: &[u8]) -> Vec<usize> {
    let mut prefix = vec![0; pattern.len()];
    let mut matched = 0;
    for (i, &byte) in pattern.iter().enumerate().skip(1) {
        while matched != 0 && byte != pattern[matched] {
            matched = prefix[matched - 1];
        }
        matched += usize::from(byte == pattern[matched]);
        prefix[i] = matched;
    }
    prefix
}

pub fn str_str(text: &[u8], pattern: &[u8]) -> Option<usize> {
    if pattern.is_empty() {
        return Some(0);
    }
    let prefix = prefix_function(pattern);
    let mut matched = 0;
    for (i, &byte) in text.iter().enumerate() {
        while matched != 0 && byte != pattern[matched] {
            matched = prefix[matched - 1];
        }
        matched += usize::from(byte == pattern[matched]);
        if matched == pattern.len() {
            return Some(i + 1 - matched);
        }
    }
    None
}
