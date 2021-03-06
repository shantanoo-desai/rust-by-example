// Chapter 1.2.2.1 Testcase: List
// Handling Sequential Structures for Displaying
use std::fmt;

//////////////////////////////////////////////////////////////////////
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // extract the vector
        let vec = &self.0;
        
        write!(f, "[")?; // `?` will handle all varying results

        // iterate over the vector
        for (count, v) in vec.iter().enumerate() {
            // except the first element add comma and space
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}


//////////////////////////////////////////////////////////////////////

struct List2(Vec<i32>);

impl fmt::Display for List2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]")

    }
}

#[allow(dead_code)]
pub fn call_formatted_display_list() {
    println!("");
    println!("Chapter 1.2.2.1 Lists");
    let v = List(vec![1, 2, 3]);
    println!("List Struct: {}", v);

    let v2 = List2(vec![20, 40, 60]);

    println!("Updated List: {}", v2);

}
