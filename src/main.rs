use clap::Parser;
use paste::paste;
use seq_macro::seq;
use std::{
    collections::HashMap,
    fs, io,
    path::{self, Path},
};

mod days;
mod utils;

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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    create: bool,

    day_number: u8,
    part_number: Option<u8>,
}

fn main() {
    let args = Args::parse();
    if args.create {
        if let Err(e) = create(args.day_number) {
            panic!("{e}");
        }
    } else {
        solve(
            args.day_number,
            args.part_number.expect("Part number not provided!"),
        );
    }
}

fn solve(day_number: u8, part_number: u8) {
    let mut solvers: HashMap<u8, HashMap<u8, fn()>> = HashMap::new();
    seq!(N in 1..=14 {
        solvers.insert(N, day_solver!(N));
    });
    solvers[&day_number][&part_number]();
}

fn create(day_number: u8) -> io::Result<()> {
    fn copy_template_files(template_path: &Path, dst_path: &Path) -> io::Result<()> {
        for entry in fs::read_dir(template_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let file_name = entry.file_name();
                let dst_file_path = Path::new(dst_path).join(file_name);
                fs::copy(path, dst_file_path)?;
            }
        }
        Ok(())
    }

    let dst_dir_path_str = format!("src/days/day{day_number}");
    let dst_mod_path_str = format!("src/days/day{day_number}.rs");
    let template_dir_path = Path::new(&"src/days/template");
    let dst_dir_path = Path::new(&dst_dir_path_str);
    let dst_mod_path = Path::new(&dst_mod_path_str);
    fs::create_dir(dst_dir_path)?;
    fs::write(
        dst_mod_path,
        r"pub mod part1;
pub mod part2;",
    )?;
    copy_template_files(template_dir_path, dst_dir_path)?;
    Ok(())
}
