//! Animals

trait Quack {
    fn quack(&self);
}

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
fn quack_ref(q: &dyn Quack) {
    q.quack();
}

// ---
trait Named {
    fn name(&self) -> String;

    fn upper_case(&self) -> String {
        self.name().to_uppercase()
    }
}

struct Boo();

impl Named for Boo {
    fn name(&self) -> String {
        String::from("boo")
    }
}

// ---
pub fn animals_examples() {
    let duck1 = Duck();
    let duck2 = RandomBird { is_a_parrot: false };
    let parrot = RandomBird { is_a_parrot: true };
    let n = 4;
    let ducks: Vec<&dyn Quack> = vec![&duck1, &duck2, &parrot, &n];
    for d in &ducks {
        d.quack();
    }

    println!("quack_ref() ");
    for d in ducks {
        quack_ref(d);
    }

    println!("Named Boo");
    let boo = Boo();
    println!("boo.name() = {}", boo.name());
    println!("boo.upper_case() = {}", boo.upper_case());
}
