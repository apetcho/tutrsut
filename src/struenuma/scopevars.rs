//! Scope of Variables

mod scope {
    pub fn scope_examples() {
        // Rust is a block-scoped language
        {
            let _a = 10;
            let _b = "Hello";
            {
                let _c = "Hello".to_string();
                // _a, _b and _c are visible
            }
            // the string _c is dropped
            // _a, and _b are visible
            for _i in 0.._a {
                let _b = &_b[1..];
                // original b is no longer visible - it is shadowd.
            }
            // the slice _b is dropped
            // _i is _not_ visible
        }
    }
}

mod ref1 {
    pub fn ref1_examples() {
        let s1 = "Hello Rust!".to_string();
        let mut rs1 = &s1;
        println!("ref: {}", rs1);
        {
            let _tmp = "Hello C and C++!".to_string();
            //rs1 = &_tmp;
        } // _tmp dropped here while still borrowed
          // - borrowed value needs to live until
        let tmp = "Hello C and C++ buddies".to_string();
        rs1 = &tmp;
        println!("ref: {}", rs1);
        // here
    }
}

pub fn scopevars_examples() {
    scope::scope_examples();
    ref1::ref1_examples();
}
