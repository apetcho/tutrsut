//! Simple Errors (simple-errors)

//#[macro_use]
//extern crate simple_error;
use simple_error::SimpleError;

mod demo {
    use super::*;
    //
    fn do_foo() -> Result<(), SimpleError> {
        Err(SimpleError::new("cannot do foo"))
    }

    pub fn demo_examples() {
        match do_foo() {
            Err(msg) => println!("{}", msg),
            Ok(_) => println!("Cool"),
        }
    }
}

// ---
mod simerr1 {
    //use super::*;
    use std::error::Error;
    type BoxResult<T> = Result<T, Box<dyn Error>>;

    fn run(s: &str) -> BoxResult<i32> {
        if s.len() == 0 {
            bail!("empty string");
        }
        Ok(s.trim().parse()?)
    }

    pub fn simerr1_examples() {
        println!("{:?}", run("23"));
        println!("{:?}", run("2x"));
        println!("{:?}", run(""));
    }
}

pub fn simple_error_examples() {
    demo::demo_examples();
    simerr1::simerr1_examples();
}
