#![allow(dead_code)]
pub fn call_types_literals() {
    println!();
    println!("Literals");

    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed Literals
    // depends on usage

    let i = 1;
    let f = 1.0;

    println!("Size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("Size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("Size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("Size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("Size of `f` in bytes: {}", std::mem::size_of_val(&f));
}