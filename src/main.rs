//mod basics;
//mod struenuma;
//mod fsprocs;
mod errhandling;
#[allow(unused_imports)]
#[macro_use]
extern crate simple_error;
#[macro_use]
extern crate error_chain;

// -----------
// MAIN DRIVER
// -----------
fn main() {
    //println!("============");
    //println!("   BASICS   ");
    //println!("============");
    //basics::basictut::basictut_examples();

    //println!("===========================");
    //println!("   STRUCTS, ENUMS, MATCH   ");
    //println!("===========================");
    //struenuma::tutorial::structs_enums_and_matching();

    //println!("==============================");
    //println!("   FILESYSTEM AND PROCESSES   ");
    //println!("==============================");
    //fsprocs::fsprocs_examples();

    println!("=====================");
    println!("   ERROR HANDLING    ");
    println!("=====================");
    errhandling::errhandling_examples();
}
