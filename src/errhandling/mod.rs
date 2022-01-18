//! Error Handling
mod basicserhdlr;

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

// ---
pub fn errhandling_examples() {
    errhdlr::errhdlr_example();
}
