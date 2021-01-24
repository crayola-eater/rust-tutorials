struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    let mut blue = Colour { red: 0, green: 0, blue: 246 };
    print_colour(&blue);

    blue.blue += 9;
    print_colour(&blue);
}

fn print_colour(c: &Colour) {
    println!("Colour - R:{} G:{} B:{}", c.red, c.green, c.blue);
}
