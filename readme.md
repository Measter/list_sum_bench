A quick benchmark of three ways of summing integers in both Rust (rustc 1.61) and C# (.Net 6). The three ways are:

 * A `for` loop over the range of indices which are then used to index into the list. In C# this uses `for`, in Rust a
   `for` loop over the range `0..length`.
 * A `foreach` loop over the collection. In C# this uses `foreach`, in Rust a `for` loop over the collection.
 * The Iterator/LINQ method. In C# this uses the `Aggregate` method (the `Sum` method overflow checks), while in Rust
   the `sum` method was used.

The integers to be summed are 20 million signed 32-bit integers, which are read and parsed from a file at runtime prior to
benchmarking. The numbers are generated using the program in the `generator` directory using the ChaCha12 RNG from the
[`rand_chacha`](https://crates.io/crates/rand_chacha) library. Simply run the generator program with `cargo run --release`
to generate the `numbers.txt` file. It should be approximately 209MB. The benchmarks will try to read the file from the
`generator` directory.

The benchmarks can be run by running `dotnet run -c Release` in the `dotnet_sum` directory, and `cargo bench` in the `rust_sum`
directory. My results were:

# C#

```
// * Summary *

BenchmarkDotNet=v0.13.1, OS=Windows 10.0.19044.1706 (21H2)
11th Gen Intel Core i7-11700K 3.60GHz, 1 CPU, 16 logical and 8 physical cores
.NET SDK=6.0.105
  [Host]     : .NET 6.0.5 (6.0.522.21309), X64 RyuJIT
  DefaultJob : .NET 6.0.5 (6.0.522.21309), X64 RyuJIT


|      Method |       Mean |     Error |    StdDev |
|------------ |-----------:|----------:|----------:|
|     Sum_For |   8.771 ms | 0.0473 ms | 0.0442 ms |
| Sum_ForEach |  12.801 ms | 0.0412 ms | 0.0385 ms |
|    Sum_Linq | 102.969 ms | 0.5060 ms | 0.4733 ms |
```

# Rust

```
sum_for                 time:   [2.5527 ms 2.5617 ms 2.5715 ms]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

sum_foreach             time:   [2.5543 ms 2.5626 ms 2.5714 ms]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

sum_iter                time:   [2.5485 ms 2.5572 ms 2.5665 ms]
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high mild
```
