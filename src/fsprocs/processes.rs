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

// ---
mod proc2 {
    use super::*;

    // ---
    pub fn proc2_examples() {
        let output = Command::new("rustc").arg("-V").output().expect("no rustc");

        if output.status.success() {
            println!("Ok!");
        }

        println!(
            "len stdout {} stderr {}",
            output.stdout.len(),
            output.stderr.len()
        );
    }
}

// ---
pub fn processes_examples() {
    proc1::proc1_examples();
    proc2::proc2_examples();
}
