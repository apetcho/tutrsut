//! Ducks and Generics

trait Quack {
    fn quack(&self);
}

// --
fn quack<Q>(q: &Q)
where
    Q: Quack,
{
    q.quack();
}
// ---

struct Duck();

impl Quack for Duck {
    fn quack(&self) {
        println!("quack!");
    }
}

// ---
struct RandomBird {
    is_a_parrot: bool,
}

impl Quack for RandomBird {
    fn quack(&self) {
        if !self.is_a_parrot {
            println!("quack!");
        } else {
            println!("squack!");
        }
    }
}
// ---
impl Quack for i32 {
    fn quack(&self) {
        for i in 0..*self {
            print!("quack {} ", i);
        }
        println!("");
    }
}

// ---
pub fn genducks_examples() {
    let duck1 = Duck();
    quack(&duck1);
}
