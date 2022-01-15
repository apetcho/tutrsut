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

    // Optional Values
    fn optional_values() {
        basics::optvals::optional_values_examples();
    }

    // Vectors::1
    fn vectors() {
        basics::vectors::vectors_examples();
    }

    // Iterators
    fn iterators() {
        basics::iterators::iterators_examples();
    }

    // More about vectors ...
    fn more_about_vectors() {
        basics::morevecs::morevecs_examples();
    }

    // Strings
    fn strings_tut() {
        basics::strings::strings_examples();
    }

    // Getting Command Line Arguments
    fn get_cmd_args() {
        basics::cmdargs::cmdargs_examples();
    }

    // Matching
    fn matching_tut() {
        basics::matching::matching_examples();
    }

    // ***********
    // entry point
    // ***********
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
        println!("---------------");
        println!("Optional Values");
        println!("---------------");
        optional_values();
        println!("-------");
        println!("Vectors");
        println!("-------");
        vectors();
        println!("---------");
        println!("Iterators");
        println!("---------");
        iterators();
        println!("------------------");
        println!("More about vectors");
        println!("------------------");
        more_about_vectors();
        println!("-------");
        println!("Strings");
        println!("-------");
        strings_tut();
        println!("------------------------------");
        println!("Getting Command Line Arguments");
        println!("------------------------------");
        get_cmd_args();
        println!("--------");
        println!("Matching");
        println!("--------");
        matching_tut();
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
