fn main() {
    let numbers = 30..51;
    let animals = vec!["Rabbit", "Dog", "Cat"];

    for i in numbers {
        println!("The number is {}", i);
    }

    // Use .iter() to prevent the ownership moving 
    // to the the for loop.
    for animal in animals.iter() {
        println!("The animal is {}", animal)
    }

    for (i, animal) in animals.iter().enumerate() {
        println!("The animal at index {} is {}", i, animal)
    }
}
