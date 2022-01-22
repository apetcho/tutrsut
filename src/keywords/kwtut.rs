//! Tutorial
// as
pub fn kw_as() {
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
}
