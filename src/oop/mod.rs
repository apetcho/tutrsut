//! Object-Orientation in Rust
mod animals;
mod traitobjs;

mod ooptut {
    use super::*;
    pub fn ooptut_examples() {
        println!("-------------------");
        println!("  TRAITS OBJECTS   ");
        println!("-------------------");
        traitobjs::traitobjs_examples();
        println!("-------------");
        println!("   ANIMALS   ");
        println!("-------------");
        animals::animals_examples();
    }
}

// ---
pub fn oop_examples() {
    ooptut::ooptut_examples();
}
