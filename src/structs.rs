// structs are used to create custom data types

pub fn run() {
    // traditional struct
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    // tuple struct
    struct ColorTuple(u8, u8, u8);

    let mut ct = ColorTuple(255, 0, 0);

    ct.0 = 200;

    println!("Color: {} {} {}", ct.0, ct.1, ct.2);

    // person struct
    struct Person {
        first_name: String,
        last_name: String,
    }

    impl Person {
        // construct person
        fn new(first: &str, last: &str) -> Person {
            Person {
                first_name: first.to_string(),
                last_name: last.to_string(),
            }
        }

        // get full name
        fn full_name(&self) -> String {
            format!("{} {}", self.first_name, self.last_name)
        }

        // set last name
        fn set_last_name(&mut self, last: &str) {
            self.last_name = last.to_string();
        }

        // name to tuple
        fn to_tuple(self) -> (String, String) {
            (self.first_name, self.last_name)
        }
    }

    let mut p = Person::new("John", "Doe");
    println!("Person {} {}", p.first_name, p.last_name);
    println!("Person {}", p.full_name());
    p.set_last_name("Williams");
    println!("Person {}", p.full_name());
    println!("Person Tuple {:?}", p.to_tuple());
}
