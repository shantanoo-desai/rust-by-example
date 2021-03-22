type NanoSecond = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;

pub fn call_type_aliasing() {
    println!();
    println!("Aliasing");

    let nano_second : NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;
    
    println!("{} nanoseconds + {} inches = {} unit?", nano_second, inches, nano_second + inches);
}