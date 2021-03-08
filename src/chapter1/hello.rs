// hello.rs is in the Root
// By Default Rust Functions are private
// with the `pub` the function `call_hello` is not callable in `main.rs`

#[allow(dead_code)]
pub fn call_hello() {
    println!();
    println!("Chapter: 1 Hello World");
    println!("Hello, Rust World!");
    println!("I am a Rustacean!");
}
