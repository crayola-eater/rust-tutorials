struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    let mut background = Colour { red: 255, green: 70, blue: 15 };
    println!("rgb({}, {}, {})", background.red, background.green, background.blue);
    
    background.blue = 45;
    println!("rgb({}, {}, {})", background.red, background.green, background.blue);
}
