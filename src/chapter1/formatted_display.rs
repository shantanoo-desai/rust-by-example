// Chapter 1.2.2
// Implementing "custom" Display
use std::fmt;

// use Debug to check contrast between Debug and Display
#[derive(Debug)]
struct MinMax(i32, i32);

// Implement how the Struct should be displayed when using {} in println Macro
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // NOTE: DO NOT PUT SEMICOLON WHEN YOU NEED TO RETURN SOMETHING IN RUST
        write!(f, "(max: {}, min: {})", self.0, self.1)
    }
}

/////////////////////////////////////////////////////////////
#[derive(Debug)]
struct Point2D {
    x: i32,
    y: i32,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x,y): ({}, {})", self.x, self.y)
    }
}

/////////////////////////////////////////////////////////////////////////////
struct Complex2D {
    real: f32,
    imag: f32,
}

impl fmt::Display for Complex2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

#[allow(dead_code)]
pub fn call_formatted_display() {
    println!();
    println!("Chapter 1.2.2 Display");

    let first_range = MinMax(-32, 32);

    println!("Comparison: Debug and Display");
    println!("Debug: first_range {:?}", first_range);
    println!("Display: first_range: {}", first_range);

    let origin = Point2D { x: 0, y: 0 };
    println!("Origin of 2D cartesian system is: {}", origin);

    let complex_point = Complex2D {
        real: 2.34,
        imag: -3.34,
    };

    println!("Complex Point: {}", complex_point);
}
