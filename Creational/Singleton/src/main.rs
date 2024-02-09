use std::sync::{Arc, Mutex};

// Define a struct to represent the singleton object
pub struct Singleton {
    // This field holds the single instance of the Singleton
    data: String,
}

impl Singleton {
    // Private constructor to prevent external instantiation
    fn new(data: String) -> Self {
        Self { data }
    }

    // Public method to get the instance of the Singleton
    pub fn instance() -> Arc<Mutex<Singleton>> {
        // Initialize the Singleton instance lazily using a Mutex for thread safety
        static mut INSTANCE: Option<Arc<Mutex<Singleton>>> = None;

        unsafe {
            INSTANCE.get_or_insert_with(|| {
                Arc::new(Mutex::new(Self::new("Example data".to_string())))
            }).clone()
        }
    }

    // Public method to access data
    pub fn get_data(&self) -> &str {
        &self.data
    }

    // Public method to modify data
    pub fn set_data(&mut self, new_data: String) {
        self.data = new_data;
    }
}

fn main() {
    // Access the Singleton instance
    let singleton_instance = Singleton::instance();

    // Lock the Mutex to access the Singleton
    let mut singleton = singleton_instance.lock().unwrap();

    // Access data
    println!("Data: {}", singleton.get_data());

    // Modify data
    singleton.set_data("New data".to_string());

    // Access modified data
    println!("Modified Data: {}", singleton.get_data());
}
