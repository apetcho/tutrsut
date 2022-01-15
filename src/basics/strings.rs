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

// slice and &str and String
mod string2 {
    pub fn string2_examples() {
        let text = "static";
        let string = "dynamic".to_string();

        let text_s = &text[1..];
        let string_s = &string[2..4];

        println!("slices {:?} {:?}", text_s, string_s);
    }
}

// strings and UTF-8
mod string3 {
    pub fn string3_examples() {
        let multilingual = "Good glück. Voilà";
        println!("text = {}", multilingual);
        for ch in multilingual.chars() {
            print!("'{}' ", ch);
        }
        println!("");
        println!("len = {}", multilingual.len());
        println!("count = {}", multilingual.chars().count());

        let maybe = multilingual.find('ü');
        if maybe.is_some() {
            let hi = &multilingual[maybe.unwrap()..];
            println!("From 'ü' onward: {}", hi);
        }
    }
}

// split_whitespace() and collect
mod string4 {
    pub fn string4_examples() {
        let text = "The red fox and the lazy dog";
        println!("text = {}", text);
        let words: Vec<&str> = text.split_whitespace().collect();
        println!("words = {:?}", words);
        let mut words = Vec::new();
        words.extend(text.split_whitespace());
        println!("words := {:?}", words);

        let stripped: String = text.chars().filter(|ch| !ch.is_whitespace()).collect();
        println!("stripped: {}", stripped);
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
    string2::string2_examples();
    string3::string3_examples();
    string4::string4_examples();
    string5::string5_examples();
    string6::string6_examples();
}
