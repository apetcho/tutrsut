//! Tutorial
// as
pub fn kw_as() {
    println!("");
    println!("<kw>as</kw>");
    println!("> Cast between types, or rename an import");
    let thing1: u8 = 89.0 as u8;
    assert_eq!('B' as u32, 66);
    assert_eq!(thing1 as char, 'Y');
    let thing2: f32 = thing1 as f32 + 10.5;
    assert_eq!(true as u8 + thing2 as u8, 100);
}

// break
pub fn kw_break() {
    println!("");
    println!("<kw>break</kw>");
    println!("> Exit ealy from a loop");
    let mut last = 0;
    println!("Before:: last = {}", last);
    for x in 1..100 {
        print!("{} ", x);
        if x > 12 {
            break;
        }
        last = x;
    }

    println!("\nAfter:: last = {}", last);
    'outer: for i in 1..=5 {
        println!("outer iteration (i): {}", i);
        '_inner: for j in 1..=200 {
            println!("      inner iteration (j): {}", j);
            if j >= 3 {
                break; // breaks from inner loop, lets, outer loop continue.
            }
            if i >= 2 {
                break 'outer; // breaks from outer loop
            }
        }
    }
    println!("Bye!");

    // --
    let (mut a, mut b) = (1, 1);
    let result = loop {
        if b > 10 {
            break b;
        }
        let c = a + b;
        a = b;
        b = c;
    };
    println!("> result = {}", result);
}

// const
pub fn kw_const() {
    println!("");
    println!("<kw>const</kw>");
    println!(
        "> Compile-time constants, compile-time evaluable function and {}",
        "raw pointers."
    );
    //
    const THING: u32 = 0xABAD1DEA;
    println!("THING = {}", THING);
    let foo = 123 + THING;
    println!("foo = {}", foo);
    //
    const WORDS: &str = "Hello Rust!";
    println!("WORDS = {}", WORDS);
}

// continue
pub fn kw_continue() {
    println!("");
    println!("<kw>continue</kw>");
    println!("> Skip to the next iteration of a loop");
    // --
    for number in 1..=10 {
        if number % 2 == 0 {
            continue;
        }
        print!(" {}", number);
    }
    println!("");
    // -- Print Odd numbers under 30 with unit <= 5
    'tens: for ten in 0..3 {
        '_units: for unit in 0..=9 {
            if unit % 2 == 0 {
                continue;
            }
            if unit > 5 {
                continue 'tens;
            }
            print!(" {}", ten * 10 + unit);
        }
    }
    println!("");
}

// crate
pub fn kw_crate() {
    println!("");
    println!("<kw>crate</kw>");
    println!("> Rust binary or library");
}

// else
pub fn kw_else() {
    println!("");
    println!("<kw>else</kw>");
    println!(
        "> What expression to evaluate when an [`if`] condition evaluate to {}",
        "[`false`]."
    );
    let result = if true == false {
        "oh no"
    } else if "something" == "other thing" {
        "oh dear"
    } else if let Some(200) = "blarg".parse::<i32>().ok() {
        "uh oh"
    } else {
        println!("Sneaky side effect.");
        "phew, nothing's broken"
    };
    println!("result = {}", result);
    //
    if true == false {
        println!("oh no");
    } else if "something" == "other thing" {
        println!("oh dear");
    } else if let Some(200) = "blarg".parse::<i32>().ok() {
        println!("uh oh");
    } else {
        println!("phew, nothing's broken");
    }
}

// enum
pub fn kw_enum() {
    println!("");
    println!("<kw>enum</kw>");
    println!("> A type that can be any one of several variants");
    #[allow(dead_code)]
    enum SimpleEnum {
        FirstVariant,
        SecondVariant,
        ThirdVariant,
    }

    #[allow(dead_code)]
    enum Location {
        Unknown,
        Anonymous,
        Known(i32),
    }

    #[allow(dead_code)]
    enum ComplexEnum {
        Nothing,
        Something(u32),
        LostOfThings {
            usual_struct_stuff: bool,
            blah: String,
        },
    }

    #[allow(dead_code)]
    enum EmptyEnum {}
}
