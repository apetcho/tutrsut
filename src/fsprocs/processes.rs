//! Processes

use std::process::Command;

// --
mod proc1 {
    use super::*;

    //
    pub fn proc1_examples() {
        let status = Command::new("rustc").arg("-V").status().expect("no rustc?");

        println!("COOL {} CODE {}", status.success(), status.code().unwrap());
    }
}

pub fn processes_examples() {
    proc1::proc1_examples();
}
