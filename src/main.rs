mod basics;

mod basictut {
    use super::basics;
    //mod basics;

    /// Basics tutorial
    fn basics_tutorial() {
        basics::hello::hello_example();
    }

    /// Learn about loop and conditionals
    fn loopif_example() {
        basics::loopif::loop_and_if_example();
    }

    /// Learn about immutable, mutable and type castin
    fn immcast_example() {
        basics::addup::addup_example();
    }

    // entry point
    pub fn basictut_example() {
        basics_tutorial();
        loopif_example();
        immcast_example();
    }
}

// -----------
// MAIN DRIVER
// -----------
fn main() {
    basictut::basictut_example();
}
