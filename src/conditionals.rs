// Conditionals - Used to check the condition of something and act on the result

pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person_of_age = true;

    // If/Else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave");
    } else {
        println!("Bartender: I'll need to see your ID");
    }

    // Shorthand If
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is Of Age: {}", is_of_age);

    // Ternary Operator
    // let is_of_age = if age >= 21 { true } else { false };
    let is_teen = if age >= 13 && age < 20 { true } else { false };
    println!("Is Teen: {}", is_teen);

    // Match
    let person: (u8, &str) = (18, "John");
    match person {
        (18, "John") => println!("John is 18"),
        (18, "Jane") => println!("Jane is 18"),
        _ => println!("Not John or Jane"),
    }

    // Match with Ranges
    let age: u8 = 18;
    match age {
        0..=12 => println!("Child"),
        13..=19 => println!("Teen"),
        _ => println!("Adult"),
    }

    // Match with Enums
    enum Person {
        Child,
        Teen,
        Adult,
    }

    let person: Person = Person::Adult;
    match person {
        Person::Child => println!("Child"),
        Person::Teen => println!("Teen"),
        Person::Adult => println!("Adult"),
    }

    let age: u8 = 18;
    let _person: Person = match age {
        0..=12 => Person::Child,
        13..=19 => Person::Teen,
        _ => Person::Adult,
    };
}
