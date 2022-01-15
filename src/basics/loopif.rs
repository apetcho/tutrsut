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

// Tenary operator-like
mod for3 {
    pub fn for3_example() {
        for i in 0..5 {
            let even_odd = if i % 2 == 0 { "even" } else { "odd" };
            println!("{} {}", even_odd, i);
        }
    }
}

pub fn loop_and_if_example() {
    for1::for1_example();
    for2::for2_example();
    for3::for3_example();
}
