//! Learning where to find ropes
mod import {
    use std::f64::consts;

    pub fn import_example() {
        let x = 2.0 * consts::PI;
        let abs_diff = (x.cos() - 1.0).abs();
        assert!(abs_diff < 1e-10);
    }
}

pub fn findrope_examples() {
    import::import_example();
}
