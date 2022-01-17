//! Another look at Reading Files
use std::fs::File;
use std::io;
use std::io::prelude::*;

static FNAME: &str = "/usr/share/vim/vimrc";

// - file1
mod file1 {
    use super::*;

    // --
    fn read_all_lines(filename: &str) -> io::Result<()> {
        let file = File::open(&filename)?;

        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            println!("> {}", line);
        }
        Ok(())
    }

    // ***
    pub fn file1_examples() {
        //let fname: &str = FNAME;
        let _r = read_all_lines(FNAME);
        //Ok(())
    }
}

pub fn fsreadfiles_examples() {
    file1::file1_examples();
}
