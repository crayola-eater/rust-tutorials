// Tuple struct
struct Colour (u8, u8, u8);

fn main() {
    let mut background = Colour(255, 0, 0);
    println!("Colour is rgb({}, {}, {})", background.0, background.1, background.2);

    background.2 = 60;
    println!("Colour is rgb({}, {}, {})", background.0, background.1, background.2);
}
