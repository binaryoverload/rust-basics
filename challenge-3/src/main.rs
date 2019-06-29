use rprompt::prompt_reply_stdout;

fn main() {
    let width_string = prompt_reply_stdout("Enter a width for the rectangle: ").unwrap();
    let height_string = prompt_reply_stdout("Enter a height for the rectangle: ").unwrap();

    let width: u32 = width_string.trim().parse().unwrap();
    let height: u32 = height_string.trim().parse().unwrap();

    println!("The area of a rectangle with width {} and height {} is {}!", width, height, width * height);

    print!("\n\n");

    let width_string = prompt_reply_stdout("Enter a width for the cuboid: ").unwrap();
    let height_string = prompt_reply_stdout("Enter a height for the cuboid: ").unwrap();
    let depth_string = prompt_reply_stdout("Enter a depth for the cuboid: ").unwrap();

    let width: u32 = width_string.trim().parse().unwrap();
    let height: u32 = height_string.trim().parse().unwrap();
    let depth: u32 = depth_string.trim().parse().unwrap();

    println!("The volume of a cuboid with width {}, height {} and depth {} is {}!", width, height, depth, width * height * depth);

    let surface_area = (2 * width * height) + (2 * height * depth) + (2 * depth * width);

    println!("The surface area of a cuboid with width {}, height {} and depth {} is {}!", width, height, depth, surface_area);
}
