//! Standalone Rust templates for classic data structures and algorithms.

pub mod arrays {
    pub mod kadane;
}

pub mod graphs {
    pub mod union_find;
}

pub mod linked_list {
    pub mod lru;
}

pub mod maths {
    pub mod pow_x_n;
    pub mod pow_x_n_mod;
    pub mod pow_x_n_mod_recursive;
    pub mod prime_sieve;
}

pub mod strings {
    pub mod kmp;
    pub mod kmp_advanced;
    pub mod string_hashing;
    pub mod z_function;
}

pub mod trees {
    pub mod diameter_of_undirected_tree;
    pub mod fenwick_sum;
    pub mod segment_search;
    pub mod segment_sum;
    pub mod trie;
    pub mod trie_binary;
}
