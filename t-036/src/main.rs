mod printers {
    pub fn print_message() {
        private_func();
        println!("How's it going?");
    }

    fn private_func() {
        println!("You called a private function?");
    }

    pub mod nested_mode {
        pub fn say_it() {
            println!("I'm water");
        }
    }
}

fn main() {
    printers::print_message();
    printers::nested_mode::say_it();
}
