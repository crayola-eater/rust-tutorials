use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();

    // Add values
    for subject in vec!["Maths", "English", "Science"] {
        marks.insert(subject, 96);
    }

    // Find length of HashMap
    println!("How many subjects have you studied? {}", marks.len());

    match marks.get("Maths") {
        Some(mark) => println!("You got {} in Maths!", mark),
        None => println!("You didn't study Maths!"),
    }

    // Remove a value
    marks.remove("maths");

    for (subject, mark) in &marks {
        println!("For {}, you got {}%!", subject, mark);
    }

    println!("Did you study French? {}", marks.contains_key("French"));

    marks.clear();
}
