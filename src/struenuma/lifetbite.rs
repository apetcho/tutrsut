//! Lifetime Start to Bite

mod life12 {
    #[derive(Debug)]
    struct A {
        #[allow(dead_code)]
        s: &'static str,
    }

    pub fn life12_examples() {
        let a = A { s: "Hello Rust!" };
        println!("{:?}", a);
    }
}

pub fn lifetbite_examples() {
    life12::life12_examples();
}
