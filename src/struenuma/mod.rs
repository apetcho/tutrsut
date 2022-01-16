mod lifetbite;
mod moveit;
mod scopevars;
mod structs;
mod traits;
mod tuples;

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
    }
}