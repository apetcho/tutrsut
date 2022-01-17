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
            println!("1> {}", line);
        }
        Ok(())
    }

    // ---
    fn read_all_lines2() -> io::Result<()> {
        println!("");
        let file = File::open(FNAME)?;
        let mut reader = io::BufReader::new(file);
        let mut buf = String::new();
        while reader.read_line(&mut buf)? > 0 {
            {
                let line = buf.trim_end(); // trim_right() is deprecated
                println!("2> {}", line);
            }
            buf.clear();
        }

        Ok(())
    }

    // ***
    pub fn file1_examples() {
        //let fname: &str = FNAME;
        let _r = read_all_lines(FNAME);
        let _r = read_all_lines2();
        //Ok(())
    }
}

pub fn fsreadfiles_examples() {
    file1::file1_examples();
}
