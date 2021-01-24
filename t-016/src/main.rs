fn main() {
    let mut x = 10;
    println!("x is {}", x);
    
    // Immutable reference
    let immutable_reference_to_x = &x;
    println!("x (via immutable reference) is {}", immutable_reference_to_x);
    
    // Mutable reference
    let mutable_reference_to_x = &mut x;
    
    // Increment the value (not reference) by 1.
    *mutable_reference_to_x += 1;
    println!("x (via mutable reference) is {}", mutable_reference_to_x);

}
