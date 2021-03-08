// Chapter 2.2 Tuples
use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})\n({}, {})", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

#[allow(dead_code)]
pub fn call_tuples() {
    println!();
    println!("Chapter 2.2 Tuples");

    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 02f64, 'a', true,
    );

    println!("Long Tuple Value 1: {}", long_tuple.0);
    println!("Long Tuple Value 2: {}", long_tuple.1);

    // Multi-Dimensional Tuples
    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i32), -2i32);

    println!("Tuple of Tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);

    println!("reversed tuple {:?}", reverse(pair));

    let tuple_to_destructure = (1, "hello", 's', true);
    let (number, text, character, boolean) = tuple_to_destructure;
    println!(
        "Destructured Tuple: {}, {}, {}, {}",
        number, text, character, boolean
    );

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);

    println!("Transpose:\n{}", transpose(matrix));
}
