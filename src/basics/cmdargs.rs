//! Getting Command Line Arguments

mod arg0 {
    pub fn arg0_examples() {
        for arg in std::env::args() {
            println!("'{}'", arg);
        }
    }
}

pub fn cmdargs_examples() {
    arg0::arg0_examples();
}
