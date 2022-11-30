use std::env;

mod days;
mod utils;

fn main() {
    let day = env::args().nth(1).expect("Day number not provided.");
    let day_number = day.parse::<i32>().unwrap();
    match day_number {
        1 => days::day_1::solve(),
        _ => panic!("Invalid day number."),
    }
}
