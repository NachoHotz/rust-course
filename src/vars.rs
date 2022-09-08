// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    // Define a variable

    let name = "Nacho";

    // Define a mutable variable
    let mut age = 22;

    println!("My name is {} and I am {}", name, age);

    age = 23; // This will throw an error

    println!("My name is {} and I am {}", name, age);

    // Define constant
    // Constants are always immutable
    // Constants can be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime.
    // Constants must be annotated with the type of data they hold.
    // Constants can be declared in any scope, including the global scope.
    // Constants may be set only to a scalar, a boolean, or a character.
    // Constants may not be set to a reference to data created in the stack frame of a function.
    // Constants may not be set to a reference to data created in the heap.

    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables
    let (my_name, my_age) = ("Nacho", 22);

    println!("{} is {}", my_name, my_age);
}
