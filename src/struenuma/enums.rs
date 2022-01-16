//! Simple Enums

mod enum2 {
    enum Speed {
        #[allow(dead_code)]
        Slow = 10,
        #[allow(dead_code)]
        Medium = 20,
        #[allow(dead_code)]
        Fast = 50,
    }

    // ---
    pub fn enum2_examples() {
        let s = Speed::Slow;
        let speed = s as u32;
        println!("speed = {}", speed);
    }
}

pub fn enums_examples() {
    enum2::enum2_examples();
}
