# tantivy_bench_graph

regression tracking for tantivy benchmarks inspired by https://github.com/rust-lang/rustc-perf, which fetches data from `cargo bench`.

https://pseitz.github.io/tantivy_bench_graph/index.html

## Benchmarks

Regular rust benchmarks are run with `cargo bench` and then converted to csv with `convert_bench_to_csv.rs`.

## Results
Each benchmark is stored in a seperate csv file in `bench_results`. 

`convert_csv_to_uplot.rs` converts the csv to `data.json` for uplot.

## Graph
The graph is generated with uplot in `index.html`, which loads `data.json`.

