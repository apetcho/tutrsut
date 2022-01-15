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

pub fn iterators_examples() {
    iter1::iter1_example();
}
