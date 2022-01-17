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
mod proc3 {
    use super::*;

    //
    fn shell(cmd: &str) -> (String, bool) {
        let cmd = format!("{} 2>&1", cmd);
        let shell = if cfg!(windows) { "cmd.exe" } else { "/bin/sh" };
        let flag = if cfg!(windows) { "/c" } else { "-c" };
        let output = Command::new(shell)
            .arg(flag)
            .arg(&cmd)
            .output()
            .expect("no shell?");

        (
            String::from_utf8_lossy(&output.stdout)
                .trim_end()
                .to_string(),
            output.status.success(),
        )
    }

    // ---
    fn shell_success(cmd: &str) -> Option<String> {
        let (output, success) = shell(cmd);
        if success {
            Some(output)
        } else {
            None
        }
    }

    //
    pub fn proc3_examples() {
        let cmd = "ls -lh";
        match shell_success(cmd) {
            Some(out) => println!("{}", out),
            None => println!(""),
        }
    }
}

// ---
pub fn processes_examples() {
    proc1::proc1_examples();
    proc2::proc2_examples();
    proc3::proc3_examples();
}
