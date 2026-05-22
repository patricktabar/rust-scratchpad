use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

fn process_config<P: AsRef<Path>>(path: P) -> MyResult<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let content = line?;
        println!("Line {} {}", index + 1, content.trim());
    }

    Ok(())
}

fn main () {
    let path="config.txt";

    if let Err(e) = process_config(path) {
        eprintln!("Error processing config file: {}", e);
        std::process::exit(1);
    }
}
