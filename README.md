# C++23 and Rust benchmarks

Revision `2c0ef07` on Ubuntu 24.04.5 LTS · x86_64.
12 timed pairs follow 2 unreported pairs per workload.

- C++ compiler `g++-14 (Ubuntu 14.2.0-4ubuntu2~24.04.1) 14.2.0` with `-std=c++23 -O3 -DNDEBUG -Wall -Wextra -Wpedantic -Werror`
- Rust compiler `rustc 1.100.0 (0ef1bc386 2026-11-09)` with `--edition=2024 -C opt-level=3 -D warnings`
- GitHub runner image `20261116.133.1`

Both implementations receive the same deterministic input. The runner alternates which
language goes first and rejects mismatched checksums. Timing starts after input generation
and tree construction. The sieve includes allocating and building its result.

Times are medians. Ratio intervals show the middle 50% of paired Rust / C++23 runtimes.
A ratio below 1 favors Rust. A ratio above 1 favors C++23. Shared-runner load,
compiler choices, algorithms, and data layouts can all affect the results.

## Small

| Workload | Input | C++23 (ms) | Rust (ms) | Rust / C++23 | Middle 50% |
| --- | --- | ---: | ---: | ---: | ---: |
| Kadane | 8,192 integers | 0.0074 | 0.0046 | 0.62× | 0.61–0.63× |
| Union-Find | 1,024 nodes, 4,096 merges and connectivity checks | 0.0576 | 0.0252 | 0.44× | 0.41–0.46× |
| Prime sieve | Primality flags through 8,192 | 0.0129 | 0.0080 | 0.64× | 0.61–0.72× |
| KMP first match | 8,192 text bytes, 8 pattern bytes | 0.0158 | 0.0277 | 1.76× | 1.72–1.77× |
| Fenwick sums | 1,024 values, 4,096 update–query pairs | 0.104 | 0.0997 | 0.96× | 0.94–1.03× |
| Segment-tree sums | 1,024 values, 4,096 update–query pairs | 0.704 | 0.282 | 0.40× | 0.39–0.42× |

## Medium

| Workload | Input | C++23 (ms) | Rust (ms) | Rust / C++23 | Middle 50% |
| --- | --- | ---: | ---: | ---: | ---: |
| Kadane | 262,144 integers | 0.231 | 0.150 | 0.64× | 0.63–0.67× |
| Union-Find | 16,384 nodes, 65,536 merges and connectivity checks | 1.127 | 0.419 | 0.37× | 0.36–0.38× |
| Prime sieve | Primality flags through 262,144 | 0.549 | 0.343 | 0.63× | 0.60–0.64× |
| KMP first match | 262,144 text bytes, 256 pattern bytes | 0.482 | 0.901 | 1.88× | 1.82–1.90× |
| Fenwick sums | 16,384 values, 65,536 update–query pairs | 2.007 | 1.908 | 0.95× | 0.95–0.96× |
| Segment-tree sums | 16,384 values, 65,536 update–query pairs | 15.95 | 6.032 | 0.38× | 0.38–0.38× |

## Large

| Workload | Input | C++23 (ms) | Rust (ms) | Rust / C++23 | Middle 50% |
| --- | --- | ---: | ---: | ---: | ---: |
| Kadane | 8,388,608 integers | 7.615 | 6.704 | 0.88× | 0.87–0.89× |
| Union-Find | 262,144 nodes, 1,048,576 merges and connectivity checks | 23.27 | 11.18 | 0.48× | 0.47–0.51× |
| Prime sieve | Primality flags through 8,388,608 | 43.73 | 26.29 | 0.60× | 0.60–0.60× |
| KMP first match | 8,388,608 text bytes, 8,192 pattern bytes | 15.31 | 28.39 | 1.86× | 1.85–1.86× |
| Fenwick sums | 262,144 values, 1,048,576 update–query pairs | 38.23 | 37.45 | 0.98× | 0.98–1.00× |
| Segment-tree sums | 262,144 values, 1,048,576 update–query pairs | 351.33 | 128.22 | 0.37× | 0.36–0.37× |
