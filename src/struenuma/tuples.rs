//! Tuples

mod tuple1 {
    fn add_mul(x: f64, y: f64) -> (f64, f64) {
        (x + y, x * y)
    }
    pub fn tuple1_examples() {
        let t = add_mul(2.0, 10.0);
        // can debug print
        println!("tuple: {:?}", t);
        // can 'index' the values
        println!("add: {}, mul: {}", t.0, t.1);
        // can _extract_ (unpack) the values
        let (add, mul) = t;
        println!("add: {}, mul: {}", add, mul);

        // --- Tuple may contain *different types*
        let tuple = ("Hello", 5, 'c');
        assert_eq!(tuple.0, "Hello");
        assert_eq!(tuple.1, 5);
        assert_eq!(tuple.2, 'c');

        // Iterator methods and Tuples
        for t in ["Fortran", "C", "C++", "Python", "Julia"]
            .iter()
            .enumerate()
        {
            println!("{} => {}", t.0, t.1);
        }

        let languages = ["Fortran", "C", "C++", "Rust"];
        let years = [1957, 1970, 1979, 2014];
        for yela in languages.iter().zip(years.iter()) {
            println!("{} => {}", yela.1, yela.0);
        }
    }
}

pub fn tuples_examples() {
    tuple1::tuple1_examples();
}
