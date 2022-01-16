//! Traits

mod trait1 {
    trait Show {
        fn show(&self) -> String;
    }

    impl Show for i32 {
        fn show(&self) -> String {
            format!("Four-byte signed: {}", self)
        }
    }

    impl Show for f64 {
        fn show(&self) -> String {
            format!("Eight-bytes float: {}", self)
        }
    }

    // ***
    pub fn trait1_examples() {
        let answer = 42;
        let maybe_pi = 3.14;
        let s1 = answer.show();
        let s2 = maybe_pi.show();
        println!("show:: {}", s1);
        println!("show:: {}", s2);
    }
}

pub fn traits_examples() {
    trait1::trait1_examples();
}
