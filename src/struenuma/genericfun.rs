//! Generic Functions

mod gen1 {
    fn sqr<T>(x: T) -> T::Output
    where
        T: std::ops::Mul + Copy,
    {
        x * x
    }

    pub fn gen1_examples() {
        let x = sqr(3);
        let y = sqr(3.0);
        println!("x={}, y={}", x, y);
    }
}

pub fn genericfun_examples() {
    gen1::gen1_examples();
}
