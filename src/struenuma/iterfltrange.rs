//! Example: iterator over floating-point range

mod trait3 {
    struct FRange {
        val: f64,
        end: f64,
        incr: f64,
    }

    // -- range()
    fn range(x1: f64, x2: f64, skip: f64) -> FRange {
        FRange {
            val: x1,
            end: x2,
            incr: skip,
        }
    }

    // Iterator
    impl Iterator for FRange {
        type Item = f64;

        fn next(&mut self) -> Option<Self::Item> {
            let res = self.val;
            if res >= self.end {
                None
            } else {
                self.val += self.incr;
                Some(res)
            }
        }
    }

    // ***
    pub fn trait3_examples() {
        for x in range(0.0, 1.0, 0.1) {
            println!("{} ", x);
        }
    }
}

//
pub fn iterfltrange_examples() {
    trait3::trait3_examples();
}
