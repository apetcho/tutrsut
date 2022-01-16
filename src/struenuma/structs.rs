//! Structs

mod struct1 {
    struct Person {
        first_name: String,
        last_name: String,
    }

    pub fn struct1_examples() {
        let p = Person {
            first_name: "John".to_string(),
            last_name: "Smith".to_string(),
        };
        println!("person: {} {}", p.first_name, p.last_name);
    }
}

// Associated function and impl
mod struct2 {
    struct Person {
        first_name: String,
        last_name: String,
    }
    // impl
    impl Person {
        fn new(first: &str, last: &str) -> Person {
            Person {
                first_name: first.to_string(),
                last_name: last.to_string(),
            }
        }

        //
        fn full_name(&self) -> String {
            format!("{} {}", self.first_name, self.last_name)
        }

        // Self, copy()
        fn copy(&self) -> Self {
            Self::new(&self.first_name, &self.last_name)
        }

        // modifiying through &mut self
        fn set_first_name(&mut self, name: &str) {
            self.first_name = name.to_string();
        }
        // moving or not moving
        fn to_tuple(self) -> (String, String) {
            (self.first_name, self.last_name)
        }
    }

    // -- using Person
    pub fn struct2_examples() {
        let p = Person::new("John", "Smith");
        println!("person: {} {}", p.first_name, p.last_name);
        println!("full_name(): {}", p.full_name());
        let pcopy = p.copy();
        println!("copy(): {}", pcopy.full_name());
        let mut pcopy2 = pcopy.copy();
        println!("to_tuple(): {:?}", pcopy.to_tuple());
        pcopy2.set_first_name("Cooper");
        println!("set_first_name(): {}", pcopy2.full_name());
        //println!("first_name: {}", pcopy.first_name); // Error: pcopy has
        // .. moved
    }
}

// using #[derive(Debug)]
mod struct34 {
    #[allow(unused_imports)]
    use std::fmt;

    #[derive(Debug)]
    struct Person {
        first_name: String,
        last_name: String,
    }

    impl Person {
        fn new(first: &str, last: &str) -> Person {
            Person {
                first_name: first.to_string(),
                last_name: last.to_string(),
            }
        }

        fn full_name(&self) -> String {
            format!("{} {}", self.first_name, self.last_name)
        }

        fn set_first_name(&mut self, name: &str) {
            self.first_name = name.to_string();
        }

        fn to_tuple(self) -> (String, String) {
            (self.first_name, self.last_name)
        }
    }

    // --
    pub fn struct34_examples() {
        let mut p = Person::new("John", "Smith");
        println!("{:?}", p);
        p.set_first_name("Jane");
        println!("{:?}", p);
        println!("{}", p.full_name());
        println!("{:?}", p.to_tuple());
    }
}

pub fn structs_examples() {
    struct1::struct1_examples();
    struct2::struct2_examples();
    struct34::struct34_examples();
}
