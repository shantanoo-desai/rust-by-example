// Chapter 2.3 Arrays and Slices
use std::mem;

fn analyse_slice(slice: &[i32]) {
    println!("First Element of Slice: {}", slice[0]);
    println!("Last Element of Slice: {}", slice[slice.len() - 1]);
}

#[allow(dead_code)]
pub fn call_array_slices() {
    println!();
    println!("Chapter 2.3 Arrays and Slices");

    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // Initiated Array

    let ys: [i32; 5] = [0; 5];

    println!("First Element of xs Array: {}", xs[0]);
    println!("Array: ys: {:?}", ys);

    println!("Size of xs: {}", xs.len());

    println!("Array xs occupies {} bytes", mem::size_of_val(&xs));

    println!("Borrowing from arrays as slices");
    analyse_slice(&xs);

    // Section of an array using [start..end-1]
    analyse_slice(&xs[2..5]);
}