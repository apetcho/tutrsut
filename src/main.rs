mod basics;

/// Basics tutorial
fn basics_tutorial() {
    basics::hello::hello_example();
}

fn loopif_example() {
    basics::loopif::loop_and_if_example();
}

fn main() {
    basics_tutorial();
    loopif_example();
}
