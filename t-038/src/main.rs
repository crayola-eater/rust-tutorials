extern crate reqwest;

fn main() {
    // Long way
    match reqwest::blocking::get("https://httpbin.org/ip") {
        Ok(response) => {
            if reqwest::StatusCode::OK != response.status() {
                println!("Response was not 200 OK");
            }
            match response.text() {
                Ok(text) => println!("Response text: {}", text),
                Err(_) => println!("Could not read response text"),
            }
        }
        Err(_) => println!("Could not make the request!"),
    }

    // Short way
    let response = reqwest::blocking::get("https://httpbin.org/ip")
        .expect("Couldn't make request")
        .text()
        .expect("Couldn't read response text");

    println!("Response text: {}", response);
}
