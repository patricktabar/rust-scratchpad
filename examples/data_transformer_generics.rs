/// This example show how to use traits and generics
/// to create a flexible data transformation system.
use std::num::ParseIntError;

pub trait Transformer {
    type Output;
    fn transform(&self, input: &str) -> Self::Output;
}

struct NumberParser;

impl Transformer for NumberParser {
    type Output = Result<i32, ParseIntError>;

    fn transform(&self, input: &str) -> Self::Output {
        input.trim().parse()
    }
}

pub trait Summary {
    fn summarize(&self, input: &str) -> String;
}

impl<T, O, E> Summary for T
where
    T: Transformer<Output = Result<O, E>>,
    O: std::fmt::Display,
    E: std::fmt::Display,
{
    // We can use the input parameter dynamically!
    fn summarize(&self, input: &str) -> String {
        match self.transform(input) {
            Ok(val) => format!("Transformer output success: {}", val),
            Err(err) => format!("Transformer output error: {}", err),
        }
    }
}

fn main() {
    let parser = NumberParser;

    let input_a = "100";
    let input_b = "not a number";

    // Now you can pass whatever string you want!
    println!("{}", parser.summarize(input_a));
    println!("{}", parser.summarize(input_b));
}
