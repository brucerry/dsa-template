//! The Z-function records the prefix match length at every byte offset.
//! The first entry is zero. Pattern counting includes overlapping occurrences.
//! Example problem at <https://cses.fi/problemset/task/1753>.

pub fn z_func(text: &[u8]) -> Vec<usize> {
    z_values(text)
}

pub fn count_occurs(text: &[u8], pattern: &[u8]) -> usize {
    if pattern.is_empty() {
        return text.len() + 1;
    }
    if pattern.len() > text.len() {
        return 0;
    }
    let mut combined = Vec::with_capacity(pattern.len() + 1 + text.len());
    combined.extend(pattern.iter().copied().map(u16::from));
    combined.push(256);
    combined.extend(text.iter().copied().map(u16::from));
    z_values(&combined)[pattern.len() + 1..]
        .iter()
        .filter(|&&length| length == pattern.len())
        .count()
}

fn z_values<T: PartialEq>(text: &[T]) -> Vec<usize> {
    let mut matches = vec![0; text.len()];
    let (mut left, mut right) = (0, 0);
    for i in 1..text.len() {
        if i < right {
            matches[i] = matches[i - left].min(right - i);
        }
        while i + matches[i] < text.len() && text[matches[i]] == text[i + matches[i]] {
            matches[i] += 1;
        }
        if i + matches[i] > right {
            left = i;
            right = i + matches[i];
        }
    }
    matches
}
