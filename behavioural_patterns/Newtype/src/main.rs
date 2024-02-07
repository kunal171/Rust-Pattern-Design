use std::fmt::Display;

// Create Newtype Password to override the Display trait for String
struct Password(String);

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "****************")
    }
}


// Define a newtype struct `Meters` that wraps the `f64` type
struct Meters(f64);

// Implement methods for the `Meters` struct
impl Meters {
    // Constructor method to create a new `Meters` instance
    fn new(value: f64) -> Self {
        Meters(value)
    }
    
    // Method to convert meters to feet
    fn to_feet(&self) -> f64 {
        self.0 * 3.28084
    }
}



fn main() {
    // Create a new `Meters` instance
    let distance = Meters::new(10.0);
    
    // Access the value and convert it to feet
    println!("Distance in meters: {}", distance.0);
    println!("Distance in feet: {}", distance.to_feet());
    let unsecured_password: String = "ThisIsMyPassword".to_string();
    let secured_password: Password = Password(unsecured_password.clone());
    println!("unsecured_password: {unsecured_password}");
    println!("secured_password: {secured_password}");
}
