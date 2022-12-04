use std::env;

mod days;
mod utils;

fn main() {
    let day = env::args().nth(1).expect("Day number not provided.");
    let part = env::args().nth(2).expect("Part number not provided.");
    let day_number = day.parse::<i32>().unwrap();
    let part_number = part.parse::<i32>().unwrap();

    match day_number {
        1 => match part_number {
            1 => days::day1::part1::solve(),
            2 => days::day1::part2::solve(),
            _ => panic!("Invalid part number."),
        },
        2 => match part_number {
            1 => days::day2::part1::solve(),
            2 => days::day2::part2::solve(),
            _ => panic!("Invalid part number."),
        },
        3 => match part_number {
            1 => days::day3::part1::solve(),
            2 => days::day3::part2::solve(),
            _ => panic!("Invalid part number."),
        },
        4 => match part_number {
            1 => days::day4::part1::solve(),
            2 => days::day4::part2::solve(),
            _ => panic!("Invalid part number."),
        },
        _ => panic!("Invalid day number."),
    }
}
