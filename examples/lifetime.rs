/// This example demonstrates the use of lifetimes in Rust.
/// We define a struct `LogError` that holds a reference
/// to a string slice. The `parse_log` function takes a
/// string input and returns a vector of `LogError`
/// instances, filtering for lines that contain "ERROR".
/// The main function creates a sample log string and prints
/// out the parsed errors.
pub struct LogError<'a> {
    pub code: u32,
    pub message: &'a str,
}

fn parse_log<'a>(input: &'a str) -> Vec<LogError<'a>> {
    input.lines()
        .filter(|line| line.contains("ERROR"))
        .map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            LogError {
                code: parts[0].parse().unwrap_or(0),
                message: parts[1].trim(),
            }
        })
        .collect()
}

fn main() {
    let log_data = String::from("404: File not found\n200: OK\n500: ERROR Server Meltdown\n403: Forbidden\n");

    let errors = parse_log(&log_data);

    for err in errors {
        println!("Error {}: {}", err.code, err.message);
    }
}
