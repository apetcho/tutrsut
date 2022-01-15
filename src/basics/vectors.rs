//! Vectors

mod vec1 {
    pub fn vec1_example() {
        let mut v = Vec::new();
        v.push(10);
        v.push(20);
        v.push(30);

        let first = v[0]; // will panic if out-of-range
        let maybe_first = v.get(0);
        println!("v is {:?}", v);
        println!("first is {}", first);
        println!("maybe_first is {:?}", maybe_first);
    }
}

// relationship between vectors and slices
mod vec2 {
    fn dump(arr: &[i32]) {
        println!("arr is {:?}", arr);
    }
    pub fn vec2_example() {
        let mut v = Vec::new();
        v.push(10);
        v.push(20);
        v.push(30);
        dump(&v);
        let slice = &v[1..];
        println!("slice is {:?}", slice);
    }
}

pub fn vectors_examples() {
    vec1::vec1_example();
    vec2::vec2_example();
}
