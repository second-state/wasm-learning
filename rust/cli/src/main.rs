use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", String::from("Hello ") + &args[1]);
}
