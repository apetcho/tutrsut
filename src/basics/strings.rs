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

pub fn strings_examples() {
    string1::string1_examples();
}
