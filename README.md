# rust-scratchpad

A personal sandbox for experimenting with Rust language features, patterns, and standard library APIs. No external dependencies — everything is built on Rust's stdlib.

## Project Structure

```
rust-scratchpad/
├── Cargo.toml           # Package manifest (edition 2024, no dependencies)
├── config.txt           # Sample file used by the file_io example
├── src/
│   └── lib.rs           # Library crate root (currently empty)
└── examples/
    ├── concurrency_and_shared_state.rs
    ├── file_io_and_error_handling.rs
    └── secure_file_uploader.rs
```

### Examples

Each example is a standalone binary you can run with `cargo run --example <name>`.

| Example | Topic | Key Concepts |
|---|---|---|
| `concurrency_and_shared_state` | Multi-threading | `Arc`, `Mutex`, `mpsc` channels, thread spawning, error handling across threads |
| `file_io_and_error_handling` | File I/O | `File` / `BufReader`, custom `Result` aliases, `Box<dyn Error>`, line-by-line reading |
| `secure_file_uploader` | Type-safe state machines | Phantom types, compile-time state transitions (`Pending → Validated \| Failed`), the type-state pattern in Rust |

## Usage

Run any example with:

```sh
cargo run --example <example_name>
```

For example:

```sh
# Demonstrates shared state and message passing between threads
cargo run --example concurrency_and_shared_state

# Reads config.txt and prints each line
cargo run --example file_io_and_error_handling

# Simulates a file upload workflow with compile-time state guarantees
cargo run --example secure_file_uploader
```

Build everything (library + all examples) without running:

```sh
cargo build
```

## About

This is a learning/scratchpad repository. Each example focuses on a specific Rust concept or standard library feature. The `src/lib.rs` crate is intentionally minimal — the real content lives in the examples.