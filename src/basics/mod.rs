mod addup;
mod arrslice;
mod cmdargs;
mod findrope;
mod funtype;
mod hello;
mod iterators;
mod loopif;
mod matching;
mod morevecs;
mod optvals;
mod readingfiles;
mod slicedice;
mod strings;
mod vectors;

//

pub mod basictut {
    //use super::basics;
    //mod basics;
    use super::*;

    /// Basics tutorial
    fn basics_tutorial() {
        hello::hello_example();
    }

    /// Learn about loop and conditionals
    fn loopif_example() {
        loopif::loop_and_if_example();
    }

    /// Learn about immutable, mutable and type castin
    fn immcast_example() {
        addup::addup_example();
    }

    /// Functions
    fn funtype_examples() {
        funtype::fun_examples();
    }

    /// Import and include-like stuff
    fn include_import_stuff() {
        findrope::findrope_examples();
    }

    // Arrays and Slices
    fn arrays_and_slices() {
        arrslice::arrslice_example();
    }

    // Slicing and Dicing
    fn slicing_and_dicing() {
        slicedice::slicedice_examples();
    }

    // Optional Values
    fn optional_values() {
        optvals::optional_values_examples();
    }

    // Vectors::1
    fn vectors() {
        vectors::vectors_examples();
    }

    // Iterators
    fn iterators() {
        iterators::iterators_examples();
    }

    // More about vectors ...
    fn more_about_vectors() {
        morevecs::morevecs_examples();
    }

    // Strings
    fn strings_tut() {
        strings::strings_examples();
    }

    // Getting Command Line Arguments
    fn get_cmd_args() {
        cmdargs::cmdargs_examples();
    }

    // Matching
    fn matching_tut() {
        matching::matching_examples();
    }

    // Reading From Files
    fn reading_from_files() {
        readingfiles::readingfiles_examples();
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
        println!("------------------");
        println!("Reading From Files");
        println!("------------------");
        reading_from_files();
    }
}
