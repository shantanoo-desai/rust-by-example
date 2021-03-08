// Chapter 2.1
#[allow(dead_code)]
pub fn call_primitives() {
    println!();
    println!("Chapter 2.1 Literals and Operators");

    println!("Boolean Logic");
    println!("true AND false = {}", true && false);
    println!("true OR false = {}", true || false);
    println!("NOT true = {}", !true);

    println!("BitWise Operator");
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 = {}", 1u32 << 5);
    println!("0x80 >> 2 = 0x{:x}", 0x80u32 >> 2);

    println!("One Million with underscores: {}", 1_000_000u32);
}
