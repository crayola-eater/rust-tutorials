fn main() {
    let name = String::from("Abe");

    for number in &[1, 8] {
        println!(
            "Character at index {}: {}",
            number,
            match name.chars().nth(*number) {
                Some(char) => char.to_string(),
                None => "No character found!".to_string(),
            }
        );
    }

    println!(
        "Occupation is: {}",
        match get_occupation("Abe") {
            Some(o) => o,
            None => "No occupation found!",
        }
    );
}

fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Abe" => Some("Software developer"),
        "Michael" => Some("Dentist"),
        _ => None,
    }
}
