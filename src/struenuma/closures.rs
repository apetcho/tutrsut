//! Closures
//mod iterfltrange::{};

mod closurestut {
    fn apply<F>(x: f64, f: F) -> f64
    where
        F: Fn(f64) -> f64,
    {
        f(x)
    }

    // --
    fn mutate<F>(mut f: F)
    where
        F: FnMut(),
    {
        f()
    }

    // --
    pub fn closurestut_examples() {
        let f = |x| x * x;
        let res = f(10);
        println!("res={}", res);
        // let resf = f(1.2);  // #Error:: mismatch type
        let m = 2.0;
        let c = 1.0;
        let lin = |x| m * x + c;
        println!("reslin: {} {}", lin(1.0), lin(2.0));
        let res1 = apply(3.0, lin);
        let res2 = apply(3.14, |x| x.sin());
        println!("res1={}, res2={}", res1, res2);
        // -- mutate
        let mut s = "world";
        println!("Before: s = {}", s);
        mutate(|| s = "Hello");
        println!("After : s = {}", s);
        assert_eq!(s, "Hello");
        // -- closure and mutable borrow
        let mut s = "World";
        println!("Before:: changer(): s= {}", s);
        let mut changer = || s = "Rust";
        changer();
        println!("After :: changer(): s= {}", s);
        let lang = "C++";
        {
            println!("Before:: changer(): lang = {}", lang);
            let mut changer = || s = "Rust";
            changer();
            println!("Before:: changer(): lang = {}", lang);
        }
        assert_eq!(lang, "C++");
        // --
        let name = "John Doe".to_string();
        let age = 45;
        let clos = move || {
            println!("name={}, age={}", name, age);
        };
        clos();
        // println!("name={}", name); // error: name has been moved
        // --
        let tuples = [(10, "ten"), (20, "twenty"), (30, "thirty"), (40, "forty")];
        let iter = tuples.iter().filter(|t| t.0 > 20).map(|t| t.1);
        for name in iter {
            println!("=> name={}", name);
        }
    }
}

pub fn closures_examples() {
    closurestut::closurestut_examples();
}
