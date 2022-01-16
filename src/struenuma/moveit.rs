//! Rust like to Move It, Move It

mod movetut {
    fn dump(s: &str) {
        println!("{}", s);
    }
    pub fn movetut_examples() {
        let s1 = "Hello World!".to_string();
        //let s2 = s1;
        dump(&s1);
        println!("s1 = {}", s1);
    }
}

pub fn moveit_examples() {
    movetut::movetut_examples();
}
