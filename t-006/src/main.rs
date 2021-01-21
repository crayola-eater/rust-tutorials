fn main() {
    let n = 45;

    if n < 30 {
        println!("The number {} is less than 30!", n);
    } else if n > 40 {
        println!("The number {} is greater than 40!", n);
    } else {
        println!("The number {} is greater than 30 and less than or equal to 40!", n);
    }

    if 45 == n {
        println!("The number is 45! Rust!");
    }
}
