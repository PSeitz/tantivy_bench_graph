#!/usr/bin/env bash

#set -x

git clone https://github.com/quickwit-oss/tantivy.git 2>/dev/null

# get command line parameters
# usage: ./bench -d 2022-09-22        // will checkout commit close to 2022-09-22 on main branch. results stored in bench_results folder
while getopts m:c:d: flag
do
    case "${flag}" in
        d) commit_date=${OPTARG};;
        *) exit
    esac
done

run_date_ts=$(date +%s)
run_date=$(date +"%d-%m-%y %T")
rustc_version=$(rustc --version)

cd tantivy || exit

git fetch --all

commit_hash=$(git rev-list --max-count=1 --first-parent --before="$commit_date" main)
commit_timestamp=$(git log -n 1 --pretty=format:%ct "$commit_hash")
commit_message=$(git log -n 1 --pretty=format:%s "$commit_hash")
commit_message=${commit_message:0:60}
commit_message=${commit_message//,/} #remove comma for csv compat

echo "Checkout $commit_message ($commit_hash) for commit_date $commit_date"

git checkout "$commit_hash"

#exec bench

run_bench() {
  benchoutput=$(cargo +nightly bench --features unstable | cargobench_to_csv)

  cd - || exit
  mkdir -p bench_results 2>/dev/null

  #store results
  echo "$benchoutput"| while read -r line; do
    IFS=',' read -ra bench_result <<< "$line"
    bench_name=${bench_result[0]}
    ns=${bench_result[1]}
    variance=${bench_result[2]}
    throughput=${bench_result[3]}

    out="$ns,$variance,$throughput,$commit_hash,$commit_message,$commit_timestamp,$commit_date,$rustc_version,$run_date_ts,$run_date"
    echo "$out" >> "bench_results/$bench_name"

  done

}

run_bench;
cd tantivy/fastfield_codecs || exit
run_bench;






