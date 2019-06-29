use rprompt::prompt_reply_stdout;

fn main() {

    let speed_string = prompt_reply_stdout("Enter the speed (km/h): ").unwrap();
    let time_string = prompt_reply_stdout("Enter the time travelled at the speed (minutes): ").unwrap();

    let speed: f32 = speed_string.trim().parse().unwrap();
    let time: f32 = time_string.trim().parse().unwrap();

    println!("Distance travelled in the time {}km", speed * (time / 60.0));

    print!("\n\n");

    let distance_string = prompt_reply_stdout("Enter the distance you need to travel (km): ").unwrap();
    let time_string = prompt_reply_stdout("Enter the time you need to travel in (minutes): ").unwrap();

    let distance: f32 = distance_string.trim().parse().unwrap();
    let time: f32 = time_string.trim().parse().unwrap();

    println!("The average speed needed to travel {}km in {} hours is {}km/h", distance, time / 60.0, distance / (time / 60.0));
}
