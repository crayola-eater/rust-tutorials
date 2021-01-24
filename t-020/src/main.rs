fn main() {
    let numbers = [1, 2, 3, 4, 5];

    for n in numbers.iter() {
        println!("{}", n);
    }

    for (n, index) in numbers.iter().enumerate() {
        println!("Value {} at index {}", n, index);
    }

    for index in 0..numbers.len() {
        println!("Value {} at index {}", numbers[index], index)
    }
}
