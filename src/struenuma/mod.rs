mod moveit;
mod scopevars;

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

    pub fn structs_enums_and_matching() {
        rust_likes_to_move_it();
        scope_of_variables();
    }
}
