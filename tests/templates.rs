use dsa_template::{
    arrays::kadane::kadane,
    graphs::union_find::UnionFind,
    linked_list::lru::LruCache,
    maths::{pow_x_n, pow_x_n_mod, pow_x_n_mod_recursive, prime_sieve::build_sieve},
    strings::{kmp, kmp_advanced, string_hashing, z_function},
    trees::{
        diameter_of_undirected_tree::diameter, fenwick_sum::Fenwick, segment_search, segment_sum,
        trie, trie_binary,
    },
};

#[test]
fn kadane_handles_empty_and_all_negative_inputs() {
    assert_eq!(kadane(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), Some(6));
    assert_eq!(kadane(&[i64::MIN, -8, -3, -6]), Some(-3));
    assert_eq!(kadane(&[]), None);
}

#[test]
fn union_find_tracks_successful_merges() {
    let mut sets = UnionFind::new(5);
    assert!(sets.unite(0, 1));
    assert!(sets.unite(2, 3));
    assert!(sets.unite(1, 3));
    assert!(!sets.unite(0, 2));
    assert_eq!(sets.find(0), sets.find(3));
    assert_ne!(sets.find(0), sets.find(4));
    assert_eq!(sets.groups(), 2);
}

#[test]
fn lru_matches_a_simple_reference_cache() {
    for capacity in 0..=5 {
        let mut cache = LruCache::new(capacity);
        let mut reference = Vec::<(i32, i32)>::new();
        let mut state = 1_u32;
        for value in 0..500 {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let key = (state >> 24) as i32 % 7;
            let found = reference.iter().position(|&(stored, _)| stored == key);
            if state & 4 == 0 {
                let expected = found.map(|index| {
                    let entry = reference.remove(index);
                    reference.push(entry);
                    entry.1
                });
                assert_eq!(cache.get(key), expected);
            } else {
                cache.put(key, value);
                if let Some(index) = found {
                    reference.remove(index);
                }
                reference.push((key, value));
                if reference.len() > capacity {
                    reference.remove(0);
                }
            }
        }
    }
}

#[test]
fn floating_power_handles_signed_exponents() {
    assert_eq!(pow_x_n::pow(2.0, 10), 1024.0);
    assert_eq!(pow_x_n::pow(2.0, -3), 0.125);
    assert_eq!(pow_x_n::pow(-2.0, 3), -8.0);
    assert_eq!(pow_x_n::pow(1.0, i32::MIN), 1.0);
    assert_eq!(pow_x_n::pow(0.0, 0), 1.0);
}

#[test]
fn modular_powers_handle_custom_moduli_and_wide_products() {
    for power in [pow_x_n_mod::pow, pow_x_n_mod_recursive::pow] {
        assert_eq!(power(2, 30, 1_000_000_007), 73_741_817);
        assert_eq!(power(12, 34, 97), 47);
        assert_eq!(power(5, 0, 1), 0);
        assert_eq!(power(u64::MAX - 1, 2, u64::MAX), 1);
        assert_eq!(power(u64::MAX - 1, 3, u64::MAX), u64::MAX - 1);
    }
}

#[test]
fn sieve_marks_primes_through_the_limit() {
    assert_eq!(build_sieve(0), [false]);
    assert_eq!(build_sieve(1), [false, false]);
    assert_eq!(
        build_sieve(10),
        [
            false, false, true, true, false, true, false, true, false, false, false
        ],
    );
}

#[test]
fn string_matchers_agree_with_byte_window_search() {
    let texts = [b"".as_slice(), b"aaaa", b"ababa", b"#\0#\xff#"];
    let patterns = [
        b"".as_slice(),
        b"a",
        b"aa",
        b"aba",
        b"ababa#",
        b"#",
        b"\xff#",
    ];
    for text in texts {
        for pattern in patterns {
            let expected: Vec<usize> = if pattern.is_empty() {
                (0..=text.len()).collect()
            } else {
                text.windows(pattern.len())
                    .enumerate()
                    .filter_map(|(i, window)| (window == pattern).then_some(i))
                    .collect()
            };
            assert_eq!(kmp::str_str(text, pattern), expected.first().copied());
            assert_eq!(kmp_advanced::kmp(text, pattern), expected);
            assert_eq!(string_hashing::count_occurs(text, pattern), expected.len());
            assert_eq!(z_function::count_occurs(text, pattern), expected.len());
        }
    }
    assert_eq!(kmp::prefix_function(b"ababaca"), [0, 0, 1, 2, 3, 0, 1]);
    assert_eq!(z_function::z_func(b"ababa"), [0, 0, 3, 0, 1]);
    let hash = string_hashing::Hash::new(b"abab");
    assert_eq!(hash.hash(0..2), hash.hash(2..4));
}

#[test]
fn diameter_counts_edges() {
    assert_eq!(diameter(&[]), 0);
    assert_eq!(diameter(&[(0, 1), (1, 2), (2, 3)]), 3);
    assert_eq!(diameter(&[(0, 1), (0, 2), (0, 3)]), 2);
}

#[test]
fn range_sum_trees_agree_with_direct_sums_after_updates() {
    let mut values = [1, -2, 3, 4, 5];
    let mut fenwick = Fenwick::new(&values);
    let mut segment = segment_sum::SegmentTree::new(&values);
    for (index, value) in [(0, 9), (4, -4), (2, 10)] {
        values[index] = value;
        fenwick.update(index, value);
        segment.update(index, value);
        for left in 0..values.len() {
            for right in left..values.len() {
                let expected = values[left..=right].iter().sum();
                assert_eq!(fenwick.sum_range(left, right), expected);
                assert_eq!(segment.query(left, right), expected);
            }
        }
    }
}

#[test]
fn segment_search_finds_and_reserves_the_leftmost_match() {
    let mut tree = segment_search::SegmentTree::new(&[1, 5, 3]);
    assert_eq!(tree.query(3), Some(1));
    tree.update(1, -1);
    assert_eq!(tree.query(3), Some(2));
    assert_eq!(tree.query(8), None);
    assert_eq!(segment_search::SegmentTree::new(&[]).query(i32::MIN), None);
    assert_eq!(
        segment_search::search(&[3, 3, i32::MIN, i32::MIN], &[i32::MIN, 5, 3]),
        [Some(1), Some(2), Some(0), None],
    );
}

#[test]
fn prefix_trie_distinguishes_words_from_prefixes() {
    let mut trie = trie::Trie::new();
    trie.insert("cat");
    trie.insert("car");
    assert!(trie.search("cat") && trie.search("car"));
    assert!(trie.search_prefix("ca").is_some());
    assert!(!trie.search("ca") && !trie.search("dog"));
    trie.insert("");
    assert!(trie.search(""));
}

#[test]
fn binary_trie_minimizes_xor_across_all_32_bits() {
    let mut trie = trie_binary::Trie::new();
    assert_eq!(trie.find_possible(1), None);
    let keys = [0, 5, 11, 1 << 31, u32::MAX];
    for key in keys {
        trie.insert(key);
    }
    for target in [0, 4, 7, 15, 1 << 31, u32::MAX - 1] {
        let expected = keys.into_iter().min_by_key(|key| key ^ target);
        assert_eq!(trie.find_possible(target), expected);
    }
}
