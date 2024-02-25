# SIMD Benchmarking for x86/x86_64

An experimental project to benchmark SIMD by computing the rotation matrix from a quaternion.

Make sure the module `criterion` is added. If not, run `cargo add --dev criterion`.

Run `cargo test` to ensure that the functions work fine.

Just run `cargo bench` to see the result, showing time elapsed for each iteration. It looks something like below:

```
portable                time:   [8.1156 ns 8.1581 ns 8.2037 ns]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

half-simd               time:   [3.9866 ns 4.0079 ns 4.0316 ns]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe

simd                    time:   [3.6035 ns 3.6135 ns 3.6248 ns]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
```

It turns out that the totally-SIMD version is the fastest on my machine (maybe it could be faster).
