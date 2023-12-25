use std::io;

fn main() {
    let mut value = String::new().to_string();
    println!("Enter the first value.");
    io::stdin().read_line(&mut value).expect("error");
    let value_1: u32 = value.trim().parse::<u32>().unwrap();

    value = String::new().to_string();

    println!("Enter the second value.");
    io::stdin().read_line(&mut value).expect("error");
    let value_2: u32 = value.trim().parse::<u32>().unwrap();

    let sum: u32 = value_1 + value_2;

    println!("The sum is = {} + {} = {}", value_1, value_2, sum);
}
