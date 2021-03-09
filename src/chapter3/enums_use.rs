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

    // use crate::Work::*;

    let status = Status::Poor;
    let work = Work::Civilian;

    match status {
        Status::Rich => println!("the rich have lots of money!"),
        Status::Poor => println!("the poor have no money.."),
    }

    match work {
        Work::Civilian => println!("Civilians work!"),
        Work::Soldier => println!("Soldiers fight!"),
    }
}