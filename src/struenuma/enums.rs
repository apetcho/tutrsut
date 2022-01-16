//! Simple Enums

mod enum12 {
    enum Speed {
        #[allow(dead_code)]
        Slow = 10,
        #[allow(dead_code)]
        Medium = 20,
        #[allow(dead_code)]
        Fast = 50,
    }

    // ---
    pub fn enum12_examples() {
        let s = Speed::Slow;
        let speed = s as u32;
        println!("speed = {}", speed);
    }
}

// --
mod enum3 {
    #[derive(Debug)]
    enum Value {
        Number(f64),
        Str(String),
        Bool(bool),
    }

    // --
    fn eat_and_dump(v: Value) {
        use Value::*;
        match v {
            Number(n) => println!("= number is {}", n),
            Str(s) => println!("= string is '{}'", s),
            Bool(b) => println!("= boolean is {}", b),
        }
    }

    // --
    fn dump(v: &Value) {
        use Value::*;
        match *v {
            Number(n) => println!("* number is {}", n),
            Str(ref s) => println!("* string is '{}'", s),
            Bool(b) => println!("* boolean is {}", b),
        }
    }

    impl Value {
        fn to_str(self) -> Option<String> {
            if let Value::Str(s) = self {
                Some(s)
            } else {
                None
            }
        }
    }

    pub fn enum3_examples() {
        use Value::*;
        let n = Number(2.3);
        let s = Str("Hello".to_string());
        let b = Bool(true);
        dump(&n);
        dump(&s);
        dump(&b);
        println!("n = {:?}, s = {:?}, b = {:?}", n, s, b);
        eat_and_dump(n);
        eat_and_dump(s);
        eat_and_dump(b);
        let msg = Value::Str("Rust is Cool".to_string());
        println!("=msg := {:?}", msg.to_str());
    }
}

pub fn enums_examples() {
    enum12::enum12_examples();
    enum3::enum3_examples();
}
