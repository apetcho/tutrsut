//! Filesystem and Processes
mod fsreadfiles;
mod fswritefiles;

mod tutorial {
    use super::*;
    // Another look at Reading Files
    fn fsreadfilestut() {
        fsreadfiles::fsreadfiles_examples();
        println!("");
    }
    // Writing To Files
    fn fswritefilestut() {
        fswritefiles::fswritefiles_examples();
        println!("");
    }
    // Files, Paths and Directories
    // Processes

    // ----
    pub fn fsprocstut() {
        println!("-----------------------------------");
        println!("   ANOTHER LOOK AT READING FILES   ");
        println!("-----------------------------------");
        fsreadfilestut();
        println!("----------------------");
        println!("   WRITING TO FILES   ");
        println!("----------------------");
        fswritefilestut();
    }
}

// ----------------
// Main Entry Point
// ----------------
pub fn fsprocs_examples() {
    tutorial::fsprocstut();
}
