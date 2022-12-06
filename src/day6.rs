use crate::reader::read_lines;
use colored::Colorize;

pub fn day6() {
    let day = 6;
    println!("{}{}{} ", "\nDay ".bold().bright_green(), day.to_string().bold().bright_red(), ":".bold().bright_green());
    if let Ok(lines) = read_lines(format!("./data/day{}.txt", day)) {
        for line in lines {
            if let Ok(ip) = line {
                let mut count: u32 = 0;
                let mut last_chars: [char; 4] = ['0'; 4];
                for c in ip.chars() {
                    count = count + 1;
                    array_shift(c, &mut last_chars);
                    if !last_chars[0..4].contains(&'0') && !has_duplicates(&last_chars[0..4]) {
                        break;
                    }
                }
                println!("{}", count.to_string().bright_white());

                let mut count: u32 = 0;
                let mut last_chars: [char; 14] = ['0'; 14];
                for c in ip.chars() {
                    count = count + 1;
                    array_shift(c, &mut last_chars);
                    if !last_chars[0..14].contains(&'0') && !has_duplicates(&last_chars[0..14]) {
                        break;
                    }
                }
                println!("{}", count.to_string().bright_yellow());
            }
        }
    }
}

fn array_shift(c: char,  chars: &mut[char]) {
    for i in 0..chars.len() - 1 {
        chars[chars.len() - (i + 1)] = chars[chars.len() - (i + 2)];
    }
    chars[0] = c;
}

fn has_duplicates(array: &[char]) -> bool {
    for i in 0..array.len() - 1 {
        let check_char = &array[i];
        for c in &array[(i+1)..(array.len())] {
            if check_char == c {
                return true;
            }
         }
    }
    return false;
}