use std::io::{stdout, Write, stdin};

fn main() {
    print!("Please enter your name: ");
    stdout().flush().unwrap();
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    println!("Hello {}!", name.replace("\n", ""));
}
