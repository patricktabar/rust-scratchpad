/// This example demonstrates how to use Arc and Mutex for shared state across threads,
/// and how to use channels for communication between threads.

use std::{sync::{Arc, Mutex, mpsc}, thread};
use std::time::Duration;

fn main() {
    // Shared state using Arc and Mutex
    let counter = Arc::new(Mutex::new(0));

    // Channel for communication between threads; tx for sending messages, rx for receiving
    let (tx, rx) = mpsc::channel();

    for i in 0..5 {
        let counter_clone = counter.clone();
        let sender_clone = tx.clone();

        thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;

            sender_clone.send(format!("Thread {} finished. Current count: {}", i, *num)).unwrap();

            thread::sleep(Duration::from_millis(100));
        });
    }

    drop(tx); // Close the channel

    // Wait for all threads to finish and print messages
    for message in rx {
        println!("Main received: {}", message);
    }

    println!("Final shared count: {}", *counter.lock().unwrap());
}
