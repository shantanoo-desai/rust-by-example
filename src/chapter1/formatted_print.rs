#[allow(dead_code)]
pub fn call_formatted() {
    println!();
    println!("Chapter 1.2 Formatted Print");
    // You can add suffix type to values i.e. i/u8,16,32,64 | f32,64
    println!("Without Suffix Print: {} days", 31);
    println!("With Suffix Print: {} days", 356u32);

    // Positional Arguments in Formatted Printing
    println!("Positional Arguments: {1} {0}", "Alice", "Bob");

    // Named Arguments in Formatted Printing
    println!(
        "Named Arguments: {subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Special Formatting
    println!("Binary of {number}: {number:b}", number = 255);
    println!("Octal of {number}: {number:o}", number = 255);
    println!("Hexadecimal of {number}: {number:x}", number = 255);

    // Text alignment

    println!(
        "White Space Padding: {number:>padding$}",
        number = 1,
        padding = 6
    );
    println!(
        "Padding with Zeroes: {number:>0padding$}",
        number = 1,
        padding = 6
    );

    // Precision Control

    println!(
        "value of {pi} till {precision} precisions is: {pi:.precision$}",
        pi = 3.141592,
        precision = 2
    );
}
