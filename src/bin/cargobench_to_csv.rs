use std::io::{self, BufReader};

use rust_bench_parser::parse_lines;

fn main() -> io::Result<()> {
    let benchmarks = parse_lines(BufReader::new(io::stdin()))?;

    for benc in benchmarks {
        println!(
            "{},{},{},{}",
            benc.name,
            benc.ns,
            benc.variance,
            benc.throughput
                .map(|throughput| throughput.to_string())
                .unwrap_or("".to_string())
        )
    }

    Ok(())
}
