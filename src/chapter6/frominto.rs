#![allow(dead_code)]

#[derive(Debug)]
struct Number {
    value: i32,
}

impl std::convert::From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl std::convert::From<f32> for Number {
    fn from(item: f32) -> Self {
        Number { value : item as i32 }
    }
}

pub fn call_conversion_frominto() {
    println!();
    println!("Chapter 6: From and Into");

    let mut num = Number::from(23);

    println!("Number: {:?}", num);

    num = Number::from(23.0);

    println!("Number from(f32): {:?}", num);

    let temp_int = 32;

    let into_num: Number = temp_int.into();

    println!("Into (int): {:?}", into_num);

    let temp_float = 3.2323f32;

    let into_f_num: Number = temp_float.into();

    println!("Floating Number: {:?}", into_f_num);

}