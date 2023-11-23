fn divide(a: i32, b: i32) -> Result<f64, String> {
    if b == 0 {
        // Return an error if the divisor is zero
        Err("Cannot divide by zero".to_string())
    } else {
        // Return the result if division is successful
        Ok(a as f64 / b as f64)
    }
}

fn main() {
    // Call the fallible function and handle the result
    match divide(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }

    // Handling another case
    match divide(5, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}
