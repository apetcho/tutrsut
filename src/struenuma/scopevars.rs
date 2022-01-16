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

pub fn scopevars_examples() {
    scope::scope_examples();
}
