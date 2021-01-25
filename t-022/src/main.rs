fn main() {
    let mut my_string = String::from("How's it going? My name is Margaret.");
    println!("Length: {}", my_string.len());
    println!("Is empty?: {}", my_string.is_empty());

    for token in my_string.split_whitespace() {
        println!("{}", token);
    }

    println!("Does the string contain 'Margaret'? {}", my_string.contains("Margaret"));

    my_string.push_str(" Welcome to your tutorial on Strings!");

    println!("After push: {}", my_string);
}