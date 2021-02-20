#![allow(dead_code)]

enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Day {
    fn is_weekday(&self) -> bool {
        match self {
            &Day::Saturday | &Day::Sunday => false,
            _ => true,
        }
    }
}

fn main() {
    let saturday = Day::Saturday;
    let monday = Day::Monday;

    println!("Is saturday a weekday? {}", saturday.is_weekday());
    println!("Is monday a weekday? {}", monday.is_weekday());
}
