/*
 * Primitive Types--
 *
 * Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
 * Floats: f32, f64
 * Boolean (bool)
 * Characters (char)
 * Tuples
 * Arrays
 */

// Rust is a statically typed language, which means that it must know the types of all
// variables at compile time, however, the compiler can usually infer what type we want to use
// based on the value and how we use it.

pub fn run() {
    // Default is "i32"
    let x = 1;
    println!("x = {}", x);

    // Default is "f64"
    let y = 2.5;
    println!("y = {}", y);

    // Add explicit type
    let z: i64 = 454545454545454545;
    println!("z = {}", z);

    // Find max size
    println!("Max i32: {}", std::i32::MAX);

    // Boolean
    let is_active: bool = true;
    println!("is_active = {}", is_active);

    // Get boolean from expression
    let is_greater: bool = 10 < 5;
    println!("is_greater = {}", is_greater);

    // Character
    // Single quotes
    // Unicode character
    // 4 bytes in size
    let a1 = 'a';
    let face = '\u{1F600}';
    println!("a1 = {}, face = {}", a1, face);

    // Tuples
    // Tuples group together values of different types
    // Max 12 elements
    // Tuples are immutable by default
    // Tuples can be destructured to create bindings to their individual elements
    // Tuples can be printed
    // Tuples can be used as function arguments and as return values
    // Tuples can be used as keys in a HashMap
    // Tuples can be used as elements in an array
    // Tuples can be used as elements in a vector
    // Tuples can be used as elements in a slice
    // Tuples can be used as elements in a tuple
    // Tuples can be used as elements in a tuple struct
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tup = ({}, {}, {})", x, y, z);

    // Access tuple by index
    // Index starts at 0
    // Index is a constant expression
    println!("tup.0 = {}", tup.0);

    // Arrays
    // Arrays are fixed length
    // Arrays are stack allocated
    // Arrays are not as flexible as vectors
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr = {:?}", arr);

    // Access array by index
    println!("arr[0] = {}", arr[0]);

    // Get array length
    // Array length is a constant expression
    // Array length is a property of the array type
    // Array length is not a property of the array value
    // Array length is not a property of the array variable
    println!("arr length = {}", arr.len());

    // Arrays are zero initialized
}
