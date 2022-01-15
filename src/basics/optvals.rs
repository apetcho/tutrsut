//! Optional Values

mod slice2 {
    pub fn slice2_examples() {
        let ints = [1, 2, 3, 4, 5];
        let slice = &ints;
        let first = slice.get(0);
        let last = slice.get(5);
        println!("first: {:?}", first);
        println!("last: {:?}", last);
        println!("first:: {} {}", first.is_some(), first.is_none());
        println!("last:: {} {}", last.is_some(), last.is_none());
        println!("first value: {}", first.unwrap());
        let maybe_last = slice.get(5);
        let last = if maybe_last.is_some() {
            *maybe_last.unwrap()
        } else {
            -1
        };
        println!("last = {}", last);
    }
}

pub fn optional_values_examples() {
    slice2::slice2_examples();
}
