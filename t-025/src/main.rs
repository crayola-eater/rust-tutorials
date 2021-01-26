use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("./data/info.txt").expect("Can't open file!");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Cannot read the file!");
    println!("File contents:\n\n{}", contents)
}
