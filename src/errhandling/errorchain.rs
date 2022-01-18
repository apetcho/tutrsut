//! Error-chain for Serious Errors
use error_chain;

static MY_FILE: &str = "/Users/oriprox/Documents/cs/rust/practice/tutrust/test.txt";

mod errors {
    use super::*;

    error_chain! {
        foreign_links{
            Io(::std::io::Error);
        }
    }
}

use errors::*;

mod demo {
    use super::*;

    fn run() -> Result<()> {
        use std::fs::File;
        //File::open("file")?;
        File::open(MY_FILE)?;
        println!("COOL");
        Ok(())
    }

    pub fn demo_examples() {
        if let Err(e) = run() {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

pub fn errorchain_examples() {
    demo::demo_examples();
}
