use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Error reading from STDIN");

    println!("{}", String::from("Hello ") + &buffer);
}
