fn main() {
    let mut my_vector: Vec<i32> = Vec::new();
    my_vector.push(49);
    my_vector.append(&mut vec![50, 51, 52]);
    my_vector.remove(0);

    for (index, number) in my_vector.iter().enumerate() {
        println!("[#{}] {}", index, number);
    }
    
    let another_vector = vec![1, 2, 3, 4, 5];
    for (index, number) in another_vector.iter().enumerate() {
        println!("[#{}] {}", index, number);
    }
}
