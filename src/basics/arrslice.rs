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

// Pass array or slice to a function
mod array2 {
    fn sum(values: &[i32]) -> i32 {
        let mut res = 0;
        for i in 0..values.len() {
            res += values[i];
        }
        res
    }
    pub fn array2_example() {
        let arr = [10, 20, 30];
        println!("arr = {:?}", &arr);
        let res = sum(&arr);
        println!("sum = {}", res);
    }
}

pub fn arrslice_example() {
    array1::array1_example();
    array2::array2_example();
}
