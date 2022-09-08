use std::mem;
// Arrays are fixed-size lists where elements are the same data types
// Arrays are stack allocated
// Arrays are zero-indexed
// Arrays are immutable by default
// Arrays are not allowed to grow or shrink in size
// Arrays are useful when you want your data allocated on the stack rather than the heap
// Arrays are useful when you want to ensure you always have a fixed number of elements
// Arrays are useful when you want to ensure you always have exactly the same types of elements
// Arrays are useful when you want to ensure you always have a contiguous block of memory
// Arrays are useful when you want to ensure you always have a known amount of memory used

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(6);

    // Remove last value
    numbers.pop();

    // Get single value
    println!("Single value: {}", numbers[0]);

    // Get vector length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Arrays are immutable by default
    // numbers[2] = 20;
    // println!("{:?}", numbers);
    // error[E0594]: cannot assign to immutable indexed content `numbers[2]`
    //  --> arrays.rs:25:5

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);

    // Vector of vectors
    let matrix: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    println!("{:?}", matrix);

    for row in matrix.iter() {
        for col in row.iter() {
            println!("{}", col);
        }
    }
}
