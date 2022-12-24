use std::fs::read_to_string;

pub fn solve() {
    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;
    let result = internal_solve("src/days/day10/input.txt", WIDTH, HEIGHT);
    println!("Result: {}", result);
}

fn internal_solve(path: &str, width: usize, height: usize) -> String {
    let content = read_to_string(path).expect("Fail to read file.");
    let mut grid = "".to_string();
    let mut cycle = 1;
    let mut register = 1;

    for line in content.lines() {
        draw_sprite(cycle, register, &mut grid, width, height);
        if let Some(x) = parse_addx(line) {
            cycle += 1;
            draw_sprite(cycle, register, &mut grid, width, height);
            cycle += 1;
            register += x;
        } else {
            cycle += 1;
        }
    }

    grid
}

fn draw_sprite(cycle: i32, register: i32, grid: &mut String, width: usize, height: usize) {
    let crt_index = cycle - 1;
    let (c_row, c_col) = get_row_col(crt_index as usize, width, height);

    let mut adjusted_register = if register < 0 {
        0 as usize
    } else {
        register as usize
    };
    adjusted_register += c_row * width;

    let mut any_visible = false;
    let lower_bound: usize = if adjusted_register > 0 {
        adjusted_register - 1
    } else {
        adjusted_register
    };
    let upper_bound: usize = adjusted_register + 1;
    for r in lower_bound..=upper_bound {
        let (r_row, r_col) = get_row_col(r, width, height);
        if c_row == r_row && c_col == r_col {
            any_visible = true;
            break;
        }
    }

    grid.push_str(if any_visible { "#" } else { "." });
    if cycle as usize % width == 0 {
        grid.push_str("\n");
    }
}

fn get_row_col(idx: usize, width: usize, _height: usize) -> (usize, usize) {
    (idx / width, idx % width)
}

fn parse_addx(line: &str) -> Option<i32> {
    line.split_once(" ").map(|x| x.1.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_first_row() {
        const WIDTH: usize = 40;
        const HEIGHT: usize = 6;

        const PATH: &str = "src/days/day10/test-input.txt";
        const EXPECTED: &str = "##..##..##..##..##..##..##..##..##..##..";
        let r = internal_solve(PATH, WIDTH, HEIGHT);
        let result = &r[..WIDTH];
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn sample() {
        const WIDTH: usize = 40;
        const HEIGHT: usize = 6;
        const PATH: &str = "src/days/day10/test-input.txt";
        const EXPECTED: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
        let result = internal_solve(PATH, WIDTH, HEIGHT);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn result() {
        const WIDTH: usize = 40;
        const HEIGHT: usize = 6;
        const PATH: &str = "src/days/day10/input.txt";
        const EXPECTED: &str = "###..#..#..##...##...##..###..#..#.####.
##.#.#..#.#..#.#..#.#..#.#..#.#..#....#.
###..#..#.#....#..#.#....###..#..#...#..
#..#.#..#.#....####.#....#..#.#..#..#...
##.#.#..#.#..#.#..#.#..#.#..#.#..#.#....
###...##...##..#..#..##..###...##..####.
";
        let result = internal_solve(PATH, WIDTH, HEIGHT);
        assert_eq!(result, EXPECTED);
    }
}
