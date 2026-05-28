/// This example demonstrates the use of `Cow` (Clone on Write)
/// in Rust. Cow is useful for String, Array and PathBuf types
/// when you want to avoid unnecessary cloning.
use std::borrow::Cow;

fn sanitize_username(input: &str) -> Cow<'_, str> {
    if input.contains(' ') {
        // Allocation happens here: we modify the string, so Cow becomes Cow::Owned
        let sanitized = input.replace(' ', "_");
        Cow::Owned(sanitized)
    } else {
        // No allocation: we just borrow the original input as Cow::Borrowed
        Cow::Borrowed(input)
    }
}

fn main() {
    // Case 1: The input is already clean.
    // No new memory is allocated. `result1` just points to the original string literal.
    let user1 = "FerrisTheCrab";
    let result1 = sanitize_username(user1);

    println!(
        "Result 1: {} (Allocated: {})",
        result1,
        matches!(result1, Cow::Owned(_))
    );

    // Case 2: The input needs cleaning.
    // A new String is allocated and owned by `result2`.
    let user2 = "Ferris The Crab";
    let result2 = sanitize_username(user2);

    println!(
        "Result 2: {} (Allocated: {})",
        result2,
        matches!(result2, Cow::Owned(_))
    );
}
