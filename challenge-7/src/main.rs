#![feature(duration_float)]

use rprompt::prompt_reply_stdout;
use std::io::{Read, Write, stdout, stdin};
use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();
    let response = prompt_reply_stdout("Enter the alphabet as quick as possible: ").unwrap();
    let time_taken = SystemTime::now().duration_since(start).unwrap().as_secs_f64();
    if response.eq_ignore_ascii_case("abcdefghijklmnopqrstuvwxyz") {
        println!("Well done! You typed the alphabet in {} seconds", time_taken);
    } else {
        println!("Oh no! You didn't quite type that right...");
    }
}
