//! Function types are explicit

/// User-defined function
mod fun1 {
    fn sqr(x: f64) -> f64 {
        return x * x;
    }
    pub fn fun1_example() {
        let res = sqr(2.0);
        println!("square is {}", res);
    }
}

pub fn fun_examples() {
    fun1::fun1_example();
}
