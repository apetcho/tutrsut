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

// --
mod erchn {
    use super::*;

    //
    pub fn erchn_examples() {
        errorchain::errorchain_examples();
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
    println!("------------------------------------");
    println!("   ERROR-CHAIN FOR SERIOUS ERRORS   ");
    println!("------------------------------------");
    erchn::erchn_examples();
}
