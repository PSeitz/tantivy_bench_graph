use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

fn main() {
    let entries = fs::read_dir("./bench_results").unwrap();

    let mut bench_name_to_paths: HashMap<String, Vec<_>> = HashMap::new();
    for entry in entries {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().to_str().unwrap().to_string();
            let bench_name = file_name
                .rsplit_once("::")
                .map(|(_suffix, bench_name)| bench_name.to_string())
                .unwrap_or(file_name.to_string());
            let bench_paths = bench_name_to_paths
                .entry(bench_name.to_string())
                .or_default();
            bench_paths.push(entry.path());
        }
    }
    // get duplicates
    let iter_duplicates = bench_name_to_paths
        .iter()
        .filter(|(_name, paths)| paths.len() > 1);

    for (_name, paths) in iter_duplicates {
        let last_modified: Vec<(PathBuf, std::time::SystemTime)> = paths
            .iter()
            .map(|path| {
                (
                    path.to_owned(),
                    fs::metadata(path)
                        .unwrap()
                        .modified()
                        .expect("could not get last modified of file"),
                )
            })
            .collect::<Vec<_>>();
        // newest file is the target
        let target = last_modified
            .iter()
            .max_by_key(|(_path, last_modified)| last_modified)
            .map(|(path, _last_modified)| path)
            .unwrap();

        let other_paths = paths
            .iter()
            .filter(|path| path != &target)
            .collect::<Vec<_>>();
        println!("Merging {:?} into {:?}", other_paths, target);

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&target)
            .unwrap();

        for duplicate in other_paths {
            let data = fs::read_to_string(duplicate).unwrap();
            file.write_all(&data.as_bytes()).unwrap();
            fs::remove_file(duplicate).unwrap();
        }
    }
}
