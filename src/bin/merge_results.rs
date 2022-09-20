use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::Write,
};

fn main() {
    let entries = fs::read_dir("./bench_results").unwrap();

    let mut bench_name_to_path: HashMap<String, Vec<_>> = HashMap::new();
    for entry in entries {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().to_str().unwrap().to_string();
            let bench_name = file_name
                .rsplit_once("::")
                .map(|(_suffix, bench_name)| bench_name.to_string())
                .unwrap_or(file_name.to_string());
            let bench_paths = bench_name_to_path
                .entry(bench_name.to_string())
                .or_default();
            bench_paths.push(entry.path());
        }
    }
    // get duplicates
    let iter = bench_name_to_path
        .iter()
        .filter(|(_name, paths)| paths.len() > 1);

    for (_name, paths) in iter {
        // newest file is the target
        let target = paths
            .iter()
            .max_by_key(|path| {
                fs::metadata(path)
                    .unwrap()
                    .modified()
                    .expect("could not get last modified of file")
            })
            .unwrap();

        let other_paths = paths.iter().filter(|path| path != &target);

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(target)
            .unwrap();

        for duplicate in other_paths {
            let data = fs::read_to_string(duplicate).unwrap();
            file.write_all(&data.as_bytes()).unwrap();
            fs::remove_file(duplicate).unwrap();
        }
    }
}
