mod conditionals;
mod loops;
mod print;
mod strings;
mod structs;
mod tuples;
mod types;
mod vars;
mod vectors;

fn main() {
    print::run();
    vars::run();
    types::run();
    strings::run();
    tuples::run();
    vectors::run();
    conditionals::run();
    loops::run();
    structs::run();
}
