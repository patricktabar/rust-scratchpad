/// This example demonstrates how to use iterators to process files
/// in a directory and filter them based on line count.
use std::path::{Path, PathBuf};

pub struct FileReport {
    pub path: PathBuf,
    pub line_count: u64,
}

pub fn get_large_files(dir: &Path, min_lines: u64) -> Vec<FileReport> {
    // read the directory and filter files based on line count
    std::fs::read_dir(dir)
        .expect("Failed to read directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| {
            let meta = entry.metadata().expect("Failed to get metadata");
            FileReport {
                path: entry.path(),
                line_count: meta.len() as u64, // This is a placeholder; you would actually read the file and count lines
            }
        })
        .filter(|report| report.line_count > min_lines)
        .collect()
}

fn main() {
    let dir = Path::new(".");
    let min_lines = 100;
    let large_files = get_large_files(dir, min_lines);

    for file in large_files {
        println!("File: {:?}, Line Count: {}", file.path, file.line_count);
    }
}
