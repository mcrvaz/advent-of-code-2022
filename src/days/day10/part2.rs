use std::fs::read_to_string;

const WIDTH: usize = 40;
const HEIGHT: usize = 6;

pub fn solve() {
    let result = internal_solve("src/days/day9/input.txt");
    println!("Result: {}", result);
}

fn internal_solve(path: &str) -> String {
    let mut grid = [[""; WIDTH]; HEIGHT];
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            grid[i][j] = ".";
        }
    }

    let content = read_to_string(path).expect("Fail to read file.");
    let mut cycle = 1;
    let mut register = 1;
    for line in content.lines() {
        draw_sprite(cycle, register, &mut grid);
        if let Some(x) = parse_addx(line) {
            cycle += 1;
            draw_sprite(cycle, register, &mut grid);
            cycle += 1;
            register += x;
        } else {
            cycle += 1;
        }
    }

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            print!("{}", grid[i][j]);
        }
        println!("");
    }

    grid.concat().concat()
}

fn draw_sprite(cycle: i32, register: i32, grid: &mut [[&str; WIDTH]; HEIGHT]) {
    let (c_row, c_col) = get_row_col(cycle as usize);
    println!("cycle row {} cycle col {}", c_row, c_col);
    // println!(
    //     "register row {} register col {}",
    //     register_row, register_col
    // );

    let mut visible = false;
    for r in register..register + 2 {
        let (r_row, r_col) = get_row_col(r as usize);
        if c_row == r_row && c_col == r_col {
            visible = true;
        }
    }

    if visible {
        println!("draw");
        grid[c_row][c_col] = "#";
    }
}

fn get_row_col(idx: usize) -> (usize, usize) {
    (idx / WIDTH, idx % WIDTH)
}

fn parse_addx(line: &str) -> Option<i32> {
    line.split_once(" ").map(|x| x.1.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        const PATH: &str = "src/days/day10/test-input.txt";
        const EXPECTED: &str = "##..##..##..##..##..##..##..##..##..##..
        ###...###...###...###...###...###...###.
        ####....####....####....####....####....
        #####.....#####.....#####.....#####.....
        ######......######......######......####
        #######.......#######.......#######.....";
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn result() {
        const PATH: &str = "src/days/day10/input.txt";
        const EXPECTED: &str = "";
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }
}
