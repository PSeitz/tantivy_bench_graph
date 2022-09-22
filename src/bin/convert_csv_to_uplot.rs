use chrono::NaiveDate;
use csv::StringRecord;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::io::Write;

use serde::Deserialize;

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
struct Record {
    ns: u64,
    variance: u64,
    throughput: Option<f64>,
    commit_hash: String,
    commit_message: String,
    /// Timestamp of the commit
    commit_ts: u64,
    /// The tool accepts a date for which the closest commit is selected
    /// Format: 2022-09-19
    selected_date: String,
    rustc_version: String,
    run_date_ts: i64,
    run_date: String,
    machine_name: String,
}

impl Record {
    fn get_selected_date_as_timestamp(&self) -> u64 {
        NaiveDate::parse_from_str(&self.selected_date, "%Y-%m-%d")
            .unwrap()
            .and_time(Default::default())
            .timestamp() as u64
    }
}

// uPlot format
//
// let data = [
//  [1546300800, 1546387200],    // x-values (timestamps)  selected_date (so when it's run once per
//  day, we get a nice graph)
//  [        35,         71],    // y-values (series 1)    ns
//  [        90,         15],    // y-values (series 2)    variance
// ];
//

#[derive(Debug, Deserialize, Serialize)]
struct DataFormat {
    commit_hash_to_message: HashMap<String, String>,
    benchmarks: Vec<BenchMarkInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
struct BenchMarkInfo {
    name: String,
    commit_hashes: Vec<String>,
    uplot_data: [Vec<u64>; 3], //timestamps, duration, variance
}

fn main() -> std::io::Result<()> {
    let paths = fs::read_dir("./bench_results").unwrap();

    let mut commit_hash_to_message = HashMap::new();

    let mut benchmarks = Vec::new();
    for dir_entry in paths.filter_map(|dir_entry| dir_entry.ok()) {
        let bench_test = dir_entry.file_name().to_str().unwrap().to_string();
        let records = get_records(std::fs::File::open(dir_entry.path()).unwrap());
        let bench_data_uplot = get_uplot_prepared_data(&records);

        let commit_hashs = records
            .iter()
            .map(|record| record.commit_hash.to_string())
            .collect::<Vec<_>>();
        let benchmark_info = BenchMarkInfo {
            name: bench_test,
            commit_hashes: commit_hashs.to_owned(),
            uplot_data: bench_data_uplot,
        };
        benchmarks.push(benchmark_info);

        for record in records {
            commit_hash_to_message.insert(record.commit_hash, record.commit_message);
        }
    }

    benchmarks.sort_by_key(|benchmark| benchmark.name.to_string());
    let data = DataFormat {
        commit_hash_to_message,
        benchmarks,
    };

    let jsonstr = serde_json::to_string_pretty(&data).unwrap();

    std::fs::File::create("data.json")
        .unwrap()
        .write_all(jsonstr.as_bytes())
        .unwrap();

    Ok(())
}

fn get_records<R: Read>(reader: R) -> Vec<Record> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut records = vec![];

    for result in rdr.records() {
        let record: StringRecord = result.unwrap();
        let record: Record = record.deserialize(None).unwrap();

        records.push(record);
    }
    records.sort_by_key(|record| record.commit_ts);
    records
}

fn get_uplot_prepared_data(records: &[Record]) -> [Vec<u64>; 3] {
    let mut timestamps = vec![];
    let mut duration = vec![];
    let mut variance = vec![];

    for record in records {
        timestamps.push(record.get_selected_date_as_timestamp());
        duration.push(record.ns);
        variance.push(record.variance);
    }
    [timestamps, duration, variance]
}
