/// This example demonstrates how to create a simple macro
/// in Rust to define newtypes.
/// This is how it works:
/// the `newtype!` macro takes two arguments:
/// the name of the newtype and the inner type.
/// When macro newtype! get used, it generates a new struct
/// with the specified name and inner type, the macro provide two
/// implementations:
/// - one for the Display trait to allow the newtype to be printed,
/// - one for the Deref trait to allow access to the inner type's methods.
macro_rules! newtype {
    ($name: ident, $inner: ty) => {
        pub struct $name($inner);

        // Implementing the Display trait for the newtype
        // to allow it to be printed
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}({})", stringify!($name), self.0)
            }
        }

        // Implementing the Deref trait to allow access
        // to the inner type's methods
        impl std::ops::Deref for $name {
            type Target = $inner;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

macro_rules! reminder {
    ($message:expr) => {
           println!("REMINDER: {}", $message);
       };
}

newtype!(UserId, u32);
newtype!(Username, String);

fn process_user(id: UserId, name: Username) {
    println!("Processing user: {} with ID: {}", name, id);
}

fn complicated_algorithm() {
    reminder!("Need to implement the optimization phase 😃");
}

fn main() {
    let user_id = UserId(42);
    let username = Username("Alice".to_string());

    process_user(user_id, username);
    complicated_algorithm();
}
