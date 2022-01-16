//! More about matching

mod mam {
    pub fn mam_examples() {
        let t = (10, "Rust".to_string());
        let (ref n, ref s) = t;
        println!("=> n={}, s={}", n, s);
        let (n, s) = t;
        println!("=> n={}, s={}", n, s);
        // -- Structurin struct
        struct Point {
            x: f32,
            y: f32,
        }
        let p = Point { x: 1.0, y: 2.0 };
        let Point { x, y } = p;
        println!("=> x={}, y={}", x, y);
    }
}

pub fn morematches_examples() {
    mam::mam_examples();
}
