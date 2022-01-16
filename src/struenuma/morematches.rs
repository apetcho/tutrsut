//! More about matching

mod mam {
    // --
    fn match_tuple(t: (i32, String)) {
        let text = match t {
            (0, s) => format!("zero {}", s),
            (1, ref s) if s == "hello" => format!("hello one!"),
            tt => format!("no match {:?}", tt),
        };

        println!("{}", text);
    }

    // --
    pub fn mam_examples() {
        let t = (10, "Rust".to_string());
        let (ref n, ref s) = t;
        println!("=> n={}, s={}", n, s);
        let (n, s) = t;
        println!("=> n={}, s={}", n, s);
        // -- Structurin struct
        struct Point {
            x: f32,
            y: f32,
        }
        let p = Point { x: 1.0, y: 2.0 };
        let Point { x, y } = p;
        println!("=> x={}, y={}", x, y);
        // --
        match_tuple((0, "None".to_string()));
        // --
        let ot = Some((2, "hello".to_string()));
        if let Some((_, ref s)) = ot {
            assert_eq!(s, "hello");
        }

        if let Ok(n) = "42".parse::<i32>() {
            println!("=> n={}", n);
        }
        let n: i32 = "42".parse().unwrap();
        println!("=> n={}", n);
    }
}

pub fn morematches_examples() {
    mam::mam_examples();
}
