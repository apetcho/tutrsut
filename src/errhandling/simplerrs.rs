//! Simple Errors (simple-errors)

//#[macro_use]
//extern crate simple_error;
use simple_error::SimpleError;

mod demo{
    use super::*;
    //
    fn do_foo() -> Result<(), SimpleError>{
        Err(SimpleError::new("cannot do foo"))
    }

    pub fn demo_examples(){
        match do_foo(){
            Err(msg) => println!("{}", msg),
            Ok(_) => println!("Cool")
        }
    }
}


pub fn simple_error_examples(){
    demo::demo_examples();
}
