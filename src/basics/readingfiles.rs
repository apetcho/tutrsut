//! Reading From Files
use std::env;
use std::fs::File;
use std::io::Read;

mod file1 {
    use super::env;
    use super::File;
    use super::Read;
    pub fn file1_examples() {
        let second = env::args().nth(2).expect("please supply a filename");
        let mut file = File::open(&second).expect("Can't open the file");
        let mut text = String::new();
        file.read_to_string(&mut text).expect("Can't read the file");
        println!("file had {} bytes", text.len());
    }
}

pub fn readingfiles_examples() {
    file1::file1_examples();
}
