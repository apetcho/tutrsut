//! Learn immutable, mutable and type casting

mod add12 {
    pub fn add12_example() {
        let mut sum = 0;
        for i in 0..5 {
            sum += i;
        }
        println!("sum is {}", sum);
    }
}

pub fn addup_example() {
    add12::add12_example();
}
