use std::io::{stdin, stdout, Read, Write};
use std::time::SystemTime;

fn main() {
    print!("Press enter when you think 10 seconds has elapsed!");
    stdout().flush().unwrap();
    let start = SystemTime::now();
    stdin().read(&mut [0]).unwrap();
    let time_taken = (SystemTime::now().duration_since(start).unwrap().as_millis()) / 1000;
    print!("You pressed enter after {} seconds! {}", time_taken, if time_taken as u32 == 10 {"You got the achievement!"} else {"You didn't quite get it..."});
}
