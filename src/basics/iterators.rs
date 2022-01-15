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

// Idiomatic use of iterator to perform sum of array of integers
mod sum1 {
    pub fn sum1_example() {
        let sum: i32 = (0..5).sum();
        println!("sum1 = {}", sum);

        let sum: i64 = [10, 20, 30].iter().sum();
        println!("sum2 = {}", sum);
    }
}

// windows() method
mod slice4 {
    pub fn slice4_example() {
        let ints = [1, 2, 3, 4, 5];
        let slice = &ints;

        println!("slice = {:?}", slice);
        for s in slice.windows(2) {
            println!("window => {:?}", s);
        }

        // chunks()
        for s in slice.chunks(2) {
            println!("chunks => {:?}", s);
        }
    }
}

pub fn iterators_examples() {
    iter1::iter1_example();
    iter23::iter23_example();
    sum1::sum1_example();
    slice4::slice4_example();
}
