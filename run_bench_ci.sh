#!/usr/bin/env bash
# runs tantivy benchmarks for today, updates the graphs and commits to the repository
# set MACHINE_NAME env if not on awst3

git fetch --all
git rebase

# run benchmarks for today
./bench_dates_tantivy.sh

if [[ -z "${MACHINE_NAME}" ]]; then
  export MACHINE_NAME='awst3'
fi

# Merge csv files with same name
cargo run --release --bin merge_results
# convert data for uplot and store in data.json
cargo run --release --bin convert_csv_to_uplot

git add -A

commit_message=$(date +%F)
git commit -m"Update benchmarks for $commit_message"
git push
