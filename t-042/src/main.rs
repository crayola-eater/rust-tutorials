extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    is_male: bool,
}

fn main() {
    let json_str = r#"
        {
            "name": "Domenic",
            "age": 65,
            "is_male": true
        }
    "#;

    let res = serde_json::from_str(json_str);

    if res.is_ok() {
        let parsed: JsonValue = res.unwrap();
        println!("The name is {}", parsed["name"]);
    } else {
        println!("Sorry, could not parse JSON!");
    }

    let person = Person {
        name: String::from("Abe"),
        age: 75,
        is_male: true,
    };

    let serialised = serde_json::to_string(&person).unwrap();
    println!("serialised = {}", serialised);

    let deserialised: Person = serde_json::from_str(&serialised).unwrap();
    println!("deserialised = {:?}", deserialised);
}
