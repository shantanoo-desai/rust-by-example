pub fn call_type_inference() {
    println!();
    println!("Inference");

    let elem = 5u8;

    let mut vec = Vec::new(); // not sure what type of data to store in array

    vec.push(elem);

    vec.push(1.0 as u8);

    println!("{:?}", vec);
}