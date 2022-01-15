//! Hello, World

mod hello {
    pub fn hello_world() {
        println!("Hello, World");
    }
}

/// First glace at keyword `let`
mod let1 {
    pub fn let1_example() {
        // introduce a variable
        let answer = 42;
        // format and output answer
        println!("Hello {}", answer);
    }
}

pub fn hello_example() {
    // Welcome to rust programming
    hello::hello_world();
    // Example: introduce a variable with `let`keyword
    let1::let1_example();
}
