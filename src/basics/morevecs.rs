//! More about vectors ...

mod vec3 {
    pub fn vec3_example() {
        let mut v1 = vec![10, 20, 30, 40];
        println!("v1 = {:?}", v1);
        println!("v1.pop() = {:?}", v1.pop());

        let mut v2 = Vec::new();
        v2.push(10);
        v2.push(20);
        v2.push(30);
        assert_eq!(v1, v2);
        v2.extend(0..2);
        assert_eq!(v2, &[10, 20, 30, 0, 1]);
    }
}

// sort() and dedup()
mod vec4 {
    pub fn vec4_example() {
        let mut v1 = vec![1, 10, 5, 1, 2, 11, 2, 40];
        println!("v1 = {:?}", v1);
        v1.sort();
        println!("v1.sort() = {:?}", v1);
        v1.dedup();
        println!("v1.dedup() = {:?}", v1);
    }
}

pub fn morevecs_examples() {
    vec3::vec3_example();
    vec4::vec4_example();
}
