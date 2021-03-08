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
    p1: Point,
    p2: Point,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.p1, self.p2)
    }
}

fn area_rect(rec: Rectangle) -> f32 {
    match rec {
        Rectangle {
            p1: Point { x: x1, y: y1 },
            p2: Point { x: x2, y: y2 },
        } => f32::abs((x2 - x1) * (y1 - y2)),
    }
}

fn square(ref_point: Point, width: f32) -> Rectangle {
    let side = Point {
        x: ref_point.x + width,
        y: ref_point.y + width,
    };
    Rectangle {
        p1: ref_point,
        p2: side,
    }
}

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
        p1: rect_top_left,
        p2: rect_bottom_right,
    };
    println!("Area of Rectangle: {}", area_rect(rect));

    let ref_point_sq = Point { x: 10.0, y: 4.0 };
    // println!("{}", square(ref_point_sq, 5.0));
    println!("Area of Square: {}", area_rect(square(ref_point_sq, 3.0)));
}
