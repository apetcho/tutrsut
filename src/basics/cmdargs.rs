//! Getting Command Line Arguments

mod args0 {
    pub fn args0_examples() {
        for arg in std::env::args() {
            println!("'{}'", arg);
        }
    }
}

// nth() and expect()
mod args1 {
    use std::env;

    pub fn args1_examples() {
        let first = env::args().nth(1).expect("please supply an argument");
        let n: i32 = first.parse().expect("not an integer");
        println!("argv[1] = {}", n);
    }
}

pub fn cmdargs_examples() {
    args0::args0_examples();
    args1::args1_examples();
}
