//! Error Handling
mod basicserhdlr;
mod errorchain;
mod simplerrs;

// ---
mod errhdlr {
    use super::*;

    fn basics_error_tut() {
        basicserhdlr::basics_error_handling();
    }

    // ---
    pub fn errhdlr_example() {
        basics_error_tut();
    }
}

// --
mod simerr {
    use super::simplerrs;

    pub fn simerr_examples() {
        simplerrs::simple_error_examples();
    }
}

// ---
pub fn errhandling_examples() {
    println!("--------------------------");
    println!("   BASIC ERROR HANDLING   ");
    println!("--------------------------");
    errhdlr::errhdlr_example();
    println!("------------------------------------");
    println!("   SIMPLE-ERROR FOR SIMPLE ERRORS   ");
    println!("------------------------------------");
    simerr::simerr_examples();
}
