use std::convert::TryInto;
use chrono::offset::TimeZone;
use chrono::{Utc, Date};
use promptly::prompt;

fn main() {
    /*
        ABANDONED, rust doesn't really have any good way to parse dates natively
        Maybe I'll come back to this at some point 

    let date_born_input : String = prompt("Enter your date of birth (dd/mm/yyyy): ");
    let date_parse = Date::parse_from_str(&date_born_input, "%d/%m/%Y");
    let date;
    match date_parse {
        Ok(parsed_date) => date = parsed_date,
        Err(e) => panic!("Could not parse date! {}", e)
    }

    let duration = Utc::now().signed_duration_since(date);

    println!("You are {} years, {} months, {} days old", duration.num_days() / 365, duration.num_days() / 30, duration.num_days())
    */

}
