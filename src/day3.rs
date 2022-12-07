use crate::reader::read_lines;
use colored::Colorize;

pub fn day3() {
    let day = 3;
    println!(
        "{}{}{} ",
        "\nDay ".bold().bright_green(),
        day.to_string().bold().bright_red(),
        ":".bold().bright_green()
    );
    if let Ok(lines) = read_lines(format!("./data/day{}.txt", day)) {
        let mut rucksacks: Vec<Rucksack> = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                rucksacks.push(Rucksack::from_string(ip.as_str()));
            }
        }
        let sum: u32 = rucksacks.iter().map(|r| r.double_item_priority()).sum();
        println!("{}", sum.to_string().bright_white());
        let mut total: u32 = 0;
        for i in 0..rucksacks.len() / 3 {
            let group = &rucksacks[(i * 3)..(i * 3) + 3];
            for c in group[0].all_items.chars() {
                if group[1].contains(&c) && group[2].contains(&c) {
                    total += get_item_priority(c);
                    break;
                }
            }
        }
        println!("{}", total.to_string().bright_yellow());
    }
}

fn get_item_priority(c: char) -> u32 {
    match c.is_ascii_uppercase() {
        true => (c as u32) - 38,
        false => (c as u32) - 96,
    }
}

struct Rucksack {
    all_items: String,
    compartement1: Vec<char>,
    compartement2: Vec<char>,
}

impl Rucksack {
    fn from_string(input: &str) -> Rucksack {
        Rucksack {
            all_items: String::from(input),
            compartement1: input[0..(input.len() / 2)].chars().collect::<Vec<char>>(),
            compartement2: input[(input.len() / 2)..].chars().collect::<Vec<char>>(),
        }
    }

    fn double_item(&self) -> char {
        for c in &self.compartement1 {
            if self.compartement2.contains(c) {
                return c.clone();
            }
        }
        '0'
    }

    fn double_item_priority(&self) -> u32 {
        let c = self.double_item();
        get_item_priority(c)
    }

    fn contains(&self, c: &char) -> bool {
        self.compartement1.contains(c) || self.compartement2.contains(c)
    }
}
