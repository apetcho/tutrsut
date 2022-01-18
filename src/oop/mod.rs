//! Object-Orientation in Rust
mod traitobjs;

mod ooptut {
    use super::*;
    pub fn ooptut_examples() {
        traitobjs::traitobjs_examples();
    }
}

// ---
pub fn oop_examples() {
    println!("-------------------");
    println!("  TRAITS OBJECTS   ");
    println!("-------------------");
    ooptut::ooptut_examples();
}
