//! Learn about loops and ifs

mod for1 {
    pub fn for1_example() {
        for i in 0..5 {
            println!("Hello {}", i);
        }
    }
}

pub fn loop_and_if_example() {
    for1::for1_example();
}
