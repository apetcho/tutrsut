//! Closures

mod closurestut {
    pub fn closurestut_examples() {
        let f = |x| x * x;
        let res = f(10);
        println!("res={}", res);
    }
}

pub fn closures_examples() {
    closurestut::closurestut_examples();
}
