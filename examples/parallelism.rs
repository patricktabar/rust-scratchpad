/// This example demonstrates how to compute hashes of files
/// in parallel using the Rayon crate.
///
use rayon::prelude::*;
use std::path::PathBuf;

pub struct HashResult {
    pub path: PathBuf,
    pub hash: String,
}

fn compute_hashes_parallel(files: Vec<PathBuf>) -> Vec<HashResult> {
    files
        .into_par_iter()
        .map(|path| {
            // Simulate hash computation (replace with actual hash logic)
            let mock_hash = format!("hash_of_{:?}", path);
            HashResult {
                path,
                hash: mock_hash,
            }
        })
        .collect()
}

fn main() {
    let files = vec![
        PathBuf::from("file1.txt"),
        PathBuf::from("file2.txt"),
        PathBuf::from("file3.txt"),
    ];

    let count = std::thread::available_parallelism().unwrap().get();
    println!("This machine can handle {} parallel tasks.", count);

    let results = compute_hashes_parallel(files);

    for result in results {
        println!("File: {:?}, Hash: {}", result.path, result.hash);
    }
}
