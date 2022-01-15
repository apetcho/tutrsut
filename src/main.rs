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

    /// Functions
    fn funtype_examples() {
        basics::funtype::fun_examples();
    }

    // entry point
    pub fn basictut_examples() {
        println!("---------------");
        println!("Basics tutorial");
        println!("---------------");
        basics_tutorial();
        println!("----------------------");
        println!("Loops and conditionals");
        println!("----------------------");
        loopif_example();
        println!("------------------------------------");
        println!("Immutable, mutable, and type casting");
        println!("------------------------------------");
        immcast_example();
        println!("----------------------");
        println!("User-defined functions");
        println!("----------------------");
        funtype_examples();
    }
}

// -----------
// MAIN DRIVER
// -----------
fn main() {
    println!("============");
    println!("   BASICS   ");
    println!("============");
    basictut::basictut_examples();
}
