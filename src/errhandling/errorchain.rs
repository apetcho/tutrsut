//! Error-chain for Serious Errors
use error_chain;

static MY_FILE: &str = "/Users/oriprox/Documents/cs/rust/practice/tutrust/test.txt";

mod errors {
    use super::*;

    error_chain! {
        foreign_links{
            Io(::std::io::Error);
        }

        errors {
            NoArgument(t: String){
                display("no argument prodived: '{}'", t)
            }
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
            //std::process::exit(1);
        }

        // ---
        if let Err(e) = run() {
            match e.kind() {
                &ErrorKind::Msg(ref s) => println!("msg {}", s),
                &ErrorKind::Io(ref s) => println!("io {}", s),
                &ErrorKind::NoArgument(ref s) => {
                    println!("no argument {:?}", s);
                }
                _ => println!(),
            }
        }

        let _ = another_example();
    }
    // ---
    fn another_example() -> Result<()> {
        use std::env::args;
        use std::fs::File;
        use std::io::prelude::*;
        use std::io::BufReader;

        let file = args()
            .skip(1)
            .next()
            .ok_or(Error::from("filename needed"))?;

        // ---- chain explicitly ----
        let f = File::open(&file).chain_err(|| "unable to read the file")?;
        let mut lineno = 1;
        for line in BufReader::new(f).lines() {
            let line = line.chain_err(|| "cannot read a line")?;
            println!("[{}]-> {}", lineno, line);
            lineno += 1;
            if lineno == 10 {
                break;
            }
        }
        Ok(())
    }
    // ---
}
//
mod erchn {
    use super::*;

    pub fn erchn_examples() -> Result<()> {
        use std::env::args;
        use std::fs::File;
        use std::io::prelude::*;
        use std::io::BufReader;

        let file = args().skip(1).next().ok_or(Error::from("provide a file"))?;

        let f = File::open(&file)?;
        let mut l = 0;
        for line in BufReader::new(f).lines() {
            let line = line?;
            println!("=> {}", line);
            l += 1;
            if l == 10 {
                break;
            }
        }

        Ok(())
    }
}

// ---
pub fn errorchain_examples() {
    demo::demo_examples();
    let _ = erchn::erchn_examples();
}
