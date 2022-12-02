use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day1() {
    println!("\nDay {}:", 1);
    if let Ok(lines) = read_lines("./data/day1.txt") {
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
        println!("{}", total_calories.iter().max().unwrap());
        total_calories.sort();
        println!("{}", total_calories.iter().rev().take(3).sum::<u32>())
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
