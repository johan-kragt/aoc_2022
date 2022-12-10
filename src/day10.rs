use crate::reader::read_lines;
use colored::Colorize;

pub fn day10() {
    let day = 10;
    println!(
        "{}{}{} ",
        "\nDay ".bold().bright_green(),
        day.to_string().bold().bright_red(),
        ":".bold().bright_green()
    );
    if let Ok(lines) = read_lines(format!("./data/day{}.txt", day)) {
        let mut register: i32 = 1;
        let mut cycle: i32 = 0;
        let mut measuring_point = 20;
        let mut signal_strength = 0;

        let mut draw_cycle = 0;
        let mut screen = String::new();
        for line in lines {
            if let Ok(ip) = line {
                let mut add = 0;
                match &ip[0..4] {
                    "noop" => cycle = cycle + 1,
                    "addx" => {
                        cycle = cycle + 2;
                        add = ip[5..].parse::<i32>().unwrap()
                    }
                    _ => panic!("Error"),
                }
                if cycle >= measuring_point {
                    signal_strength = signal_strength + (register * measuring_point);
                    measuring_point = measuring_point + 40;
                }

                for _ in draw_cycle..cycle {
                    if (draw_cycle % 40 - register).abs() <= 1 {
                        screen.push('#');
                    } else {
                        screen.push(' ');
                    }
                    draw_cycle = draw_cycle + 1;
                    if draw_cycle % 40 == 0 {
                        screen.push('\n');
                    }
                }

                register = register + add;
            }
        }
        println!("{}", signal_strength.to_string().bright_white());
        println!("{}", screen.to_string().bright_yellow())
    }
}
