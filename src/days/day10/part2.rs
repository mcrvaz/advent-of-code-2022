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
        if let Some(x) = parse_addx(line) {
            cycle += 1;
            draw_sprite(register, &mut grid);
            cycle += 1;
            register += x;
        } else {
            cycle += 1;
        }
        draw_sprite(register, &mut grid);
    }

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            print!("{}", grid[i][j]);
        }
        println!("");
    }

    grid.concat().concat()
}

fn draw_sprite(register: i32, grid: &mut [[&str; WIDTH]; HEIGHT]) {
    let row = (register as usize / HEIGHT) % HEIGHT;
    let col = register as usize % WIDTH;
    if row == 6 {
        println!("ayy");
    }
    // println!("row {} col {}", row, col);
    grid[row][col] = "#";
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
