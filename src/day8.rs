use crate::reader::read_lines;
use colored::Colorize;

pub fn day8() {
    let day = 8;
    println!(
        "{}{}{} ",
        "\nDay ".bold().bright_green(),
        day.to_string().bold().bright_red(),
        ":".bold().bright_green()
    );
    const GRIDSIZE: usize = 99;
    if let Ok(lines) = read_lines(format!("./data/day{}.txt", day)) {
        let mut trees = [[0u32; GRIDSIZE]; GRIDSIZE];
        let mut y = 0;
        let mut visible_trees = 0;
        let mut max_scenic_score = 0;
        for line in lines {
            if let Ok(ip) = line {
                for x in 0..ip.len() {
                    trees[y][x] = ip[x..x + 1].parse::<u32>().unwrap()
                }
            }
            y = y + 1;
        }
        for y in 0..GRIDSIZE {
            for x in 0..GRIDSIZE {
                if is_visible(x, y, trees) {
                    visible_trees = visible_trees + 1;
                }
            }
        }
        println!("{}", visible_trees.to_string().bright_white());

        for y in 0..GRIDSIZE {
            for x in 0..GRIDSIZE {
                let scenic_score = calculate_scenic_score(x, y, trees);
                if scenic_score > max_scenic_score {
                    max_scenic_score = scenic_score;
                }
            }
        }
        println!("{}", max_scenic_score.to_string().bright_yellow());
    }

    fn is_visible(x: usize, y: usize, trees: [[u32; GRIDSIZE]; GRIDSIZE]) -> bool {
        is_visible_north(x, y, trees)
            || is_visible_east(x, y, trees)
            || is_visible_south(x, y, trees)
            || is_visible_west(x, y, trees)
    }

    fn is_visible_north(x: usize, y: usize, trees: [[u32; GRIDSIZE]; GRIDSIZE]) -> bool {
        let height = trees[y][x];
        for y in (0..y).rev() {
            if trees[y][x] >= height {
                return false;
            }
        }
        true
    }

    fn is_visible_east(x: usize, y: usize, trees: [[u32; GRIDSIZE]; GRIDSIZE]) -> bool {
        let height = trees[y][x];
        for x in x + 1..trees[y].len() {
            if trees[y][x] >= height {
                return false;
            }
        }
        true
    }

    fn is_visible_south(x: usize, y: usize, trees: [[u32; GRIDSIZE]; GRIDSIZE]) -> bool {
        let height = trees[y][x];
        for y in y + 1..trees[y].len() {
            if trees[y][x] >= height {
                return false;
            }
        }
        true
    }

    fn is_visible_west(x: usize, y: usize, trees: [[u32; GRIDSIZE]; GRIDSIZE]) -> bool {
        let height = trees[y][x];
        for x in (0..x).rev() {
            if trees[y][x] >= height {
                return false;
            }
        }
        true
    }

    fn calculate_scenic_score(x: usize, y: usize, trees: [[u32; GRIDSIZE]; GRIDSIZE]) -> u32 {
        calculate_scenic_score_north(x, y, trees)
            * calculate_scenic_score_east(x, y, trees)
            * calculate_scenic_score_south(x, y, trees)
            * calculate_scenic_score_west(x, y, trees)
    }

    fn calculate_scenic_score_north(x: usize, y: usize, trees: [[u32; GRIDSIZE]; GRIDSIZE]) -> u32 {
        let height = trees[y][x];
        let mut viewing_distance = 0;
        for y in (0..y).rev() {
            viewing_distance = viewing_distance + 1;
            if trees[y][x] >= height {
                break;
            }
        }
        viewing_distance
    }

    fn calculate_scenic_score_east(x: usize, y: usize, trees: [[u32; GRIDSIZE]; GRIDSIZE]) -> u32 {
        let height = trees[y][x];
        let mut viewing_distance = 0;
        for x in x + 1..trees[y].len() {
            viewing_distance = viewing_distance + 1;
            if trees[y][x] >= height {
                break;
            }
        }
        viewing_distance
    }

    fn calculate_scenic_score_south(x: usize, y: usize, trees: [[u32; GRIDSIZE]; GRIDSIZE]) -> u32 {
        let height = trees[y][x];
        let mut viewing_distance = 0;
        for y in y + 1..trees[y].len() {
            viewing_distance = viewing_distance + 1;
            if trees[y][x] >= height {
                break;
            }
        }
        viewing_distance
    }

    fn calculate_scenic_score_west(x: usize, y: usize, trees: [[u32; GRIDSIZE]; GRIDSIZE]) -> u32 {
        let height = trees[y][x];
        let mut viewing_distance = 0;
        for x in (0..x).rev() {
            viewing_distance = viewing_distance + 1;
            if trees[y][x] >= height {
                break;
            }
        }
        viewing_distance
    }
}
