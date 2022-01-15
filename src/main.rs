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

    /// Import and include-like stuff
    fn include_import_stuff() {
        basics::findrope::findrope_examples();
    }

    // Arrays and Slices
    fn arrays_and_slices() {
        basics::arrslice::arrslice_example();
    }

    // Slicing and Dicing
    fn slicing_and_dicing() {
        basics::slicedice::slicedice_examples();
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
        println!("------------------------");
        println!("Import and Include Stuff");
        println!("------------------------");
        include_import_stuff();
        println!("-----------------");
        println!("Arrays and Slices");
        println!("-----------------");
        arrays_and_slices();
        println!("------------");
        println!("More slicing");
        println!("------------");
        slicing_and_dicing();
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
