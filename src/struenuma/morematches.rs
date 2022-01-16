//! More about matching

mod mam {
    pub fn mam_examples() {
        let t = (10, "Rust".to_string());
        let (ref n, ref s) = t;
        println!("=> n = {}, s = {}", n, s);
        let (n, s) = t;
        println!("=> n = {}, s = {}", n, s);
    }
}

pub fn morematches_examples() {
    mam::mam_examples();
}
