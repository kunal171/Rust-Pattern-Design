//!Error handling in Foreign Function Interface (FFI) code involves handling errors that may occur when interacting with functions written in languages other than Rust,
//! particularly when using C libraries. FFI often involves crossing language boundaries, and it's crucial to manage errors gracefully. 

/*One common approach is to use the return value of the foreign function to indicate success or failure.
 Conventionally, C functions often use an integer return type, where a non-zero value indicates an error. */
extern "C" {
    fn foreign_function() -> i32;
}

fn main() {
    let result = unsafe { foreign_function() };

    if result != 0 {
        println!("Error: {}", result);
    } else {
        println!("Success");
    }
}
