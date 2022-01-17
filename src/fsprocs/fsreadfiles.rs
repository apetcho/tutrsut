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

    // ***
    pub fn file1_examples() {
        let _r = read_all_lines(FNAME);
    }
}

// file2
mod file2 {
    use super::*;
    // ---
    fn read_all_lines() -> io::Result<()> {
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
    pub fn file2_examples() {
        let _r = read_all_lines();
    }
}

// ---
mod file3 {
    use super::*;

    struct Lines<R> {
        reader: io::BufReader<R>,
        buf: String,
    }

    //
    impl<R: Read> Lines<R> {
        fn new(r: R) -> Lines<R> {
            Lines {
                reader: io::BufReader::new(r),
                buf: String::new(),
            }
        }

        // --
        fn next<'a>(&'a mut self) -> Option<io::Result<&'a str>> {
            self.buf.clear();
            match self.reader.read_line(&mut self.buf) {
                Ok(nbytes) => {
                    if nbytes == 0 {
                        None // no more lines!
                    } else {
                        let line = self.buf.trim_end();
                        Some(Ok(line))
                    }
                }
                Err(e) => Some(Err(e)),
            }
        }
    } // end impl

    // read_all_lines()
    fn read_all_lines(fname: &str) -> io::Result<()> {
        println!("");
        let file = File::open(&fname)?;

        let mut lines = Lines::new(file);
        while let Some(line) = lines.next() {
            let line = line?;
            println!("3> {}", line);
        }

        Ok(())
    }

    pub fn file3_examples() {
        let _r = read_all_lines(FNAME);
    }
}

// ---
pub fn fsreadfiles_examples() {
    file1::file1_examples();
    file2::file2_examples();
    file3::file3_examples();
}
