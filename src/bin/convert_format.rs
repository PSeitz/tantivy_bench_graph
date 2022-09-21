use csv::{StringRecord, Writer};
use serde::{Deserialize, Serialize};
use std::{fs, io::Read};

// Tool to edit csv format

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
struct Record {
    ns: u64,
    variance: u64,
    throughput: Option<f64>,
    commit_hash: String,
    commit_message: String,
    commit_ts: u64,
    selected_date: String,
    rustc_version: String,
    run_date_ts: i64,
    run_date: String,
    machine_name: String,
}

fn main() {
    let paths = fs::read_dir("./bench_results").unwrap();
    for path in paths {
        let path = path.unwrap();
        let records = get_records(std::fs::File::open(path.path()).unwrap());
        let mut wtr = Writer::from_path(path.path()).unwrap();
        for record in records {
            wtr.write_record(&[
                record.ns.to_string(),
                record.variance.to_string(),
                record
                    .throughput
                    .map(|el| el.to_string())
                    .unwrap_or("".to_string()),
                record.commit_hash.to_string(),
                record.commit_message.to_string(),
                record.commit_ts.to_string(),
                record.selected_date.to_string(),
                record.rustc_version.to_string(),
                record.run_date_ts.to_string(),
                record.run_date.to_string(),
                record.machine_name.to_string(),
            ])
            .unwrap();
        }

        wtr.flush().unwrap();
    }
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
