//! Trait Objects

trait Show {
    fn show(&self) -> String;
}

// ---
impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

// ---
impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}

// --
pub fn traitobjs_examples() {
    let answer = 42;
    let maybe_pi = 3.14;
    let v: Vec<&dyn Show> = vec![&answer, &maybe_pi];
    for d in v.iter() {
        println!("show -> {}", d.show());
    }

    let answer = Box::new(42);
    let maybe_pi = Box::new(3.14);
    let show_list: Vec<Box<dyn Show>> = vec![answer, maybe_pi];
    for d in &show_list {
        println!("show => {}", d.show());
    }
}
