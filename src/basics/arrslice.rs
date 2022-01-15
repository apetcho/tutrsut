//! Arrays and Slices

mod array1 {
    pub fn array1_example() {
        let arr = [10, 20, 30, 40];
        let first = arr[0];
        println!("first {}", first);

        for i in 0..4 {
            println!("[{}] = {}", i, arr[i]);
        }

        println!("length {}", arr.len());
    }
}

pub fn arrslice_example() {
    array1::array1_example();
}
