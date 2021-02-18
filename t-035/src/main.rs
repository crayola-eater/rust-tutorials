extern crate regex;
use regex::Regex;

fn main() {
    check_and_print_matches();
    capture_groups();
}

fn check_and_print_matches() {
    let re = Regex::new(r"\w{5}").unwrap();
    for text in &["abcdef", "abcde", "abcd"] {
        println!("Found match for '{}'? {}", text, re.is_match(text));
    }
}

fn capture_groups() {
    let re = Regex::new(r"(\w{5})").unwrap();

    for text in &["abcef", "abcde", "abcd"] {
        match re.captures(text) {
            Some(caps) => println!(
                "Found capture group match: {}",
                caps.get(0).unwrap().as_str()
            ),
            None => println!("No capture group match found"),
        }

        match re.captures(text) {
            Some(caps) => println!(
                "Found capture group match (alternative syntax): {}",
                &caps[0]
            ),
            None => println!("No capture group match found (alternative syntax)"),
        }
    }
}
