fn main() {
    let x = 10;
    {
        let x = 15;
        println!("x is {}!", x);
    }

    println!("x is {}!", x);
    
    // Seems better not to assign new value + data type.
    // Instead, could probably just create a new variable.
    let x = "some_string";
    println!("x is {}!", x);
    
    // Seems better not to assign new value + data type.
    // Instead, could probably just create a new variable.
    let x = true;
    println!("x is {}!", x);
}
