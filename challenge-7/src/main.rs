#![feature(duration_float)]

use rprompt::prompt_reply_stdout;
use std::time::SystemTime;

fn main() {
    prompt_reply_stdout("Press enter when you are ready to enter the alphabet as quick as possible!").expect("Could not read from input");
    let start = SystemTime::now();
    let response = prompt_reply_stdout("Enter the alphabet: ").expect("Could not read from input");
    let time_taken = SystemTime::now().duration_since(start).expect("Could not get duration!").as_secs_f64();
    if response.eq_ignore_ascii_case("abcdefghijklmnopqrstuvwxyz") {
        println!("Well done! You typed the alphabet in {} seconds", time_taken);
    } else {
        println!("Oh no! You didn't quite type that right...");
    }
}
