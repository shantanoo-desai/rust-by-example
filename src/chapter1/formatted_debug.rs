// Chapter 1.2.1
// By Default Rust makes Structs in a Module private

#[derive(Debug)] 
struct DebugPrintable(i32);


// Nested Structure
#[derive(Debug)]
struct DeepStruct(DebugPrintable);


#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

#[allow(dead_code)]
pub fn call_formatted_debug() {

    println!();
    println!("Chapter 1.2.1: Debug");
    println!("Structure printable with `:?` : {:?}", DebugPrintable(3));
    println!("Deeper Structure called with `:?` {:?}", DeepStruct(DebugPrintable(2)));

    println!("Pretty Printing a Struct with `:#?`");
    let name = "Alice";
    let age = 20;
    let alice = Person{ name, age};
    println!("{:#?}", alice);
}