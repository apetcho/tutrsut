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

/// More user-defined functions
mod fun2 {
    // Passy by reference
    fn by_ref(x: &i32) -> i32 {
        *x + 1
    }

    pub fn fun2_examples() {
        let absval = abs(-2.0);
        assert_eq!(absval, 2.0);
        assert_eq!(clamp(-1.0, 0.0, 1.0), 0.0);
        assert_eq!(factorial(3), 6);

        let i = 10;
        let res1 = by_ref(&i);
        let res2 = by_ref(&42);
        println!("{} {}", res1, res2);
    }

    // absolute value of a floating-point number
    fn abs(x: f64) -> f64 {
        if x > 0.0 {
            x
        } else {
            -x
        }
    }

    // ensure the number always falls in the given range
    fn clamp(x: f64, x1: f64, x2: f64) -> f64 {
        if x < x1 {
            x1
        } else if x > x2 {
            x2
        } else {
            x
        }
    }

    // recursive factorial
    fn factorial(n: u64) -> u64 {
        if n == 0 {
            1
        } else {
            n * factorial(n - 1)
        }
    }
}

// Pass mutable reference
mod fun3 {
    // mutable references
    fn modifies(x: &mut f64) {
        *x = 1.0;
    }

    pub fn fun3_example() {
        let mut res = 0.0;
        println!("Before: res = {}", res);
        modifies(&mut res);
        println!("After : res = {}", res);
    }
}

// entry point
pub fn fun_examples() {
    fun1::fun1_example();
    println!("----- Pass by reference -----");
    fun2::fun2_examples();
    println!("----- Pass by mutable reference -----");
    fun3::fun3_example();
}
