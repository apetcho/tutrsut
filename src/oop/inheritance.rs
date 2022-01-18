//! Inheritance

trait Show {
    fn show(&self) -> String;
}

trait Location {
    fn location(&self) -> String;
}

//
trait ShowTell: Show + Location {}

//
#[derive(Debug)]
struct Foo {
    name: String,
    location: String,
}

impl Foo {
    fn new(name: &str, location: &str) -> Self {
        Foo {
            name: String::from(name),
            location: String::from(location),
        }
    }
}

impl Show for Foo {
    fn show(&self) -> String {
        self.name.clone()
    }
}

impl Location for Foo {
    fn location(&self) -> String {
        self.location.clone()
    }
}

impl ShowTell for Foo {}

// --
macro_rules! dbg {
    ($x:expr) => {
        println!("{} = {:?}", stringify!($x), $x);
    };
}

// ---
fn show_it_all(r: &dyn ShowTell) {
    dbg!(r.show());
    dbg!(r.location());
}

// ---
fn show(s: &dyn Show) {
    dbg!(s.show());
}

// Entry Point
pub fn inheritance_examples() {
    let foo = Foo::new("Hello", "World");
    dbg!(foo.show());
    dbg!(foo.location());

    let st: &dyn ShowTell = &foo;

    dbg!(st.show());
    dbg!(st.location());

    // ---
    let angela = Foo::new("Angela", "Berlin");
    show_it_all(&angela);
    show(&angela);
}
