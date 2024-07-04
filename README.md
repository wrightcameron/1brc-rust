# 1 Billion Row Challenge in Rust

My own implementation of the 1 billion row challange, though done in Rust.

## Requirements

- Rust
- Java 21 (To generate 1b rows)

### Flamegraph

For flamegraph to work perf needs to be installed.  On Ubuntu that will be installed with `linux-tools-generic`.

## Setup

`cd tools; ./downloadSampleData.sh`

## Usage

`cargo run --release -- ./data/<file>`

### Profiling

`cargo flamegraph --open -- ./data/<file>`

## Resources

- [The One Billion Row Challenge Blogpost](https://www.morling.dev/blog/one-billion-row-challenge/)
- [1BRC Github Repo](https://github.com/gunnarmorling/1brc)
- [1BRC Baseline](https://github.com/gunnarmorling/1brc/blob/main/src/main/java/dev/morling/onebrc/CalculateAverage_baseline.java)
- [1 Billion Row Challange Rust](https://curiouscoding.nl/posts/1brc/#the-problem)

### Profiling Referneces

- [Flamegraph](https://github.com/flamegraph-rs/flamegraph)
- [StackOverflow: How to Change Kernel Perf Paranoid Settings](https://askubuntu.com/questions/1471162/how-to-change-kernel-perf-event-paranoid-settings)
