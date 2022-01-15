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

// to_string() and format!
mod string6 {
    fn array_to_str(arr: &[i32]) -> String {
        let mut res = '['.to_string();
        for v in arr {
            res += &v.to_string();
            res.push(',');
        }
        res.pop();
        res.push(']');
        res
    }
    pub fn string6_examples() {
        let arr = array_to_str(&[10, 20, 30]);
        let res = format!("arr = {}", arr);
        println!("{}", res);
    }
}

pub fn strings_examples() {
    string1::string1_examples();
    string5::string5_examples();
    string6::string6_examples();
}
