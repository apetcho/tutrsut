//! Learn about loops and ifs

// for loop
mod for1 {
    pub fn for1_example() {
        for i in 0..5 {
            println!("Hello {}", i);
        }
    }
}

// for loop with conditional
mod for2 {
    pub fn for2_example() {
        for i in 0..5 {
            if i % 2 == 0 {
                println!("even {}", i);
            } else {
                println!("odd {}", i);
            }
        }
    }
}

pub fn loop_and_if_example() {
    for1::for1_example();
    for2::for2_example();
}
