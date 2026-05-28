use std::cell::RefCell;
/// This example demonstrates how to use Rc and RefCell
/// to achieve interior mutability in Rust.
use std::rc::Rc;

#[derive(Debug)]
struct Config {
    theme: String,
    volume: u8,
}

fn main() {
    let shared_config = Rc::new(RefCell::new(Config {
        theme: "Dark".to_string(),
        volume: 75,
    }));

    let config_pointer = shared_config.clone();

    {
        // this borrow happens at runtime,
        // so we can mutate the config even though it's shared
        // Since it is not thread-safe it does not implement
        // Send or Sync, so we cannot share it across threads
        let mut config = config_pointer.borrow_mut();
        config.theme = "Light".to_string();
        config.volume = 50;
    }

    println!("Updated config: {:?}", shared_config.borrow());
}
