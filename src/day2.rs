use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day2() {
    println!("\nDay {}:", 2);
    if let Ok(lines) = read_lines("./data/day2.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut matches: Vec<Match> = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                let chars: Vec<char> = ip.chars().collect();
                matches.push(Match {
                    left: chars[0],
                    right: chars[2],
                })
            }
        }
        println!(
            "{}",
            matches.iter().map(|m| m.response_points()).sum::<u32>()
        );
        println!(
            "{}",
            matches.iter().map(|m| m.strategy_points()).sum::<u32>()
        );
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Match {
    left: char,
    right: char,
}

impl Match {
    fn response_points(&self) -> u32 {
        match self.right {
            'X' => {
                1 + match self.left {
                    'A' => 3,
                    'C' => 6,
                    _ => 0,
                }
            }
            'Y' => {
                2 + match self.left {
                    'A' => 6,
                    'B' => 3,
                    _ => 0,
                }
            }
            _ => {
                3 + match self.left {
                    'C' => 3,
                    'B' => 6,
                    _ => 0,
                }
            }
        }
    }

    fn strategy_points(&self) -> u32 {
        match self.right {
            'X' => {
                0 + match self.left {
                    'A' => 3,
                    'B' => 1,
                    _ => 2,
                }
            }
            'Y' => {
                3 + match self.left {
                    'A' => 1,
                    'B' => 2,
                    _ => 3,
                }
            }
            _ => {
                6 + match self.left {
                    'A' => 2,
                    'B' => 3,
                    _ => 1,
                }
            }
        }
    }
}
