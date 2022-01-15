//! Reading From Files
use std::env;
use std::fs::File;
use std::io;
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

// Using Result
mod file2 {
    use super::{env, io, File, Read};
    fn read_to_string(filename: &str) -> Result<String, io::Error> {
        let mut file = match File::open(&filename) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let mut text = String::new();
        match file.read_to_string(&mut text) {
            Ok(_) => Ok(text),
            Err(e) => Err(e),
        }
    }
    pub fn file2_examples() {
        let file = env::args().nth(2).expect("Please supply a filename");
        let text = read_to_string(&file).expect("Bad file man!");
        println!("{} had {} bytes", file, text.len());
    }
}

// io::Result<T>
mod file3 {
    use super::{env, io, File, Read};

    fn read_to_string(filename: &str) -> io::Result<String> {
        let mut file = File::open(filename)?;
        let mut text = String::new();
        file.read_to_string(&mut text)?;
        Ok(text)
    }
    pub fn file3_examples() {
        let file = env::args().nth(2).expect("Please supply a filename");
        let text = read_to_string(&file).expect("Bad file man!");
        println!("{} had {} bytes", file, text.len())
    }
}

pub fn readingfiles_examples() {
    file1::file1_examples();
    resultintro::resultintro_examples();
    file2::file2_examples();
    file3::file3_examples();
}
