//! Slicing and Dicing
mod array3 {
    pub fn array3_example() {
        let ints = [1, 2, 3];
        let floats = [1.0, 2.1, 3.1];
        let strings = ["hello", "world"];
        let ints_ints = [[1, 2], [10, 10]];
        println!("ints: {:?}", ints);
        println!("floats: {:?}", floats);
        println!("strings: {:?}", strings);
        println!("ints_ints: {:?}", ints_ints);
    }
}
// views
mod slice1 {
    pub fn slice1_example() {
        let ints = [1, 2, 3, 4, 5];
        let slice1 = &ints[0..2];
        let slice2 = &ints[1..];
        println!("ints: {:?}", ints);
        println!("slice1: {:?}", slice1);
        println!("slice2: {:?}", slice2);
    }
}

pub fn slicedice_examples() {
    array3::array3_example();
    slice1::slice1_example();
}
