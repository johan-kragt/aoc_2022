use crate::reader::read_lines;
use colored::Colorize;

pub fn day2() {
    let day = 2;
    println!("{}{}{} ", "\nDay ".bold().green(), day.to_string().bold().red(), ":".bold().green());
    if let Ok(lines) = read_lines(format!("./data/day{}.txt", day)) {
        // Consumes the iterator, returns an (Optional) String
        let mut matches: Vec<Match> = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                let chars: Vec<char> = ip.chars().collect();
                matches.push(Match {
                    left: Item::from_char(chars[0]),
                    right: chars[2],
                })
            }
        }
        println!(
            "{}",
            matches.iter().map(|m| m.response_points()).sum::<u32>().to_string().bright_white()
        );
        println!(
            "{}",
            matches.iter().map(|m| m.strategy_points()).sum::<u32>().to_string().bright_yellow()
        );
    }
}

enum Item {
    ROCK,
    PAPER,
    SCISSORS,
}

impl Item {
    fn from_char(c: char) -> Item {
        match c {
            'A' => Item::ROCK,
            'X' => Item::ROCK,
            'B' => Item::PAPER,
            'Y' => Item::PAPER,
            'C' => Item::SCISSORS,
            'Z' => Item::SCISSORS,
            _ => panic!("Could not map to gameItem"),
        }
    }

    fn points_against(&self, counter: &Item) -> u32 {
        match self {
            Item::ROCK => {
                1 + match counter {
                    Item::ROCK => 3,
                    Item::PAPER => 0,
                    Item::SCISSORS => 6,
                }
            }
            Item::PAPER => {
                2 + match counter {
                    Item::ROCK => 6,
                    Item::PAPER => 3,
                    Item::SCISSORS => 0,
                }
            }
            Item::SCISSORS => {
                3 + match counter {
                    Item::ROCK => 0,
                    Item::PAPER => 6,
                    Item::SCISSORS => 3,
                }
            }
        }
    }

    fn winning_item(&self) -> Item {
        match self {
            Item::ROCK => Item::PAPER,
            Item::PAPER => Item::SCISSORS,
            Item::SCISSORS => Item::ROCK,
        }
    }

    fn losing_item(&self) -> Item {
        match self {
            Item::ROCK => Item::SCISSORS,
            Item::PAPER => Item::ROCK,
            Item::SCISSORS => Item::PAPER,
        }
    }

    fn draw_item(&self) -> Item {
        match self {
            Item::ROCK => Item::ROCK,
            Item::PAPER => Item::PAPER,
            Item::SCISSORS => Item::SCISSORS,
        }
    }
}

struct Match {
    left: Item,
    right: char,
}

impl Match {
    fn response_points(&self) -> u32 {
        Item::from_char(self.right).points_against(&self.left)
    }

    fn strategy_points(&self) -> u32 {
        match self.right {
            'X' => self.left.losing_item().points_against(&self.left),
            'Y' => self.left.draw_item().points_against(&self.left),
            _ => self.left.winning_item().points_against(&self.left),
        }
    }
}
