fn main() {
    let number = 15;

    match number {
        -1 | 0 => println!("Either negative one or zero!"),
        1 => println!("It is one!"),
        2..=20 => println!("There are two of them!"),
        _ => println!("It doesn't match!"),
    }

    let name = "Ross";

    match name {
        "Jan" => println!("a"),
        "Ross" => println!("b"),
        "Aisle" => println!("c"),
        _ => println!("No match")
    }
}
