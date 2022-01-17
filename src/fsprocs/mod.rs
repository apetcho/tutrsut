//! Filesystem and Processes
mod fsreadfiles;

mod tutorial {
    use super::*;
    // Another look at Reading Files
    fn fsreadfilestut() {
        fsreadfiles::fsreadfiles_examples();
        println!("");
    }
    // Writing To Files
    // Files, Paths and Directories
    // Processes

    // ----
    pub fn fsprocstut() {
        println!("-----------------------------");
        println!("Another look at Reading Files");
        println!("-----------------------------");
        fsreadfilestut();
    }
}

// ----------------
// Main Entry Point
// ----------------
pub fn fsprocs_examples() {
    tutorial::fsprocstut();
}
