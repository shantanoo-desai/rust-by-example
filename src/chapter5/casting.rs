#[allow(dead_code)]
#[allow(overflowing_literals)]
pub fn call_types_casting() {
    println!();
    println!("Chapter 5: Casting");

    let decimal = 65.4321_f32;

    // Can't DO THIS in Rust
    // let integer: i32 = decimal;

    let integer = decimal as i32;

    let character_u8 = integer as u8;

    let character = character_u8 as char;

    println!("Casted: {} => {} => {} => {}", decimal, integer, character_u8, character);

    println!("Casting to unsigned types");
    println!("1000 as u16 is: {}", 1000 as u16);

    println!("1000 as a u8 is {}", 1000 as u8);

    println!("-1 as u8 is {}", (-1i8) as u8);

    println!("1000 mod 256 is {}", 1000 % 256);

    println!("Casting to signed types");

    println!("128 as i16 is {}", 128u8 as i16);

    println!("128 as a i8 is {}", 128u8 as i8);

    println!("232 as a i8 is {}", 232 as i8);

    println!("Float to Int Conversion");

    // Upper-bound of u8
    println!("300.0 in u8 is {}", 300.0_f32 as u8);

    // Lower-bound of u8
    println!("-100.0 as u8 is {}", -100.0_f32 as u8);

    println!("nan as u8 is {}", f32::NAN as u8);

    println!("Unsafe Conversions");

    unsafe {
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }

}