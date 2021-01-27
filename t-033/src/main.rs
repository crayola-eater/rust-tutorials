fn main() {
    // Replace
    {
        let some_string = String::from("Rust is good.");
        println!("Before replace: {}", some_string);
        println!("After replace: {}", some_string.replace("good.", "fantastic!"));
    }
    
    // Lines
    {
        let some_string = String::from("The weather is nice outside.\nTime for ice cream!");
        for line in some_string.lines() {
            println!("{}", line);
        }
    }

    // Split
    {
        let some_string = String::from("a,b,c,d");
        let tokens: Vec<&str> = some_string.split(",").collect();

        println!("At index 2: {}", tokens[2]);
    }

    // Trim
    {
        let some_string = String::from(" Some text that needs trimming \n ");
        println!("Before trim {}", some_string);
        println!("After trim {}", some_string.trim());
    }

    // Chars
    {
        let some_string = String::from("ABCDE");

        // Get character at index.
        match some_string.chars().nth(4) {
            Some(char) => println!("Character at index 4 is '{}'", char),
            None => println!("No character at index 4")
        }
    }
}
