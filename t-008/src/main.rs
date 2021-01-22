fn main() {
    let mut n = 1;

    while n <= 50 {

        if 0 == n % 5 {
            println!("n is {}", n);
        }

        n += 1;
    }
}
