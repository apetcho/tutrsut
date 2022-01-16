//! Closures

mod closurestut {
    pub fn closurestut_examples() {
        let f = |x| x * x;
        let res = f(10);
        println!("res={}", res);
        // let resf = f(1.2);  // #Error:: mismatch type
        let m = 2.0;
        let c = 1.0;
        let lin = |x| m * x + c;
        println!("reslin: {} {}", lin(1.0), lin(2.0));
    }
}

pub fn closures_examples() {
    closurestut::closurestut_examples();
}
