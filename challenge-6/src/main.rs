use rprompt::prompt_reply_stdout;
use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();
    prompt_reply_stdout("Press enter when you think 10 seconds has elapsed!").expect("Could not read from input!");
    let time_taken = (SystemTime::now().duration_since(start).unwrap().as_millis()) / 1000;
    print!("You pressed enter after {} seconds! {}", time_taken, if time_taken as u32 == 10 {"You got the achievement!"} else {"You didn't quite get it..."});
}
