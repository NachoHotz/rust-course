pub fn run() {
    println!("Hello, world! from print.rs");

    // Basic formatting
    println!("{} is from {}", "Brad", "Mass");

    // Positional Arguments
    // Arguments are passed in order
    // Arguments are passed by index
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Brad", "Mass", "code"
    );

    // Named arguments
    // Arguments are passed by name
    // Arguments are passed in any order
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "Baseball"
    );

    // Placeholder traits
    // :b = binary
    // :x = hex
    // :o = octal
    // :p = pointer
    // :? = debug
    // :#? = pretty debug
    // :#x = pretty hex
    // :#X = pretty hex upper
    // :#o = pretty octal
    // :#b = pretty binary
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    // :? = debug
    println!("{:?}", (12, true, "hello"));

    // Basic math
    // Add
    println!("10 + 10 = {}", 10 + 10);
}
