use std::ffi::CString;
use std::os::raw::c_char;

// This example demonstrates how to call a C function
// from Rust using FFI (Foreign Function Interface).
// extern "C" is used to specify that the function follows the C
// calling convention.
unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
}

pub fn get_c_string_length(s: &str) -> usize {
    let c_str = CString::new(s).expect("CString::new failed");

    unsafe { strlen(c_str.as_ptr()) }
}

fn main() {
    let my_rust_str = "Rust is safe, but C is fast!";
    let len = get_c_string_length(my_rust_str);
    println!("Length according to C: {}", len);
}
