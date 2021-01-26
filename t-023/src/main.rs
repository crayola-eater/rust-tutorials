struct Person {
    name: String,
    age: u8,
}

impl ToString for Person {
    fn to_string(&self) -> String {
        format!("My name is {} and I am {}.", self.name, self.age)
    }
}

fn main() {
    let margaret = Person { name: String::from("Margaret"), age: 21 };
    println!("{}", margaret.to_string());
}
