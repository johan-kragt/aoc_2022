use crate::reader::read_lines;
use colored::Colorize;

pub fn day1() {
    let day = 1;
    println!("{}{}{} ", "\nDay ".bold().green(), day.to_string().bold().red(), ":".bold().green());
    if let Ok(lines) = read_lines(format!("./data/day{}.txt", day)) {
        let mut elves: Vec<Elf> = Vec::new();
        let mut elf = Elf::new();
        for line in lines {
            if let Ok(ip) = line {
                match ip.len() {
                    0 => {
                        elves.push(elf);
                        elf = Elf::new();
                    }
                    _ => {
                        elf.add_calories(ip.parse::<u32>().unwrap());
                    }
                }
            }
        }
        let mut total_calories = elves
            .iter()
            .map(|e| e.total_calories())
            .collect::<Vec<u32>>();
        println!("{}", total_calories.iter().max().unwrap().to_string().bright_white());
        total_calories.sort();
        println!("{}", total_calories.iter().rev().take(3).sum::<u32>().to_string().bright_yellow())
    }
}

struct Elf {
    calories: Vec<u32>,
}

impl Elf {
    fn new() -> Elf {
        Elf {
            calories: Vec::new(),
        }
    }

    fn add_calories(&mut self, value: u32) {
        self.calories.push(value);
    }

    fn total_calories(&self) -> u32 {
        self.calories.iter().sum()
    }
}
