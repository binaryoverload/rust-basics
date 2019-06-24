use std::io::{stdout, Write, stdin, Read};

fn pause() {
    stdin().read(&mut [0]).unwrap();
}

fn main() {
    print!("Why is Peter Pan always flying?");
    stdout().flush();
    pause();
    println!("He neverlands!");
}
