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

pub fn structs_examples() {
    struct1::struct1_examples();
}
