use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("./data/output.txt").expect("Could not create file!");
    file.write_all(b"Some textual data.").expect("Could not write to the file!");
}
