//! Strings

mod string1 {
    fn dump(s: &str) {
        println!("str = '{}'", s);
    }

    pub fn string1_examples() {
        let text = "Hello Rust";
        let s = text.to_string();
        dump(text);
        dump(&s);
    }
}

// push and pop on String object
mod string5 {
    pub fn string5_examples() {
        let mut s = String::new();
        s.push('H');
        s.push_str("ello");
        s.push(' ');
        s += "Rust String!";
        println!("s = {}", s);
        println!("s.pop() = {:?}", s.pop());
        println!("s = {}", s);
    }
}

pub fn strings_examples() {
    string1::string1_examples();
    string5::string5_examples();
}
