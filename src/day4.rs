use crate::reader::read_lines;
use colored::Colorize;
use std::ops::Range;

pub fn day4() {
    let day = 4;
    println!(
        "{}{}{} ",
        "\nDay ".bold().bright_green(),
        day.to_string().bold().bright_red(),
        ":".bold().bright_green()
    );
    if let Ok(lines) = read_lines(format!("./data/day{}.txt", day)) {
        let mut assignment_pair: Vec<AssignmentPair> = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                assignment_pair.push(AssignmentPair::from_string(ip.as_str()));
            }
        }
        let count_fully_contains = assignment_pair.iter().filter(|p| p.full_overlap()).count();
        println!("{}", count_fully_contains.to_string().bright_white());

        let count_partially_overlapping = assignment_pair
            .iter()
            .filter(|p| p.partial_overlap())
            .count();
        println!(
            "{}",
            count_partially_overlapping.to_string().bright_yellow()
        );
    }
}

struct AssignmentPair {
    left: Range<u32>,
    right: Range<u32>,
}

impl AssignmentPair {
    fn from_string(input: &str) -> AssignmentPair {
        let split: Vec<&str> = input.split(',').collect();
        let l: Vec<&str> = split[0].split('-').collect();
        let r: Vec<&str> = split[1].split('-').collect();
        AssignmentPair {
            left: (l[0].parse::<u32>().unwrap() as u32)..(l[1].parse::<u32>().unwrap() as u32),
            right: (r[0].parse::<u32>().unwrap() as u32)..(r[1].parse::<u32>().unwrap() as u32),
        }
    }

    fn full_overlap(&self) -> bool {
        AssignmentPair::fully_contains(&self.left, &self.right)
            || AssignmentPair::fully_contains(&self.right, &self.left)
    }

    fn partial_overlap(&self) -> bool {
        self.left.start.max(self.right.start) <= self.left.end.min(self.right.end)
    }

    fn fully_contains(left: &Range<u32>, right: &Range<u32>) -> bool {
        left.start <= right.start && left.end >= right.end
    }
}
