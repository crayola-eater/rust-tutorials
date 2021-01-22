fn main() {
    let mut n = 0;

    loop {
        n += 1;

        if 7 == n {
            continue;
        }
        
        if n > 10 {
            break;
        }

        println!("The value of n is {}!", n);
    }
}
