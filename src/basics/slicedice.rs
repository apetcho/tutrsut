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

pub fn slicedice_examples() {
    array3::array3_example();
}
