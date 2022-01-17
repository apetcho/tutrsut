//! Filesystem and Processes
mod fsfpathdirs;
mod fsreadfiles;
mod fswritefiles;
mod processes;

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
    fn fsfpathdirstut() {
        fsfpathdirs::fsfpathdirs_examples();
        println!("");
    }
    // Processes
    fn processestut() {
        processes::processes_examples();
        println!("");
    }

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
        println!("----------------------------------");
        println!("  FILES, PATHS, AND DIRECTORIES   ");
        println!("----------------------------------");
        fsfpathdirstut();
        println!("---------------");
        println!("   PROCESSES   ");
        println!("---------------");
        processestut();
    }
}

// ----------------
// Main Entry Point
// ----------------
pub fn fsprocs_examples() {
    tutorial::fsprocstut();
}
