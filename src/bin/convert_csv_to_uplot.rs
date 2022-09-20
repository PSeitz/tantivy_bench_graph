use csv::StringRecord;
use serde::Serialize;
use serde_json::json;
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
    commit_ts: u64,
    rustc_version: String,
    run_date_ts: i64,
    run_date: String,
}

// uPlot format
//
// let data = [
//  [1546300800, 1546387200],    // x-values (timestamps)  ts commit_date
//  [        35,         71],    // y-values (series 1)    ns
//  [        90,         15],    // y-values (series 2)    variance
// ];

fn main() {
    let paths = fs::read_dir("./bench_results").unwrap();

    let mut bench_name_to_result = HashMap::new();
    let mut commits = HashMap::new();
    for path in paths {
        let path = path.unwrap();
        let bench_test = path.file_name().to_str().unwrap().to_string();
        let records = get_records(std::fs::File::open(path.path()).unwrap());
        let bench_data_uplot = get_uplot_prepared_data(&records);
        let mut bench_info = HashMap::new();
        bench_info.insert("uplot_data", bench_data_uplot);
        let commit_hashs = records
            .iter()
            .map(|record| record.commit_hash.to_string())
            .collect::<Vec<_>>();
        bench_info.insert(
            "commit_hashes",
            serde_json::to_value(&commit_hashs).unwrap(),
        );
        bench_name_to_result.insert(bench_test, serde_json::to_value(&bench_info).unwrap());

        for record in records {
            commits.insert(record.commit_hash, record.commit_message);
        }
    }

    bench_name_to_result.insert(
        "commits".to_string(),
        serde_json::to_value(&commits).unwrap(),
    );

    let jsonstr = serde_json::to_string_pretty(&bench_name_to_result).unwrap();

    std::fs::File::create("data.json")
        .unwrap()
        .write_all(jsonstr.as_bytes())
        .unwrap();
}

fn get_records<R: Read>(reader: R) -> Vec<Record> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut records = vec![];

    for result in rdr.records() {
        let record: StringRecord = result.unwrap();
        let record: Record = record.deserialize(None).unwrap();

        records.push(record);
    }
    records
}

fn get_uplot_prepared_data(records: &[Record]) -> serde_json::Value {
    let mut timestamps = vec![];
    let mut duration = vec![];
    let mut variance = vec![];

    for record in records {
        timestamps.push(record.commit_ts);
        duration.push(record.ns);
        variance.push(record.variance);
    }
    json!(vec![timestamps, duration, variance])
}
