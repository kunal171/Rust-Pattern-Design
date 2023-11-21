#![allow(unused)]
use std::{path::PathBuf, time::Duration};

// note that we can simply auto-derive Default here.
#[derive(Default, Debug, PartialEq)]
struct MyConfiguration {
    // Option defaults to None
    output: Option<PathBuf>,
    // Vecs default to empty vector
    search_path: Vec<PathBuf>,
    // Duration defaults to zero time
    timeout: Duration,
    // bool defaults to false
    check: bool,
}

impl MyConfiguration {
    // add setters here
}

#[derive(Default)]
struct Rectangle {
    width: u32,
    height: u32,
}


fn main() {
    /// Time in seconds.
    ///
    /// # Example
    ///
    /// ```
    /// let s = Second::default();
    /// assert_eq!(0, s.value());
    /// ```
    #[derive(Default)]
    pub struct Second {
        value: u64
    }

    impl Second {
        /// Returns the value in seconds.
        pub fn value(&self) -> u64 {
            self.value
        }
    }
    // Using the default constructor provided by the Default trait
    let default_rect = Rectangle::default();
    println!("Default Rectangle: {} x {}", default_rect.width, default_rect.height);

    // construct a new instance with default values
    let mut conf = MyConfiguration::default();
    // do something with conf here
    conf.check = true;
    println!("conf = {:#?}", conf);
        
    // partial initialization with default values, creates the same instance
    let conf1 = MyConfiguration {
        check: true,
        ..Default::default()
    };
    assert_eq!(conf, conf1);

}

