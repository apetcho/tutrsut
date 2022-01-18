//! Basic Error Handling
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

type BoxResult<T> = Result<T, Box<dyn Error>>;

static MY_FILE: &str = "/Users/oriprox/Documents/cs/rust/practice/tutrust/test.txt";

// ----------
mod box_errors {
    use super::*;

    // ---
    // Result<i32, Box<dyn Error>>
    fn box_errors_examples(file: &str) -> BoxResult<i32> {
        let mut file = File::open(file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents.trim().parse()?)
    }

    // ---
    pub fn basicserrtut() {
        // let val = box_errors_examples(MY_FILE);
        match box_errors_examples(MY_FILE) {
            Ok(val) => println!("[SUCCESS] val={}", val),
            Err(e) => println!("[ERROR] {}", e),
        }
    }
}

pub fn basics_error_handling() {
    box_errors::basicserrtut();
}
