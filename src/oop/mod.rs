//! Object-Orientation in Rust
mod animals;
mod genducks;
mod inheritance;
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

        println!("------------------------");
        println!("   DUCKS AND GENERICS   ");
        println!("------------------------");
        genducks::genducks_examples();

        println!("-----------------------");
        println!("   TRAIT INHERITANCE   ");
        println!("-----------------------");
        inheritance::inheritance_examples();
    }
}

// ---
pub fn oop_examples() {
    ooptut::ooptut_examples();
}
