mod closures;
mod enums;
mod genericfun;
mod genstructs;
mod iterfltrange;
mod lifetbite;
mod morematches;
mod moveit;
mod scopevars;
mod structs;
mod structsdyn;
mod traits;
mod tuples;

// ----
pub mod tutorial {
    use super::*;
    // Rust likes to move it, move it
    fn rust_likes_to_move_it() {
        moveit::moveit_examples();
        println!("");
    }

    // Scope of Variables
    fn scope_of_variables() {
        scopevars::scopevars_examples();
        println!("");
    }

    // Tuples
    fn tupletut() {
        tuples::tuples_examples();
        println!("");
    }

    // Structs
    fn structstut() {
        structs::structs_examples();
        println!("");
    }

    // Lifetime Start to Bite
    fn lifetimetut() {
        lifetbite::lifetbite_examples();
        println!("");
    }

    // Traits
    fn traitstut() {
        traits::traits_examples();
        println!("");
    }

    // Example: Iterator over floating-point range
    fn iterfltrangetut() {
        iterfltrange::iterfltrange_examples();
        println!("");
    }

    // Generic Functions
    fn genericfuntut() {
        genericfun::genericfun_examples();
        println!("");
    }

    // Simple Enums
    fn simple_enums() {
        enums::enums_examples();
        println!("");
    }

    // --
    fn more_about_matching() {
        morematches::morematches_examples();
        println!("");
    }

    // -- Closures
    fn closures_tut() {
        closures::closures_examples();
        println!("");
    }

    // Structs with Dynamic Data
    fn structsdyntut() {
        structsdyn::structsdyn_example();
        println!("");
    }

    // ---
    fn genstructstut() {
        genstructs::genstructs_examples();
        println!("");
    }

    // ***
    pub fn structs_enums_and_matching() {
        println!("-------------");
        println!("Move Semantic");
        println!("-------------");
        rust_likes_to_move_it();
        println!("------------------");
        println!("Scope of Variables");
        println!("------------------");
        scope_of_variables();
        println!("------");
        println!("Tuples");
        println!("------");
        tupletut();
        println!("-------");
        println!("Structs");
        println!("-------");
        structstut();
        println!("----------------------");
        println!("Lifetime Start to Bite");
        println!("----------------------");
        lifetimetut();
        println!("------");
        println!("Traits");
        println!("------");
        traitstut();
        println!("-------------------------------------------");
        println!("Example: Iterator over floating-point range");
        println!("-------------------------------------------");
        iterfltrangetut();
        println!("-----------------");
        println!("Generic Functions");
        println!("-----------------");
        genericfuntut();
        println!("------------");
        println!("Simple Enums");
        println!("------------");
        simple_enums();
        println!("-------------------");
        println!("More about Matching");
        println!("-------------------");
        more_about_matching();
        println!("--------");
        println!("Closures");
        println!("--------");
        closures_tut();
        println!("-------------------------");
        println!("Structs with Dynamic Data");
        println!("-------------------------");
        structsdyntut();
        println!("---------------");
        println!("Generic Structs");
        println!("---------------");
        genstructstut();
    }
}
