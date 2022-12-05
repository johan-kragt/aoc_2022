use std::ops::Range;
use std::str::FromStr;
use crate::reader::read_lines;
use colored::Colorize;

pub fn day5() {
    let day = 5;
    println!("{}{}{} ", "\nDay ".bold().bright_green(), day.to_string().bold().bright_red(), ":".bold().bright_green());
    if let Ok(lines) = read_lines(format!("./data/day{}.txt", day)) {
        let mut towers: [Vec<char>; 9] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        let mut linesvec: Vec<String> = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                linesvec.push(ip);
            }
        }
        for i in 0..8 {
            let line = &linesvec[7 - i];
            for j in 0..9 {
                let x = line.chars().collect::<Vec<char>>()[(1 + j * 4)];
                if x != ' ' {
                    towers[j].push(x);
                }
            }
        }
        let mut towers9000 = towers.clone();
        for i in 10..512 {
            // move 4 from 5 to 7
            let comp: Vec<&str> = linesvec[i].split(' ').collect();
            let iterations: u32 = u32::from_str(comp[1]).unwrap();
            let src_tower: usize = usize::from_str(comp[3]).unwrap() - 1;
            let dst_tower: usize = usize::from_str(comp[5]).unwrap() - 1;

            for _ in 0..iterations {
                let x = towers9000[src_tower].pop().unwrap();
                towers9000[dst_tower].push(x);
            }
        }
        let string = towers9000.iter().map(|v| v.last().unwrap()).collect::<String>();
        println!("{}", string.bright_white());

        let mut towers9001 = towers.clone();
        for i in 10..512 {
            // move 4 from 5 to 7
            let comp: Vec<&str> = linesvec[i].split(' ').collect();
            let iterations: u32 = u32::from_str(comp[1]).unwrap();
            let src_tower: usize = usize::from_str(comp[3]).unwrap() - 1;
            let dst_tower: usize = usize::from_str(comp[5]).unwrap() - 1;

            let mut crane: Vec<char> = Vec::new();
            for _ in 0..iterations {
                let x = towers9001[src_tower].pop().unwrap();
                crane.push(x);
            }
            for _ in 0..iterations {
                let x = crane.pop().unwrap();
                towers9001[dst_tower].push(x);
            }
        }
        let string = towers9001.iter().map(|v| v.last().unwrap()).collect::<String>();
        println!("{}", string.bright_yellow());
    }
}