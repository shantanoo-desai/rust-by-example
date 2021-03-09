// Chapter 3.1 Structs in Rust
use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit Struct
#[allow(dead_code)]
struct Unit;

// Tuple Struct aka. Named Tuple
#[allow(dead_code)]
struct Pair(i32, f32);

////////////////////////////////////////////////////////////////////////
struct Point {
    x: f32,
    y: f32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

////////////////////////////////////////////////////////////////////////
struct Rectangle {
    top_left: Point,
    right_bottom: Point,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.top_left, self.right_bottom)
    }
}

#[allow(dead_code)]
fn area_rect(rec: Rectangle) -> f32 {
    match rec {
        Rectangle {
            top_left: Point { x: x1, y: y1 },
            right_bottom: Point { x: x2, y: y2 },
        } => f32::abs((x2 - x1) * (y1 - y2)),
    }
}

#[allow(dead_code)]
fn square(lower_left: Point, width: f32) -> Rectangle {
    
    // lower_left is (x1, y2) 
    // x1 + width => (x2, y2) => bottom_right
    // y2 + width => (x1, y1) => top_left
    let derived_top_left: Point = Point{ x: lower_left.x , y: lower_left.y + width };
    let derived_right_bottom: Point =  Point { x: lower_left.x + width, y: lower_left.y };
    Rectangle {
        top_left: derived_top_left,
        right_bottom: derived_right_bottom,
    }
}

#[allow(dead_code)]
pub fn call_structures() {
    println!("");
    println!("Chapter 3.1 Structs");

    let name = String::from("Peter");
    let age = 19;
    // Initialize Structs with pre-defined variables
    let peter = Person { name, age };
    println!("{:?}", peter);

    // Accessing Fields of Struct
    let point: Point = Point { x: 10.3, y: 0.5 };

    println!("X: {}, Y: {}", point.x, point.y);

    // Struct Update syntax

    let bottom_right = Point { x: 11.3, ..point };
    println!("point.y: {}, bottom_right.y: {}", point.y, bottom_right.y);

    let rect_top_left = Point { x: 20.0, y: 10.0 };
    let rect_bottom_right = Point { x: 30.0, y: 5.0 };
    let rect: Rectangle = Rectangle {
        top_left: rect_top_left,
        right_bottom: rect_bottom_right,
    };
    println!("Area of Rectangle: {}", area_rect(rect));

    let ref_point_sq = Point { x: 10.0, y: 4.0 };
    // println!("{}", square(ref_point_sq, 5.0));
    println!("Area of Square: {}", area_rect(square(ref_point_sq, 3.0)));
}
