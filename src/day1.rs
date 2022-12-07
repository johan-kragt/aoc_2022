use crate::reader::read_lines;
use colored::Colorize;

pub fn day1() {
    let day = 1;
    println!(
        "{}{}{} ",
        "\nDay ".bold().bright_green(),
        day.to_string().bold().bright_red(),
        ":".bold().bright_green()
    );
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
        elves.sort_by(|e1, e2| e2.total_calories().cmp(&e1.total_calories()));
        println!("{}", sum_calories(&elves[0..1]).to_string().bright_white());
        println!("{}", sum_calories(&elves[0..3]).to_string().bright_yellow())
    }
}

fn sum_calories(elves: &[Elf]) -> u32 {
    elves.iter().map(|e| e.total_calories()).sum::<u32>()
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
