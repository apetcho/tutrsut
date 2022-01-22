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

// extern
pub fn kw_extern() {
    println!("");
    println!("<kw>extern</kw>");
    println!("> Link to or import external code.");
}

// false
pub fn kw_false() {
    println!("");
    println!("<kw>false</kw>");
    println!("> A value of type [`bool`] representing logical **false**");
}

// fn
pub fn kw_fn() {
    println!("");
    println!("<kw>fn<kw>");
    println!("> A function or function pointer");
}

// for
pub fn kw_for() {
    println!("");
    println!(
        "{}, {}, {}",
        "Iteration with [`in`]",
        "trait implementation with [`impl`]",
        "or [higher-ranked trait bounds] (`for<'a>`)."
    );
    //
    for i in 0..5 {
        println!("{} -> {}", i, i * 2);
    }
    for i in std::iter::repeat(5) {
        println!("turns out {} never stops being 5", i);
        break; // would loop forever otherwise
    }
    'outer: for x in 5..50 {
        for y in 0..10 {
            if x == y {
                break 'outer;
            }
        }
    }
}

// if
pub fn kw_if() {
    println!("");
    println!("<kw>if</kw>");
    println!("> Evaluate a block if a condition holds");
    // logic
    if 1 == 2 {
        println!("whoops, mathematics broke");
    } else {
        println!("everything's fine!");
    }
    // expression
    let rude = true;
    let greeting = if rude {
        "super nerd."
    } else {
        "Hello, friend!"
    };
    println!("greeting = {}", greeting);
    if let Ok(x) = "123".parse::<i32>() {
        println!("{} double that and you get {}!", greeting, x * 2);
    }
}

// impl
pub fn kw_impl() {
    println!("");
    println!("<kw>impl</kw>");
    println!("> Implement some functionality for a type");
}

// in
pub fn kw_in() {
    println!("");
    println!("<kw>in</kw>");
    println!("> Iterate over a series of values with [`for`]");
}

// let
pub fn kw_let() {
    println!("");
    println!("<kw>let</kw>");
    println!("> Bind a value to a variable");
    let thing1: i32 = 100;
    let thing2 = 200 + thing1;
    println!("thing1 = {}", thing1);
    println!("thing2 = {}", thing2);
    //
    #[allow(unused_assignments)]
    let mut changing_thing = true;
    changing_thing = false;
    println!("changing_thing = {}", changing_thing);
}

// while
pub fn kw_while() {
    println!("");
    println!("<kw>while</kw>");
    println!("> Loop while a condition is upheld");
    //
    let mut counter = 0;
    while counter < 10 {
        print!(" {}", counter);
        counter += 1;
    }
    println!("");
    //
    let mut i = 1;
    while i < 100 {
        i *= 2;
        print!(" {}", i);
        if i == 64 {
            break;
        }
    }
    println!("");
    // --
    let mut counter = Some(0);
    while let Some(i) = counter {
        if i == 10 {
            counter = None;
        } else {
            print!("{}", i);
            counter = Some(i + 1);
        }
    }
    println!("");
    println!("counter = {:?}", counter);
}

// loop
pub fn kw_loop() {
    println!("");
    println!("<kw>loop</kw>");
    println!("> Loop indefinitely");
    let mut i = 1;
    loop {
        println!("i is {}", i);
        if i > 100 {
            break;
        }
        i *= 2;
    }
    println!("i = {}", i);
    //
    i = 1;
    let something = loop {
        i *= 2;
        if i > 100 {
            break i;
        }
    };
    println!("something = {}", something);
}

// match
pub fn kw_match() {
    println!("");
    println!("<kw>match</kw>");
    println!("> Control flow based on pattern matching.");
    //
    let opt = Option::None::<usize>;
    let x = match opt {
        Some(int) => int,
        None => 10,
    };
    println!("x = {}", x);
    let anum = Option::Some(10);
    match anum {
        Some(x) if x <= 5 => println!("0 to 5 num = {}", x),
        Some(x @ 6..=10) => println!("6 to 10 num = {}", x),
        None => panic!(),
        // all other numbers
        _ => panic!(),
    }
}
