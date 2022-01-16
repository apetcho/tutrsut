mod moveit;
mod scopevars;
mod tuples;

pub mod tutorial {
    use super::*;
    // Rust likes to move it, move it
    fn rust_likes_to_move_it() {
        moveit::moveit_examples();
    }

    // Scope of Variables
    fn scope_of_variables() {
        scopevars::scopevars_examples();
    }

    // Tuples
    fn tupletut() {
        tuples::tuples_examples();
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
    }
}
