<h1 align="center">DSA Template</h1>

<p align="center">
  <a href="https://github.com/brucerry/DSA-Template/actions/workflows/ci.yml"><img src="https://github.com/brucerry/DSA-Template/actions/workflows/ci.yml/badge.svg?branch=main" alt="CI status on main"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="MIT License"></a>
</p>

<p align="center">Classic data structures and algorithms.</p>

---

## Templates

### Arrays

<details>
<summary>Array source files</summary>

<table>
  <tbody>
    <tr>
      <th scope="row" align="left">Kadane's maximum subarray</th>
      <td><a href="templates/arrays/kadane.cc">C++23</a></td>
      <td><a href="templates/arrays/kadane.rs">Rust</a></td>
    </tr>
  </tbody>
</table>

</details>

### Graphs

<details>
<summary>Graph source files</summary>

<table>
  <tbody>
    <tr>
      <th scope="row" align="left">Union-Find</th>
      <td><a href="templates/graphs/union_find.cc">C++23</a></td>
      <td><a href="templates/graphs/union_find.rs">Rust</a></td>
    </tr>
  </tbody>
</table>

</details>

### Linked lists

<details>
<summary>Linked-list source files</summary>

<table>
  <tbody>
    <tr>
      <th scope="row" align="left">LRU cache</th>
      <td><a href="templates/linked_list/lru.cc">C++23</a></td>
      <td><a href="templates/linked_list/lru.rs">Rust</a></td>
    </tr>
  </tbody>
</table>

</details>

### Maths

<details>
<summary>Maths source files</summary>

<table>
  <tbody>
    <tr>
      <th scope="row" align="left">Binary exponentiation</th>
      <td><a href="templates/maths/pow_x_n.cc">C++23</a></td>
      <td><a href="templates/maths/pow_x_n.rs">Rust</a></td>
    </tr>
    <tr>
      <th scope="row" align="left">Modular exponentiation (iterative)</th>
      <td><a href="templates/maths/pow_x_n_mod.cc">C++23</a></td>
      <td><a href="templates/maths/pow_x_n_mod.rs">Rust</a></td>
    </tr>
    <tr>
      <th scope="row" align="left">Modular exponentiation (recursive)</th>
      <td><a href="templates/maths/pow_x_n_mod_recursive.cc">C++23</a></td>
      <td><a href="templates/maths/pow_x_n_mod_recursive.rs">Rust</a></td>
    </tr>
    <tr>
      <th scope="row" align="left">Sieve of Eratosthenes</th>
      <td><a href="templates/maths/prime_sieve.cc">C++23</a></td>
      <td><a href="templates/maths/prime_sieve.rs">Rust</a></td>
    </tr>
  </tbody>
</table>

</details>

### Strings

<details>
<summary>String source files</summary>

<table>
  <tbody>
    <tr>
      <th scope="row" align="left">KMP (first match)</th>
      <td><a href="templates/strings/kmp.cc">C++23</a></td>
      <td><a href="templates/strings/kmp.rs">Rust</a></td>
    </tr>
    <tr>
      <th scope="row" align="left">KMP (all matches)</th>
      <td><a href="templates/strings/kmp_advanced.cc">C++23</a></td>
      <td><a href="templates/strings/kmp_advanced.rs">Rust</a></td>
    </tr>
    <tr>
      <th scope="row" align="left">Rabin–Karp rolling hash</th>
      <td><a href="templates/strings/string_hashing.cc">C++23</a></td>
      <td><a href="templates/strings/string_hashing.rs">Rust</a></td>
    </tr>
    <tr>
      <th scope="row" align="left">Z-function</th>
      <td><a href="templates/strings/z_function.cc">C++23</a></td>
      <td><a href="templates/strings/z_function.rs">Rust</a></td>
    </tr>
  </tbody>
</table>

Rust string-matching templates take byte slices such as `b"ababa"` and report byte offsets.

</details>

### Trees

<details>
<summary>Tree source files</summary>

<table>
  <tbody>
    <tr>
      <th scope="row" align="left">Diameter of an undirected tree</th>
      <td><a href="templates/trees/diameter_of_undirected_tree.cc">C++23</a></td>
      <td><a href="templates/trees/diameter_of_undirected_tree.rs">Rust</a></td>
    </tr>
    <tr>
      <th scope="row" align="left">Fenwick tree (range sums)</th>
      <td><a href="templates/trees/fenwick_sum.cc">C++23</a></td>
      <td><a href="templates/trees/fenwick_sum.rs">Rust</a></td>
    </tr>
    <tr>
      <th scope="row" align="left">Segment tree (threshold search)</th>
      <td><a href="templates/trees/segment_search.cc">C++23</a></td>
      <td><a href="templates/trees/segment_search.rs">Rust</a></td>
    </tr>
    <tr>
      <th scope="row" align="left">Segment tree (range sums)</th>
      <td><a href="templates/trees/segment_sum.cc">C++23</a></td>
      <td><a href="templates/trees/segment_sum.rs">Rust</a></td>
    </tr>
    <tr>
      <th scope="row" align="left">Prefix trie</th>
      <td><a href="templates/trees/trie.cc">C++23</a></td>
      <td><a href="templates/trees/trie.rs">Rust</a></td>
    </tr>
    <tr>
      <th scope="row" align="left">Binary trie (minimum XOR)</th>
      <td><a href="templates/trees/trie_binary.cc">C++23</a></td>
      <td><a href="templates/trees/trie_binary.rs">Rust</a></td>
    </tr>
  </tbody>
</table>

</details>

---

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
