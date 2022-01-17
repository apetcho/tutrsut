//! Writing to Files
use std::fs::File;
use std::io;
use std::io::prelude::*;

//
mod wrfile1 {
    use super::*;

    // --
    fn write_out(f: &str) -> io::Result<()> {
        let mut out = File::create(f)?;
        write!(out, "answer is {}\n", 42)?;

        Ok(())
    }

    //
    pub fn wrfile1_examples() {
        let _r = write_out("test.txt").expect("Write failed");
        println!("Wrote to test.txt");
    }
}

// ---
pub fn fswritefiles_examples() {
    wrfile1::wrfile1_examples();
}
