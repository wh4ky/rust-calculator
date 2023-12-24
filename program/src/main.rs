use std::io;
use std::{thread, time};

fn main() {
    clock8bit();
}

fn clock8bit() {
    let wait = time::Duration::from_millis(100);
    let mut one:u8 = 0; let mut two:u8 = 0; let mut three:u8 = 0; let mut four:u8 = 0;
    let mut five:u8 = 0; let mut six:u8 = 0; let mut seven:u8 = 0; let mut eight:u8 = 0;

    println!("{}{}{}{}{}{}{}{}", one, two, three, four, five, six, seven, eight);
    loop {
        one += 1; num_1 += 1; thread::sleep(wait);

        if one == 2 {two += 1; one == 0};
        if two == 2 {three += 1; two = 0};
        if three == 2 {four += 1; three == 0};
        if four == 2 {five += 1; four == 0};
        if five == 2 {six += 1; five == 0};
        if six == 2 {seven += 1; six == 0};
        if seven == 2 {eight += 1; seven == 0};
        if eight == 2 {break};
    }
}