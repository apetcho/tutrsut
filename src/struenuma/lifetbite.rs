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

// Using Lifetime
mod life3 {
    #[derive(Debug)]
    struct A<'a> {
        #[allow(dead_code)]
        s: &'a str,
    }

    pub fn life3_examples() {
        let s = "I'm a little string".to_string();
        let a = A { s: &s };
        println!("{:?}", a);
    }
}

pub fn lifetbite_examples() {
    life12::life12_examples();
    life3::life3_examples();
}
