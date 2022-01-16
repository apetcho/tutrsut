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

// --
mod trait2 {
    use std::fmt;

    struct Person {
        fname: String,
        lname: String,
    }

    impl Person {
        fn new(first: &str, last: &str) -> Person {
            Person {
                fname: first.to_string(),
                lname: last.to_string(),
            }
        }

        fn full_name(&self) -> String {
            format!("{} {}", self.fname, self.lname)
        }
    }

    impl fmt::Debug for Person {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.full_name())
        }
    }

    pub fn trait2_examples() {
        let p = Person::new("John", "Doe");
        println!("person: {:?}", p);
    }
}

pub fn traits_examples() {
    trait1::trait1_examples();
    trait2::trait2_examples();
}
