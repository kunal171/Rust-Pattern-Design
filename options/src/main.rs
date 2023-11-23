//!Option can be viewed as a container that contains either zero or one element. 
//!In particular, it implements the IntoIterator trait, and as such can be used with generic code that needs such a type.
//! 
//! Iterating over an Option in Rust is a bit different from iterating over a collection like a Vec or a VecDeque. 
//! Since Option represents either some value (Some) or no value (None), you don't iterate over elements directly. 
//! Instead, you need to handle both cases explicitly.

#![allow(unused)]
fn main() {
let turing = Some("Turing");
    let mut logicians = vec!["Curry", "Kleene", "Markov"];

    logicians.extend(turing);

    // equivalent to
    if let Some(turing_inner) = turing {
        logicians.push(turing_inner);
    }

    for logician in logicians.iter().chain(turing.iter()) {
        println!("{} is a logician", logician);
    }    

    let maybe_value: Option<i32> = Some(42);

    // Using match to handle both Some and None cases
    match maybe_value {
        Some(value) => {
            println!("Found value: {}", value);
            // Perform actions with the value
        }
        None => {
            println!("No value found");
            // Handle the absence of a value
        }
    }

    // Alternatively, using if let for a more concise syntax
    if let Some(value) = maybe_value {
        println!("Found value: {}", value);
        // Perform actions with the value
    } else {
        println!("No value found");
        // Handle the absence of a value
    }
}