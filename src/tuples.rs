// Tuples are a way to group together a number of values with a variety of types into one compound type.
// Tuples have a fixed length: once declared, they cannot grow or shrink in size.
// Tuples are useful for returning multiple values from a function without using a struct.
// Tuples are also useful for passing around groups of values of different types.
// Tuples are indexed from 0.
// Tuples can be destructured to create bindings to their individual elements.
// Tuples can be printed using {:?} debug formatter.
// Tuples can be compared if their individual elements are comparable.
// Tuples can be used as function arguments and as return values.
// Tuples can be used as keys in a HashMap.
// Tuples can be used as elements in an array.
// Tuples can be used as elements in a vector.
// Tuples can be used as elements in a slice.
// Tuples can be used as elements in a tuple.
// Max 12 elements.

pub fn run() {
    let person: (&str, &str, i8) = ("Nacho", "Hotz", 22);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}
