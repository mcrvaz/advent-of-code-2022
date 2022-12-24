use paste::paste;
use seq_macro::seq;
use std::{collections::HashMap, env};

mod days;
mod utils;

fn main() {
    macro_rules! day_part_fn {
        ($day_number:literal, $part_number:literal) => {
            paste! {
                days::[<day$day_number>]::[<part$part_number>]::solve as fn()
            }
        };
    }

    macro_rules! day_solver {
        ($day_number:literal) => {
            HashMap::from_iter([
                (1, day_part_fn!($day_number, 1)),
                (2, day_part_fn!($day_number, 2)),
            ])
        };
    }

    let day_number = env::args()
        .nth(1)
        .map(|x| x.parse().unwrap())
        .expect("Invalid day number.");
    let part_number = env::args()
        .nth(2)
        .map(|x| x.parse().unwrap())
        .expect("Invalid part number.");

    let mut solvers: HashMap<i32, HashMap<i32, fn()>> = HashMap::new();
    seq!(N in 1..=11 {
        solvers.insert(N, day_solver!(N));
    });
    solvers[&day_number][&part_number]();
}
