//! Knuth–Morris–Pratt finds every match, including overlaps, in linear time.
//! Returns byte offsets. An empty pattern matches at every byte boundary.

pub fn kmp(text: &[u8], pattern: &[u8]) -> Vec<usize> {
    if pattern.is_empty() {
        return (0..=text.len()).collect();
    }
    let mut prefix = vec![0; pattern.len()];
    let mut matched = 0;
    for (i, &byte) in pattern.iter().enumerate().skip(1) {
        while matched != 0 && byte != pattern[matched] {
            matched = prefix[matched - 1];
        }
        matched += usize::from(byte == pattern[matched]);
        prefix[i] = matched;
    }

    let mut offsets = Vec::new();
    let mut matched = 0;
    for (i, &byte) in text.iter().enumerate() {
        while matched != 0 && byte != pattern[matched] {
            matched = prefix[matched - 1];
        }
        matched += usize::from(byte == pattern[matched]);
        if matched == pattern.len() {
            offsets.push(i + 1 - matched);
            matched = prefix[matched - 1];
        }
    }
    offsets
}
