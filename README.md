# DSA Template

Classic data structures and algorithms in **C++23** and **Rust 2024**. Each template has a `.cc` and a `.rs` file with the same name in [`templates/`](templates/).

[Try an example](#try-an-example) · [Browse templates](#browse-templates) · [Build and test](#build-and-test)

## Try an example

Open either language to try Kadane's algorithm, or open both to compare. The maximum subarray is `[4, -1, 2, 1]`, with sum `6`.

<details>
<summary>C++23 example</summary>

Save as `example.cc` in the repository root.

```cpp
#include <cassert>
#include "templates/arrays/kadane.cc"

int main() {
    std::vector<int> numbers{-2, 1, -3, 4, -1, 2, 1, -5, 4};
    assert(kadane(numbers) == 6);
}
```

```sh
c++ -std=c++23 example.cc -o /tmp/dsa-example && /tmp/dsa-example
```

</details>

<details>
<summary>Rust example</summary>

Save as `example.rs` in the repository root.

```rust
#[path = "templates/arrays/kadane.rs"]
mod kadane;

fn main() {
    let numbers = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    assert_eq!(kadane::kadane(&numbers), Some(6));
}
```

```sh
rustc --edition=2024 example.rs -o /tmp/dsa-example && /tmp/dsa-example
```

</details>

## Browse templates

Expand a category and choose **C++23** or **Rust** for the same algorithm.

<details open>
<summary>Arrays</summary>

| Template | C++23 | Rust |
| --- | --- | --- |
| Kadane's maximum subarray | [C++23](templates/arrays/kadane.cc) | [Rust](templates/arrays/kadane.rs) |

</details>

<details>
<summary>Graphs</summary>

| Template | C++23 | Rust |
| --- | --- | --- |
| Union-Find | [C++23](templates/graphs/union_find.cc) | [Rust](templates/graphs/union_find.rs) |

</details>

<details>
<summary>Linked lists</summary>

| Template | C++23 | Rust |
| --- | --- | --- |
| LRU cache | [C++23](templates/linked_list/lru.cc) | [Rust](templates/linked_list/lru.rs) |

</details>

<details>
<summary>Maths</summary>

| Template | C++23 | Rust |
| --- | --- | --- |
| Binary exponentiation | [C++23](templates/maths/pow_x_n.cc) | [Rust](templates/maths/pow_x_n.rs) |
| Modular exponentiation (iterative) | [C++23](templates/maths/pow_x_n_mod.cc) | [Rust](templates/maths/pow_x_n_mod.rs) |
| Modular exponentiation (recursive) | [C++23](templates/maths/pow_x_n_mod_recursive.cc) | [Rust](templates/maths/pow_x_n_mod_recursive.rs) |
| Sieve of Eratosthenes | [C++23](templates/maths/prime_sieve.cc) | [Rust](templates/maths/prime_sieve.rs) |

</details>

<details>
<summary>Strings</summary>

| Template | C++23 | Rust |
| --- | --- | --- |
| KMP (first match) | [C++23](templates/strings/kmp.cc) | [Rust](templates/strings/kmp.rs) |
| KMP (all matches) | [C++23](templates/strings/kmp_advanced.cc) | [Rust](templates/strings/kmp_advanced.rs) |
| Rabin–Karp rolling hash | [C++23](templates/strings/string_hashing.cc) | [Rust](templates/strings/string_hashing.rs) |
| Z-function | [C++23](templates/strings/z_function.cc) | [Rust](templates/strings/z_function.rs) |

Rust string-matching templates take byte slices such as `b"ababa"` and report byte offsets.

</details>

<details>
<summary>Trees</summary>

| Template | C++23 | Rust |
| --- | --- | --- |
| Diameter of an undirected tree | [C++23](templates/trees/diameter_of_undirected_tree.cc) | [Rust](templates/trees/diameter_of_undirected_tree.rs) |
| Fenwick tree (range sums) | [C++23](templates/trees/fenwick_sum.cc) | [Rust](templates/trees/fenwick_sum.rs) |
| Segment tree (threshold search) | [C++23](templates/trees/segment_search.cc) | [Rust](templates/trees/segment_search.rs) |
| Segment tree (range sums) | [C++23](templates/trees/segment_sum.cc) | [Rust](templates/trees/segment_sum.rs) |
| Prefix trie | [C++23](templates/trees/trie.cc) | [Rust](templates/trees/trie.rs) |
| Binary trie (minimum XOR) | [C++23](templates/trees/trie_binary.cc) | [Rust](templates/trees/trie_binary.rs) |

</details>

## Build and test

<details>
<summary>C++23 compiler checks</summary>

Use a C++23 compiler and standard library with `<print>` and `std::views::zip`. The templates are separate translation units.

```sh
find templates -name '*.cc' -print0 |
  xargs -0 c++ -std=c++23 -Wall -Wextra -Wpedantic -Werror -fsyntax-only
```

</details>

<details>
<summary>Rust build and tests</summary>

The Rust library uses the stable toolchain and has no third-party dependencies. Run these commands from the repository root.

```sh
rustup update stable
cargo test --locked
```

Each Rust template can also be compiled on its own.

```sh
rustc --edition=2024 --crate-type lib templates/arrays/kadane.rs -o /tmp/libkadane.rlib
```

</details>

[MIT License](LICENSE)
