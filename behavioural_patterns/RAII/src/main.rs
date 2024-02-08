use std::fs::File;
use std::io::{self, Read};

fn read_file_contents(filename: &str) -> io::Result<String> {
    // Acquire the file handle within a scoped block
    let file = File::open(filename)?;
    
    // Create a guard object that will release the file handle when it goes out of scope
    let guard = FileGuard { file: Some(file) };

    // Read the file contents using the file handle
    let mut contents = String::new();
    guard.file.as_ref().unwrap().read_to_string(&mut contents)?;

    // Return the file contents
    Ok(contents)
} // The guard goes out of scope here, releasing the file handle

// Define a guard struct to manage the file handle
struct FileGuard {
    file: Option<File>,
}

impl Drop for FileGuard {
    // Implement the Drop trait to release the resource when the guard goes out of scope
    fn drop(&mut self) {
        if let Some(file) = self.file.take() {
            // Release the file handle
            // The file handle is automatically closed when it goes out of scope
        }
    }
}

fn main() {
    let filename = "example.txt";
    match read_file_contents(filename) {
        Ok(contents) => println!("File contents: {}", contents),
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}
