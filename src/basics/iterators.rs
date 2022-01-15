//! Iterators

mod iter1 {
    pub fn iter1_example() {
        let mut iter = 0..3;
        println!("iter = {:?}", iter);
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }
}

// Iterator over array
mod iter23 {
    pub fn iter23_example() {
        let arr = [10, 20, 30];
        for i in arr.iter() {
            println!("=> {}", i);
        }

        // slices are implicitely converted into iterators
        let slice = &arr;
        for i in slice {
            println!("{}", i);
        }
    }
}

pub fn iterators_examples() {
    iter1::iter1_example();
    iter23::iter23_example();
}
