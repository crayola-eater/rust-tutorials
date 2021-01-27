use std::io;

fn main() {
    let mut input = String::new();
    println!("Say something");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Success! You said:\n\n{}", input);
        },
        Err(e) => println!("Oops, something went wrong! {}", e)
    }
}
