# tantivy_bench_graph

regression tracking for tantivy benchmarks inspired by https://github.com/rust-lang/rustc-perf, which fetches data from `cargo bench`.

https://pseitz.github.io/tantivy_bench_graph/index.html

Regular rust benchmarks are run with `cargo bench` and then converted to csv with `convert_bench_to_csv.rs`.

## Usage
The script will run once for each day and selects the latest commit on that day.

#### Benchmark a date range
`./bench_dates_tantivy -s 2022-08-01 -e 2022-09-01`
#### Benchmark a single day
`./bench_tantivy -d 2022-09-22`

#### Automated in CI
This repo is also the storage for the runs and serves a UI.
`run_bench_ci.sh` will run and commit the results to the repo.

## Results
Each benchmark is stored in a seperate csv file in `bench_results`. 

`convert_csv_to_uplot.rs` converts the csv to `data.json` for uplot.

## Graph
The graph is generated with uplot in `index.html`, which loads `data.json`.

