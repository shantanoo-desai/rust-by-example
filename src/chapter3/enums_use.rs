#![allow(dead_code)]
enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

pub fn call_enums_use() {

    println!();
    println!("Chapter 3.2.1 Enums use");

    // Use something like this
    // use crate::Status::{Poor, Rich};
    use Status::{Poor, Rich};
    use crate::chapter3::enums_use::Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("the rich have lots of money!"),
        Poor => println!("the poor have no money.."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}