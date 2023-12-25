use std::io;
use csv;

// We will be converting String to i64 a lot.
fn to_int(some_string: String) -> i64 {some_string.trim().parse::<i64>().unwrap()}

// Choose what to insert.
fn main() {
    loop{
        let mut choice: String = String::new().to_string();
        println!("\nWhat do you want to insert into the graph? (enter a number)\n
            1. Add a point on the graph.
            2. Add a formula to the graph.");// A GUI needs to be added for the graph.
    
        io::stdin().read_line(&mut choice).expect("error reading line");
        let choice_num = to_int(choice);

        if choice_num == 1 {add_point()}
        else {println!("invalid option")}
    }
}

// function to print coordinates of a point
fn add_point() {
    let mut value: String = String::new().to_string();

    println!("Enter the X value.");
    io::stdin().read_line(&mut value).expect("error");
    let value_x = to_int(value);

    value = String::new().to_string();

    println!("Enter the Y value.");
    io::stdin().read_line(&mut value).expect("error");
    let value_y = to_int(value);

    println!("The point is at the coordinates ({}; {})", value_x, value_y);
}
