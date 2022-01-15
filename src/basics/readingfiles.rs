//! Reading From Files
use std::env;
use std::fs::File;
use std::io::Read;

mod file1 {
    use super::{env, File, Read};
    pub fn file1_examples() {
        let second = env::args().nth(2).expect("please supply a filename");
        let mut file = File::open(&second).expect("Can't open the file");
        let mut text = String::new();
        file.read_to_string(&mut text).expect("Can't read the file");
        println!("file had {} bytes", text.len());
    }
}

// Introduction to Result
mod resultintro {
    fn good_or_bad(good: bool) -> Result<i32, String> {
        if good {
            Ok(42)
        } else {
            Err("Bad".to_string())
        }
    }
    pub fn resultintro_examples() {
        println!("GOOD:: {:?}", good_or_bad(true));
        println!("BAD::: {:?}", good_or_bad(false));

        match good_or_bad(true) {
            Ok(n) => println!("Cool, I got {}", n),
            Err(e) => println!("Huh, I just got {}", e),
        }
    }
}

pub fn readingfiles_examples() {
    file1::file1_examples();
    resultintro::resultintro_examples();
}
