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
    }
}

pub fn tuples_examples() {
    tuple1::tuple1_examples();
}
