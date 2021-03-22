// Chapter 4: Variables

#[allow(dead_code)]
pub fn call_variables() {
    println!();
    println!("Chapter 4. Variables");
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An Integer {:?}", copied_integer);
    println!("A boolean {:?}", a_boolean);
    println!("Unit Value: {:?}", unit);

    let _unused_variable = 2u32; // Suppress Unused Variable

    let mut mutable_binding = 1;
    println!("Before Mutation: {}", mutable_binding);
    mutable_binding += 1;
    println!("After Mutation: {}", mutable_binding);

    println!("\nScope and Shadowing");

    let long_lived_binding = 1;
    {
        println!("Entering Scope");
        let short_lived_binding = 2;
        println!("inner_binding: {}", short_lived_binding);
    }
    println!("Exited Scope");

    println!("Long Lived binding: {}", long_lived_binding);

    println!("\nVariable Shadowing");

    let shadowed_binding = 1;
    {
        println!("before being shadowed: {}", shadowed_binding);
        let shadowed_binding = "abd";
        println!("Shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);

    println!("\nDeclare First");

    // Declare first
    let a_binding;

    {
        let x = 2;
        a_binding = x * x;
    }

    println!("a binding {}", a_binding);

    // Freezing Variables

    let mut _mutable_integer = 7i32;

    {
        //  this is an immutable variable
        let _mutable_integer = _mutable_integer;

        // _mutable_integer = 50; // Cannot change an immutable variable
    }

    _mutable_integer = 3; // outer scope is good
}