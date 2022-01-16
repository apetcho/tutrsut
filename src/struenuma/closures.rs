//! Closures

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
    }
}

pub fn closures_examples() {
    closurestut::closurestut_examples();
}
